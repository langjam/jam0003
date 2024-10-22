#ifndef H_VMINTRIN_SRC
#define H_VMINTRIN_SRC

#include "wasm.h"

enum Reg {
  Reg_X,
  Reg_Y,
  Reg_Ret, // Return value
  Reg_Out, // Output pixel
  Reg_Time, // Time in seconds
  Reg_Base, // Memory address base
  Reg_Memory, // Memory dereferenced
  Reg_A, // Accumulator

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

  InstrType_Exit, // () Exit program

	InstrType_Call, // (ADDRESS) pushes IP to the call stack and jumps to ADDRESS
	InstrType_Ret, // Pops from the call stack and jumps to the popped value
	InstrType_Copy, // (N) copies stack item N from the base

	InstrType_Sin,
	InstrType_Cos,
  InstrType_Pow,
  InstrType_SetBase,

  InstrType_Ba,
  InstrType_Bz,
  InstrType_Bnz,
  InstrType_Bp,
  InstrType_Bn,

	InstrType_Print,

  INSTRTYPE_COUNT
};

struct Instr {
  InstrType type;
  uint argument;
};

struct Program {
  uint start = 0;
  Instr *instrs;
  uint instrs_len;
};

#define STACK_MAX (1 << 11)

struct Call {
	uint ret;
	uint base;
};

struct CallStack {
  Call calls[STACK_MAX];
  uint calls_len;
};

struct Stack {
  float values[STACK_MAX];
  uint values_len;
};

#endif
