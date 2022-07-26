#include "gtest/gtest.h"

#include "src/ast/ast.hh"
#include "src/ast/bytecode_emitter.hh"
#include "src/parser/parser.hh"
#include "src/bytecode/bytecode.hh"
#include "src/bytecode/interpreter.hh"

namespace {
TEST(NopTest, NopTest) {
  auto program = parser::ParseFullProgram("nop\n");
  auto executable = ast::LowerAst(program);
  int return_value = bytecode::InterpretBytecode(executable);
  EXPECT_EQ(return_value, 0); // is this actually true though
}
}
