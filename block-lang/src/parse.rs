#![allow(dead_code)]

use std::cell::RefCell;

use ariadne::{Color, Label, Report, Fmt, ReportKind, Source};
use chumsky::{prelude::*, text::keyword};
use hashdb::{LinkArena, TypeStore};

use crate::{expr::{BindSubTree, Expr}, name::{NamedObject, NamespaceMut}};

// Represents active bound variables in the course of parsing an expression
#[derive(Default, Debug)]
pub struct BindMap {
	map: RefCell<Vec<String>>,
}
impl BindMap {
	// Get binding index for this variable
	fn bind_index(&self, string: &String) -> Option<usize> {
		self.map.borrow().iter().enumerate().rev().find(|(_, e)|*e == string).map(|val|val.0 + 1)
	}
	fn push_bind(&self, string: &String) -> usize {
		let mut map = self.map.borrow_mut();
		map.push(string.clone());
		map.len()
	}
	fn pop_bind(&self) -> usize {
		let mut map = self.map.borrow_mut();
		let ret = map.len();
		map.pop();
		ret
	}
}

fn lookup_expr<'e, E: TypeStore<'e>>(namespace: &NamespaceMut<'e>, string: &str, _exprs: &'e E) -> Option<&'e Expr<'e>> {
	let name = namespace.find(|name|name.string == string)?;
	match name.object {
		NamedObject::Namespace(_) => None,
		NamedObject::Expr(expr) => Some(expr)
	}
}
fn name_parser() -> impl Parser<char, String, Error = Simple<char>> + Clone {
	text::ident().padded().labelled("name")
}

fn parser<'e: 'b, 'b, B: TypeStore<'b>, E: TypeStore<'e>>(namespace: &'b NamespaceMut<'e>, exprs: &'e E, binds: &'b B, bind_map: &'b BindMap) -> impl Parser<char, (&'e Expr<'e>, &'b BindSubTree<'b>), Error = Simple<char>> + Clone {
	recursive(|expr: Recursive<'b, char, (&'e Expr<'e>, &'b BindSubTree<'b>), Simple<char>>| {
		// A symbol, can be pretty much any string not including whitespace
		let number = text::int::<_, Simple<char>>(10).padded()
			.try_map(|s, span|
				s.parse::<usize>()
				.map_err(|e| Simple::custom(span, format!("{}", e)))
			).try_map(|num, span| {
				match (lookup_expr(namespace, "zero", exprs), lookup_expr(namespace, "succ", exprs)) {
					(Some(zero), Some(succ)) => {
						let expr = (0..num).into_iter().fold(zero, |acc, _|Expr::app(succ, acc, exprs));
						Ok((expr, BindSubTree::NONE))
					}
					_ => Err(Simple::custom(span, "names `zero` and `succ` must be defined to use numbers"))
				}
			}).labelled("number");

		// A resolved symbol, variable, or paranthesised expression.
		let atom = name_parser().map(|string| {
			if let Some(val) = bind_map.bind_index(&string) {
				(Expr::VAR, BindSubTree::end(val, binds))
			} else if let Some(expr) = lookup_expr(namespace, &string, exprs) {
				(expr, BindSubTree::NONE)
			} else { (Expr::VAR, BindSubTree::NONE) }
		}).labelled("expression")
    	.or(number)
		.or(expr.clone().delimited_by(just('('), just(')')).padded());

		// Parse `[x y z] x y z` as `[x] ([y] ([z] x y z))`
		let lambda = name_parser()
    		.repeated().at_least(1)
			.delimited_by(just('['), just(']'))
			.map(|symbols| {
				symbols.iter().for_each(|string|{
					bind_map.push_bind(string);
				});
				0..symbols.len()
			}).then(expr.clone()).foldr(|_, (lam_expr, mut bind_tree)| {
				let binding = bind_tree.pop_binding(binds, &bind_map.pop_bind(), exprs).expect("failed to pop lambda");
				(Expr::lambda(binding, lam_expr, exprs), bind_tree)
			}).labelled("lambda");
		
		// Parse `x y z` as `((x y) z)`
		let application = atom.clone()
			.then(atom.clone().repeated().at_least(1))
			.foldl(|(func, func_index), (args, args_index)| {
				(Expr::app(func, args, exprs), BindSubTree::branch(func_index, args_index, binds))
			}).labelled("application");


		// An expression can be a lambda: `[x y]` an application: `x y` or a standalone variable / symbol: `x`
		lambda.or(application).or(atom).padded().labelled("expression")
	}).then_ignore(end())
}
// Parse expression
pub fn parse<'e>(string: &str, namespace: &NamespaceMut<'e>, exprs: &'e LinkArena<'e>) -> Result<&'e Expr<'e>, anyhow::Error> {
	let binds = &LinkArena::new();
	let bind_map = &BindMap::default();
	{
		let parsed = parser(namespace, exprs, binds, bind_map).parse(string);
		match parsed {
			Ok((expr, _)) => Ok(expr),
			Err(errors) => {
				gen_report(errors).try_for_each(|report|report.print(Source::from(&string)))?;
				Err(anyhow::anyhow!("Error"))
			}
		}
	}
}

/// Generate cool errors with ariadne
pub fn gen_report(errors: Vec<Simple<char>>) -> impl Iterator<Item = Report> {
	// Taken from json.rs example on chumsky github
	errors.into_iter().map(|e| {
        let msg = if let chumsky::error::SimpleReason::Custom(msg) = e.reason() {
            msg.clone()
        } else {
            format!(
                "{}{}, expected {}",
                if e.found().is_some() {
                    "Unexpected token"
                } else {
                    "Unexpected end of input"
                },
                if let Some(label) = e.label() {
                    format!(" while parsing {}", label)
                } else {
                    String::new()
                },
                if e.expected().len() == 0 {
                    "something else".to_string()
                } else {
                    e.expected()
                        .map(|expected| match expected {
                            Some(expected) => expected.to_string(),
                            None => "end of input".to_string(),
                        })
                        .collect::<Vec<_>>()
                        .join(", ")
                },
            )
        };

        let report = Report::build(ReportKind::Error, (), e.span().start)
            .with_code(3)
            .with_message(msg)
            .with_label(
                Label::new(e.span())
                    .with_message(match e.reason() {
                        chumsky::error::SimpleReason::Custom(msg) => msg.clone(),
                        _ => format!(
                            "Unexpected {}",
                            e.found()
                                .map(|c| format!("token {}", c.fg(Color::Red)))
                                .unwrap_or_else(|| "end of input".to_string())
                        ),
                    })
                    .with_color(Color::Red),
            );

        let report = match e.reason() {
            chumsky::error::SimpleReason::Unclosed { span, delimiter } => report.with_label(
                Label::new(span.clone())
                    .with_message(format!(
                        "Unclosed delimiter {}",
                        delimiter.fg(Color::Yellow)
                    ))
                    .with_color(Color::Yellow),
            ),
            chumsky::error::SimpleReason::Unexpected => report,
            chumsky::error::SimpleReason::Custom(_) => report,
        };

        report.finish()
    })
}

/// Parse and reduce a string
pub fn parse_reduce<'e>(string: &str, namespace: &NamespaceMut<'e>, exprs: &'e LinkArena<'e>) -> Result<&'e Expr<'e>, anyhow::Error> {
	Ok(parse(string, namespace, exprs)?.reduce(exprs)?)
}

/// Commands for cli
#[derive(Debug, Clone)]
pub enum Command<'e> {
	// Do nothing
	None,
	// Set a name in a namespace to a certain value
	Set(String, &'e Expr<'e>),
	// Load a symbol from a file
	Load { /* name: String,  */file: String },
	// Save a symbol to a file, either overwriting or not overwriting the file.
	Save { /* name: String,  */file: String, overwrite: bool },
	/// Import names, if none listed, imports all names
	Use { name: String, items: Vec<String> },
	/// Clear current namespace
	Clear,
	/// List current namespace's names
	List,
	// Evaluate passed expression and store output in 
	Reduce(&'e Expr<'e>),
}
/// Parse commands
pub fn command_parser<'e: 'b, 'b>(namespace: &'b NamespaceMut<'e>, exprs: &'e LinkArena<'e>, binds: &'b LinkArena<'b>, bind_map: &'b BindMap) -> impl Parser<char, Command<'e>, Error = Simple<char>> + 'b {
	let expr = parser(namespace, exprs, binds, bind_map);

	let filepath = just::<_, _, Simple<char>>('"')
		.ignore_then(filter(|c| *c != '\\' && *c != '"').repeated())
		.then_ignore(just('"'))
		.collect::<String>()
    	.padded()
		.labelled("filepath");


	/* #[derive(Clone, Copy)]
	enum Comm { None, Set, List, Clear, Use, Load, Save, Reduce };
	let command = end().to(Comm::None)
    	.or(keyword("set").to(Comm::Set))
    	.or(keyword("list").to(Comm::List))
    	.or(keyword("clear").to(Comm::Clear))
    	.or(keyword("use").to(Comm::Use))
    	.or(keyword("load").to(Comm::Load))
    	.or(keyword("save").to(Comm::Save))
    .or(empty().to(Comm::Reduce))
    	.labelled("command").map(||) */

	end().to(Command::None)
    	.or(
			keyword("set")
				.ignore_then(text::ident().padded())
				.then(expr.clone()).map(|(symbol, (expr, _))| Command::Set(symbol, expr))
		)
		.or(
			keyword("list").to(Command::List)
		)
    	.or(
			keyword("load").ignore_then(filepath).map(|file|Command::Load { file })
		)
		.or(
			keyword("save").ignore_then(filepath).map(|file|Command::Save { file, overwrite: false })
		)
		.or(
			expr.clone().map(|(expr, _)|Command::Reduce(expr))
		)
		.labelled("command")
}

#[test]
fn parse_test() {
	use crate::expr::Binding;
	use hashdb::LinkArena;

	let exprs = &LinkArena::new();
	let namespace = &mut NamespaceMut::new();
	let parsed = parse("[x y] x y", namespace, exprs).unwrap();
	let test = Expr::lambda(Binding::left(Binding::END, exprs),
	Expr::lambda(Binding::right(Binding::END, exprs),
			Expr::app(Expr::VAR, Expr::VAR, exprs),
		exprs),
	exprs);
	assert_eq!(parsed, test);

	assert_eq!(test, parse("[x y] (x y)", namespace, exprs).unwrap());

	let parsed = parse_reduce("([x y] x) ([x y] y) ([x y] x)", namespace, exprs).unwrap();
	let parsed_2 = parse("([x y] y)", namespace, exprs).unwrap();
	assert_eq!(parsed, parsed_2);

	let iszero = parse_reduce("[n] n ([u] [x y] y) ([x y] x)", namespace, exprs).unwrap();
	namespace.add("iszero", iszero, exprs);

	let test = parse_reduce("iszero ([x y] y)", namespace, exprs).unwrap();
	assert_eq!(test, parse("[x y] x", namespace, exprs).unwrap())
}