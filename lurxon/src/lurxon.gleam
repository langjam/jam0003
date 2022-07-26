import gleam/erlang
import gleam/io
import gleam/erlang/file
import lex

pub fn main() {
  let [filename] = erlang.start_arguments()
  assert Ok(src) =
    filename
    |> file.read()

  lex.lex(src)
  |> io.debug()

  Ok(0)
}
