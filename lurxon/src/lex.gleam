import gleam/string
import gleam/option.{None, Option, Some}
import gleam/list

pub type Token {

  EOF

  Ident(String)
  Num(Int)

  If
  Else
  Fn
  Let

  // +
  Add
  // -
  Sub
  // ->
  Arrow
  // *
  Mul
  // /
  Div
  // ;
  Semi
  // :
  Colon
  // =
  Assign
  // ==
  Eq
  // !=
  Neq
  // !
  Not
  // <
  Lt
  // <=
  Le
  // > 
  Gt
  // >=
  Ge
  // (
  LParen
  // )
  RParen
  // {
  LBrace
  // }
  RBrace
  // .
  Field
  // ..
  Range
  // ,
  Comma
}

pub type Span {
  // Bytewize indexes
  Span(lo: Int, hi: Int)
}

pub fn lex(source: String) -> List(Token) {
  lex_to(source, [])
  |> list.reverse()
}

fn lex_to(source: String, toks: List(Token)) -> List(Token) {
  case lex_one(source) {
    Some(#(tok, ns)) -> lex_to(ns, [tok, ..toks])
    None -> toks
  }
}

fn lex_one(source: String) -> Option(#(Token, String)) {
  let source = string.trim_left(source)
  case string.first(source) {
    Error(_) -> None
    Ok(fst) ->
      Some(case fst {
        "+" -> make_token1(Add, source)
        "-" -> make_token_if(">", Sub, Arrow, source)
        "*" -> make_token1(Mul, source)
        "/" -> make_token1(Div, source)
        ";" -> make_token1(Semi, source)
        ":" -> make_token1(Colon, source)
        "=" -> make_token_if("=", Assign, Eq, source)
        "!" -> make_token_if("=", Not, Neq, source)
        "<" -> make_token_if("=", Lt, Le, source)
        ">" -> make_token_if("=", Gt, Ge, source)
        "(" -> make_token1(LParen, source)
        ")" -> make_token1(RParen, source)
        "{" -> make_token1(LBrace, source)
        "}" -> make_token1(RBrace, source)
        "." -> make_token_if(".", Field, Range, source)
        "," -> make_token1(Comma, source)
        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ->
          make_number(source)
        "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z" | "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M" | "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z" ->
          make_ident(source)
      })
  }
}

fn make_token1(token: Token, from: String) -> #(Token, String) {
  let rest = string.drop_left(from, 1)
  #(token, rest)
}

fn make_token_if(
  snd_char: String,
  t1: Token,
  t2: Token,
  src: String,
) -> #(Token, String) {
  let s1 =
    src
    |> string.drop_left(1)
  let c2 = string.first(s1)
  let s2 = string.drop_left(s1, 1)
  case Ok(snd_char) == c2 {
    True -> #(t1, s1)
    False -> #(t2, s2)
  }
}

fn make_number(source: String) -> #(Token, String) {
  do_make_number(source, 0)
}

fn make_ident(source: String) -> #(Token, String) {
  do_make_indent(source, "")
}

fn do_make_indent(source: String, id: String) -> #(Token, String) {
  case string.first(source) {
    Ok(s) ->
      case is_ident_char(s) {
        True ->
          do_make_indent(string.drop_left(source, 1), string.append(id, s))
        False -> finish_make_ident(source, id)
      }
    Error(_) -> finish_make_ident(source, id)
  }
}

fn finish_make_ident(rest: String, id: String) -> #(Token, String) {
  #(
    case id {
      "if" -> If
      "else" -> Else
      "fn" -> Fn
      "let" -> Let
      _ -> Ident(id)
    },
    rest,
  )
}

fn do_make_number(source: String, n: Int) -> #(Token, String) {
  case string.first(source) {
    Ok(d) -> {
      let rest = string.drop_left(source, 1)
      case num_val(d) {
        Some(n0) -> do_make_number(rest, n * 10 + n0)
        None -> #(Num(n), source)
      }
    }
    Error(_) -> #(Num(n), source)
  }
}

fn num_val(digit: String) -> Option(Int) {
  case digit {
    "0" -> Some(0)
    "1" -> Some(1)
    "2" -> Some(2)
    "3" -> Some(3)
    "4" -> Some(4)
    "5" -> Some(5)
    "6" -> Some(6)
    "7" -> Some(7)
    "8" -> Some(8)
    "9" -> Some(9)
    _ -> None
  }
}

fn is_ident_char(x: String) -> Bool {
  case x {
    "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z" | "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M" | "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z" | "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ->
      True
    _ -> False
  }
}
