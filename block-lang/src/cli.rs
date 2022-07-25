use hashdb::LinkArena;
use name::NamespaceMut;
use parse::{command_parser, gen_report, parse};

mod expr;
mod name;
mod parse;

pub fn print_usage() {
	println!("USAGE: tmp-lang <filename>")
}

pub fn read_from_file(filename: &str) -> Result<String, String> {
	std::fs::read_to_string(filename).map_err(|_| "could not open file".into())
}

fn cli_editor() {
	use ariadne::Source;
	use chumsky::Parser;
	use parse::Command;
	use rustyline::Editor;

	println!("block-lang cli editor!");
	let mut editor = Editor::<()>::new().unwrap();
	if editor.load_history(".editor_history").is_err() {}

	let namespace = NamespaceMut::new();
	let exprs = LinkArena::new();
	let binds = LinkArena::new();
	let bind_map = parse::BindMap::default();
	let cmdparser = command_parser(&namespace, &exprs, &binds, &bind_map);
	loop {
		let text = match editor.readline("=> ") {
			Ok(line) => line,
			Err(_) => break,
		};
		match cmdparser.parse(text.as_str()) {
			Ok(Command::None) => {}
			Ok(Command::Set(string, expr)) => {
				println!("{expr}");
				let reduced = expr.reduce(&exprs).unwrap();
				println!("{reduced}");
				namespace.add(string, reduced, &exprs);
			}
			Ok(Command::List) => {
				namespace.for_each(|name| println!("{name}"));
			}
			Ok(Command::Reduce(expr)) => {
				println!("{expr}");
				let reduced = expr.reduce(&exprs).unwrap();
				println!("{reduced}");
			}
			// Ok(Command::Load { file }) => {
			// 	todo!();
			// }
			// Ok(Command::Save { file, overwrite }) => {
			// 	todo!();
			// }
			Ok(_) => {}
			Err(errors) => {
				gen_report(errors)
					.try_for_each(|report| report.print(Source::from(&text)))
					.unwrap();
			}
		}
	}
	editor.save_history(".editor_history").unwrap();
}

pub fn run_cli_args() -> Result<(), String> {
	let mut input_files: Vec<String> = vec![];
	for arg in std::env::args() {
		match arg.as_str() {
			"-i" => {
				cli_editor();
				return Ok(());
			}
			_ => input_files.push(arg),
		};
	}
	if input_files.is_empty() {
		return Err("no input files".into());
	} else if input_files.len() > 1 {
		return Err("multiple input files".into());
	}
	let file_content =
		read_from_file(input_files[0].as_str()).map_err(|_| "could not read file".to_owned())?;
	let exprs = &LinkArena::new();
	let namespace = &mut NamespaceMut::new();
	let _parsed = parse(file_content.as_str(), namespace, exprs).unwrap();
	Ok(())
}

fn main() {
	cli_editor();
}
