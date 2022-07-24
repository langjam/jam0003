#include "gtest/gtest.h"

#include <fstream>
#include <iostream>
#include <string>

#include "src/parser/parser.hh"

namespace {
TEST(ParserTest, GrammarTestSimple) {
  std::ifstream stream("./test/test_programs/simple.aasm", std::ios::in);

  antlr4::ANTLRInputStream input(stream);
  parser::AasmLexer lexer(&input);
  antlr4::CommonTokenStream tokens(&lexer);
  parser::AasmParser parser(&tokens);

  tokens.fill();
  for (auto token : tokens.getTokens()) {
    std::cout << token->toString() << std::endl;
  }
}
} // namespace
