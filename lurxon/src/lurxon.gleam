import gleam/erlang
import gleam/io
import gleam/erlang/file

pub fn main() {
  let [filename] = erlang.start_arguments()
  assert Ok(src) =
    filename
    |> file.read()
  io.debug(src)
  Ok(0)
}
