#include "gtest/gtest.h"

#include "src/bytecode/bytecode.hh"
#include "src/bytecode/interpreter.hh"
#include "src/bytecode/opcodes.hh"
#include "spdlog/cfg/env.h"

namespace {

using namespace bytecode;

BytecodeExecutable makeFlatBinary( std::vector<Instruction> code) {
  BytecodeChunk chunk {
    .code = code,
  };
  return BytecodeExecutable {
    .chunks {
      BytecodeChunk { 
        .code = {
          { kReturn }
        } 
      },
      chunk,
    },
    .chunk_locations {
      0ull,
      0ull,
    },
  };
}

TEST(Bytecode, BasicTest) {
  spdlog::cfg::load_env_levels();
  auto bin = makeFlatBinary( {
      { kImmByte, 3 },
      { kExit },
  });
  int return_value = InterpretBytecode(bin);
  EXPECT_EQ(return_value, 3);
}

TEST(Bytecode, ConstantTestBasic) {
  spdlog::cfg::load_env_levels();
  auto bin = makeFlatBinary( {
      { kConstant, 1 },
      { kBiasConstantWindow, 2 },
      { kConstant, 0 },
      { kAddLong },
      { kExit },
  });
  bin.chunks[0].constants.push_back(1);
  bin.chunks[0].constants.push_back(10);
  bin.chunks[0].constants.push_back(100);
  bin.chunks[0].constants.push_back(1000);
  int return_value = InterpretBytecode(bin);
  EXPECT_EQ(return_value, 110);
}

TEST(Bytecode, CallFrameRestoration) {
  BytecodeChunk caller {
    .code = {
      { kImmByte, 2 },
      { kImmByte, 3 },
      { kBiasConstantWindow, -1 },
      { kConstant, 1 }, // load 50 onto stack; [50 3 2]
      { kImmByte, 1 }, // address of next chunk; [1 50 3 2]
      { kCall }, // [-5 120 50 3 2]
      { kDrop }, // [120 50 3 2]
      { kAddLong }, // [170 3 2]
      { kConstant, 2 }, // [100 170 3 2]
      { kAddLong }, // [270 3 2]
      { kAddLong }, // [273 2]
      { kExit }, // [2], return [273]
    },
    .constants = {
      50,
      100,
      1000,
    },
  };
  BytecodeChunk callee {
    .code = {
      { kImmByte, 120 },
      { kImmByte, -5 },
      { kReturn }
    }
  };

  auto exe = BytecodeExecutable {
    .chunks = { caller, callee },
    .chunk_locations = { 0x0000ull, 0x1000ull },
  };
  int return_value = InterpretBytecode(exe);

  EXPECT_EQ(return_value, 273);
}

}
