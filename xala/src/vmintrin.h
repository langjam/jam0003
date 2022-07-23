#ifndef H_VMINTRIN_SRC
#define H_VMINTRIN_SRC

#include "wasm.h"

enum Reg {
  Reg_Ret,
  Reg_Out,
	REG_COUNT
};

enum InstrType {
  InstrType_Add, // () Add 2 values from the top of the stack
  InstrType_Sub, // () Subtract 2 values from the top of the stack
  InstrType_Mul, // () Multiply 2 values from the top of the stack
  InstrType_Div, // () Divide 2 values from the top of the stack
  InstrType_Mod, // () Mod 2 values from the top of the stack

  InstrType_Pop, // (NUMBER OF ITEMS TO POP) Remove N items from stack
  InstrType_Imm, // (NUMBER) Push immediate value
  InstrType_Load, // (REGISTER) Push register value onto stack
  InstrType_Store, // (REGISTER) Pop stack value into register
};

struct Instr {
  InstrType type;
  uint argument;
};

struct Program {
  Instr *instrs;
  uint instrs_len;
};

#define STACK_MAX (1 << 16)

struct CallStack {
  uint return_addrs[STACK_MAX];
  uint return_addrs_len;
};

struct Stack {
  float values[STACK_MAX];
  uint values_len;
};

#endif
