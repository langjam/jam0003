#pragma once

namespace bytecode {

enum Opcode {
  // Debug/error ops
  kTrap = 0x00,
  kExit = 0x01,
  kBreakpoint = 0x02,
  kPrintLong = 0x03,
  kPrintChar = 0x04,
  
  // Binary Arithmetic operations
  kNop = 0x10,
  kAddLong = 0x11,
  kSubLong = 0x12,
  kMulLong = 0x13,
  kIDivLong = 0x14,    // pushes TOS1 / TOS
  kModuloLong = 0x15,  // pushes TOS1 % TOS
  kLeftShiftLong = 0x16, // pushes TOS1 << TOS
  kRightShiftLogicalLong = 0x17, // pushes TOS1 >> TOS
  kRightShiftArithmeticLong = 0x18, // pushes TOS1 >>> TOS
  kLogicalAndLong = 0x19,
  kLogicalOrLong = 0x1A,
  kAddFloat = 0x1B,
  kSubFloat = 0x1C,
  kMulFloat = 0x1D,
  kDivFloat = 0x1E,

  // Unary Arithmetic operations
  kLogicalNeg = 0x20,
  kBinaryNeg = 0x21,
  kArithmeticNeg = 0x22,

  // Loading/storing
  kImmByte = 0x40,
  kConstant = 0x41,
  kBiasConstantWindow = 0x42,
  kLoadGlobal = 0x43,
  kStoreGlobal = 0x44,
  kBiasGlobalWindow = 0x45,
  kStoreAuxiliary = 0x46,
  kLoadAuxiliary = 0x47,
  kMoveAuxiliary = 0x48, // moves the value and sets the existing one to nothing

  // Stack manipulation
  kDup = 0x60,
  kDup2 = 0x61,
  kRot2 = 0x62,
  kRot3 = 0x63,
  kDrop = 0x64,

  // Control flow
  kJump = 0x70, // jumps to pc + IMM
  kTestAndJump = 0x71, // pops TOS, jumps to pc + IMM iff TOS != 0
  kCall = 0x72,
  kReturn = 0x73,

  // Objects
  kAllocate = 0x80, // Allocates TOS bytes, pushes pointer to stack
  kAllocateImm = 0x81, // Allocates IMM bytes, pushes pointer to stack
  kDeallocate = 0x82, // Deallocates the buffer pointed to by TOS
  kLoadClassConstructor = 0x83, // Class determined by Imm
  kLoadClassDestructor = 0x84, // Class determined by Imm
  kLoadObjectField = 0x85, // push IMMth field of object @ *TOS
  kStoreObjectField = 0x86, // pop TOS into IMMth field of object @ *TOS1
  kLoadObjectDestructor = 0x87, // loads destructor based on object in frame
  kMoveOutObjectField = 0x88, // moves the field and sets the exiting one to nothing
  kDestroy = 0x89, // pop TOS and call its destructor, if it has one, otherwise do nothing
};

} // namespace bytecode
