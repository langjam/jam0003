import { tokenize } from "./tokenizer";

function test1() {
  const program = "world hello";
  const tokens = tokenize(program);
  console.log(tokens);
}
test1();

function test2() {
  const program = `
  world hello

  legend water #0000ff
  legend sand #ff0000
  legend grass #00ff00
  legend rock_lower #899012
  legend rock_upper #999999

  region island
  island 50%
  island 7x7  // max size
  island water 25%
  island sand 25%
  island grass 25%

  region rocks
  rocks 50%
  rocks 5x5
  rocks rock_upper 50%
  rocks rock_lower 50%
  `;
  const tokens = tokenize(program);
  console.log(tokens);
}
test2();
