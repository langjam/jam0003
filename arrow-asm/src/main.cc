// copyright Valkyrie AFTERSCHOOL INTERNATIONAL(c) 2022(tm) (r)

#include <cassert>
#include <cstdio>
#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <variant>

// headers defined by targets that are child nodes in the
// directory tree can be referred to by the package path directly
#include "ast/ast.hh"
#include "ast/bytecode_emitter.hh"
#include "parser/parser.hh"
#include "bytecode/bytecode.hh"
#include "bytecode/interpreter.hh"
#include "spdlog/spdlog.h"
#include "spdlog/cfg/env.h"

int main(int argc, char *argv[]) {
  if (argc < 2) {
    spdlog::error("no inputs given");
    return -1;
  }

  // load the file passed in by the first argument as our input program
  std::string program_text{""};
  if (!std::strcmp(argv[1], "-")) { // conventionally refers to stdin, use it instead of file
    while (!std::cin.eof()) {
      std::string line_buffer;
      std::cin >> line_buffer;
      program_text += line_buffer;
    }
  } else {
    std::stringstream str_buffer;
    std::ifstream ifs(argv[1]);
    if (!ifs) {
      spdlog::error("could not open input file");
      return -1;
    }
    str_buffer << ifs.rdbuf();
    program_text = str_buffer.str();
  }

  spdlog::cfg::load_env_levels();

  // now drive our compilation stack (sans code emission) to run it
  // also just a general top-level TODO: need to make sure we actually pass
  // along source locations so that we can give useful errors
  ast::ProgramNode program = parser::ParseFullProgram(program_text);
  bytecode::BytecodeExecutable executable = ast::LowerAst(program);
  return bytecode::InterpretBytecode(executable);
}
