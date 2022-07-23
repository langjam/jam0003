import gleam/string
import gleam/option.{None, Option, Some}
import gleam/list

pub type Token {
  LParen
  RParen

  Ident(String)
  Num(Int)

  If
  Else
  Fn
  Let

  Add
  Sub
  Mul
  Div

  EOF
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
        "+" -> make_token(Add, fst, source)
        "-" -> make_token(Sub, fst, source)
        "*" -> make_token(Mul, fst, source)
        "/" -> make_token(Div, fst, source)
        "(" -> make_token(LParen, fst, source)
        ")" -> make_token(RParen, fst, source)
        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ->
          make_number(source)
        "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z" | "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M" | "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z" ->
          make_ident(source)
      })
  }
}

fn make_token(token: Token, value: String, from: String) -> #(Token, String) {
  assert True =
    from
    |> string.starts_with(value)

  let len = string.length(value)
  let rest = string.drop_left(from, len)
  #(token, rest)
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
