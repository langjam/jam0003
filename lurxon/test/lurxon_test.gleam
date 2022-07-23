import gleeunit
import gleeunit/should
import lex
import lex.{Add, Num}

pub fn main() {
  gleeunit.main()
}

// gleeunit test functions end in `_test`
pub fn hello_world_test() {
  lex.lex("1 + 1")
  |> should.equal([Num(1), Add, Num(1)])
}
