#include "gtest/gtest.h"

#include <iostream>
#include <fstream>
#include <sstream>
#include <string>

#include "src/parser/parser.hh"
#include "src/ast/ast.hh"
#include "src/ast/bytecode_emitter.hh"
#include "src/bytecode/bytecode.hh"
#include "src/bytecode/interpreter.hh"
#include "spdlog/spdlog.h"

namespace {

TEST(LowerAstTest, NopTest) {
  // Test Program
  std::ifstream stream("./test/test_programs/simple.aasm", std::ios::in);
  std::stringstream ss;
  ss << stream.rdbuf();

  // Expected Instructions
  std::vector<bytecode::Instruction> instructions {
    { bytecode::Opcode::kLoadAuxiliary, 32 },
    { bytecode::Opcode::kDup, 0 },
    { bytecode::Opcode::kAddLong, 0 },
    { bytecode::Opcode::kStoreAuxiliary, 32 }
  };

  auto program = parser::ParseFullProgram(ss.str());
  auto executable = ast::LowerAst(program);
  for (auto const& c : executable.chunks) {
    spdlog::info("Logging chunk...");
    for (auto const cons : c.constants) {
      spdlog::info("Constant: {}", cons);
    }
    for (auto const& inst : c.code) {
      spdlog::info("Inst: 0x{0:x}, {1:d}", inst.opcode, inst.param);
    }
  }
  spdlog::info("Running code...");
  int return_code = bytecode::InterpretBytecode(executable);
  EXPECT_EQ(return_code, 0);
}

TEST(LowerAstTest, ExitTest) {
  // Test Program
  std::ifstream stream("./test/test_programs/exit.aasm", std::ios::in);
  std::stringstream ss;
  ss << stream.rdbuf();

  // Expected Instructions
  std::vector<bytecode::Instruction> instructions {
    { bytecode::Opcode::kConstant, 0 },
    { bytecode::Opcode::kExit, 0 }
  };

  auto program = parser::ParseFullProgram(ss.str());
  auto executable = ast::LowerAst(program);
  for (auto const& c : executable.chunks) {
    spdlog::info("Logging chunk...");
    for (auto const cons : c.constants) {
      spdlog::info("Constant: {}", cons);
    }
    for (auto const& inst : c.code) {
      spdlog::info("Inst: 0x{0:x}, {1:d}", inst.opcode, inst.param);
    }
  }
  spdlog::info("Running code...");
  int return_code = bytecode::InterpretBytecode(executable);
  EXPECT_EQ(return_code, 1);
}

TEST(LowerAstTest, FibTest) {
  // Test Program
  std::ifstream stream("./test/test_programs/fib.aasm", std::ios::in);
  std::stringstream ss;
  ss << stream.rdbuf();

  auto program = parser::ParseFullProgram(ss.str());
  auto executable = ast::LowerAst(program);
  for (auto const& c : executable.chunks) {
    spdlog::info("Logging chunk...");
    for (auto const cons : c.constants) {
      spdlog::info("Constant: {}", cons);
    }
    for (auto const& inst : c.code) {
      spdlog::info("Inst: 0x{0:x}, {1:d}", inst.opcode, inst.param);
    }
  }
  spdlog::info("Running code...");
  int return_code = bytecode::InterpretBytecode(executable);
  EXPECT_EQ(return_code, 144);
}

}