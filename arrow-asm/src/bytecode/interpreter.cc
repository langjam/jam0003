#include "interpreter.hh"

#include <cassert>
#include <array>
#include <deque>
#include <stack>
#include <iostream>
#include <unordered_set>

#include "spdlog/spdlog.h"
#include "spdlog/cfg/env.h"
#include "fmt/core.h"
#include "fmt/color.h"

#include "opcodes.hh"

#define DISPATCH_NO_INCR_PC()                                    \
  do {                                                           \
  instr = chunks[chunk_idx].code[pc];         \
  debug_dispatch_hook();                                         \
  goto *(opcode_dispatch_table[static_cast<int>(instr.opcode)]); \
  } while (false);                                               \

#define DISPATCH()      \
  do {                  \
  pc++;    \
  DISPATCH_NO_INCR_PC() \
  } while (false);      \

// I tried doing this with a repetition macro but it was 5am so I gave up
#define EMPTY_OPCODE &&BadOpcode
#define EMPTY_OPCODES_4()  \
  &&BadOpcode,             \
  &&BadOpcode,             \
  &&BadOpcode,             \
  &&BadOpcode
#define EMPTY_OPCODES_8()  \
  &&BadOpcode,             \
  &&BadOpcode,             \
  &&BadOpcode,             \
  &&BadOpcode,             \
  &&BadOpcode,             \
  &&BadOpcode,             \
  &&BadOpcode,             \
  &&BadOpcode
#define EMPTY_OPCODES_12() \
  &&BadOpcode,             \
  &&BadOpcode,             \
  &&BadOpcode,             \
  &&BadOpcode,             \
  &&BadOpcode,             \
  &&BadOpcode,             \
  &&BadOpcode,             \
  &&BadOpcode,             \
  &&BadOpcode,             \
  &&BadOpcode,             \
  &&BadOpcode,             \
  &&BadOpcode
#define EMPTY_OPCODES_16() \
  &&BadOpcode,             \
  &&BadOpcode,             \
  &&BadOpcode,             \
  &&BadOpcode,             \
  &&BadOpcode,             \
  &&BadOpcode,             \
  &&BadOpcode,             \
  &&BadOpcode,             \
  &&BadOpcode,             \
  &&BadOpcode,             \
  &&BadOpcode,             \
  &&BadOpcode,             \
  &&BadOpcode,             \
  &&BadOpcode,             \
  &&BadOpcode,             \
  &&BadOpcode

int bytecode::InterpretBytecode(BytecodeExecutable executable) {
  auto& chunks = executable.chunks;
  auto& symtab = executable.symbol_table;

  std::array<void*, 256> opcode_dispatch_table = {
    /********** 0x00 **********/
    &&Trap,
    &&Exit,
    &&Breakpoint,
    &&PrintLong,
    &&PrintChar,
    EMPTY_OPCODE,
    EMPTY_OPCODE,
    EMPTY_OPCODE,
    EMPTY_OPCODES_8(),
    /********** 0x10 **********/
    &&Nop,
    &&AddLong,
    &&SubLong,
    &&MulLong,
    &&IDivLong,
    &&ModuloLong,
    &&LeftShiftLong,
    &&RightShiftLogicalLong,
    &&RightShiftArithmeticLong,
    &&LogicalAndLong,
    &&LogicalOrLong,
    &&AddFloat,
    &&SubFloat,
    &&MulFloat,
    &&DivFloat,
    EMPTY_OPCODE,
    /********** 0x20 **********/
    &&LogicalNeg,
    &&BinaryNeg,
    &&ArithmeticNeg,
    EMPTY_OPCODE,
    EMPTY_OPCODES_4(),
    EMPTY_OPCODES_8(),
    /********** 0x30 **********/
    EMPTY_OPCODES_16(),
    /********** 0x40 **********/
    &&ImmByte,
    &&Constant,
    &&BiasConstantWindow,
    &&LoadGlobal,
    &&StoreGlobal,
    &&BiasGlobalWindow,
    &&StoreAuxiliary,
    &&LoadAuxiliary,
    &&MoveAuxiliary,
    EMPTY_OPCODE,
    EMPTY_OPCODE,
    EMPTY_OPCODE,
    EMPTY_OPCODES_4(),
    /********** 0x50 **********/
    EMPTY_OPCODES_16(),
    /********** 0x60 **********/
    &&Dup,
    &&Dup2,
    &&Rot2,
    &&Rot3,
    &&Drop,
    EMPTY_OPCODE,
    EMPTY_OPCODE,
    EMPTY_OPCODE,
    EMPTY_OPCODES_8(),
    /********** 0x70 **********/
    &&Jump,
    &&TestAndJump,
    &&Call,
    &&Return,
    EMPTY_OPCODES_12(),
    /********** 0x80 **********/
    &&Allocate,
    &&AllocateImm,
    &&Deallocate,
    &&LoadClassConstructor,
    &&LoadClassDestructor,
    &&LoadObjectField,
    &&StoreObjectField,
    &&LoadObjectDestructor,
    &&MoveOutObjectField,
    &&Destroy,
    EMPTY_OPCODE,
    EMPTY_OPCODE,
    EMPTY_OPCODES_4(),
    /********** 0x90 **********/
    EMPTY_OPCODES_16(),
    /********** 0xA0 **********/
    EMPTY_OPCODES_16(),
    /********** 0xB0 **********/
    EMPTY_OPCODES_16(),
    /********** 0xC0 **********/
    EMPTY_OPCODES_16(),
    /********** 0xD0 **********/
    EMPTY_OPCODES_16(),
    /********** 0xE0 **********/
    EMPTY_OPCODES_16(),
    /********** 0xF0 **********/
    EMPTY_OPCODES_16(),
  };

  // Set up virtual machine state
  //  - stacks
  std::stack<Value> data_stack;
  std::deque<Value> auxiliary_stack(kAuxiliaryStackBaseSize);
  struct StackFrame {
    int pc;
    int chunk_idx;
    int constant_window_base;
    int global_window_base;
  };
  std::stack<StackFrame> return_stack;
  std::unordered_set<Value> pointers;
  Instruction instr;
  //  - various counters
  int chunk_idx = 1;       // current executing chunk ID. We start at 1, 0 is our implicit empty chunk
  int cycle_count = 0;     // how many instructions we've executed
  int pc = 0; // within chunk
  int constant_window_base = 0; // within current chunk
  int global_window_base = 0; // within current chunk

  if (chunk_idx >= chunks.size()) {
    // We have no chunks, basically a noop program.
    return 0;
  }

  // debug-only hook that gets called every time we dispatch to let us log stuff
  auto debug_dispatch_hook = [&](){
    spdlog::debug("{:0>6} {:0>8} {:0>16x}: {} {}", cycle_count, chunk_idx,
                  pc * 2, instr.opcode, instr.param);
    if (pc >= chunks[chunk_idx].code.size()) {
      spdlog::critical("overran end of bytecode chunk");
      std::abort();
    }
    cycle_count++;
  };

  auto is_pointer = [&pointers](Value const& val) -> bool {
    return pointers.find(val) != pointers.end();
  };
  auto free_ptr = [&pointers](Value const& val) -> void {
    pointers.erase(val);
  };
  auto new_ptr = [&pointers](Value const& val) -> void {
    pointers.insert(val);
  };
  auto Pop = [&data_stack]() -> Value {
    Value val = data_stack.top();
    data_stack.pop();
    return val;
  };
  auto Peek = [&data_stack]() -> Value {
    return data_stack.top();
  };
  auto Push = [&data_stack](Value val) {
    data_stack.push(val);
  };

  DISPATCH_NO_INCR_PC(); // kick it off and it'll drive itself

BadOpcode:
  spdlog::critical("bad opcode at {:0>8}:{:0>16x}", chunk_idx, pc * 2);
  std::abort();
Trap:
  spdlog::critical("user code trap");
  std::abort();
Exit:
  return Peek();
Breakpoint:
  // TODO unimplemented
  goto BadOpcode;
PrintLong: {
  Value val = Pop();

  std::cout << std::dec << val << std::endl;
  DISPATCH();
}
PrintChar: {
  Value val = Pop();
  if (val >= 128)
    spdlog::error("only ascii characters are supported");
  std::cout << (char) val;
  DISPATCH();
}
Nop:
  DISPATCH();
AddLong: {
  long tos {Pop()};
  long tos1 {Pop()};
  Push(tos1 + tos);
  DISPATCH();
}
SubLong: {
  long tos {Pop()};
  long tos1 {Pop()};
  Push(tos1 - tos);
  DISPATCH();
}
MulLong: {
  long tos {Pop()};
  long tos1 {Pop()};
  Push(tos1 * tos);
  DISPATCH();
}
IDivLong: {
  long tos {Pop()};
  long tos1 {Pop()};
  Push(tos1 / tos);
  DISPATCH();
}
ModuloLong: {
  long tos {Pop()};
  long tos1 {Pop()};
  Push(tos1 % tos);
  DISPATCH();
}
LeftShiftLong: {
  long tos {Pop()};
  long tos1 {Pop()};
  Push(tos1 << tos);
  DISPATCH();
}
RightShiftLogicalLong: {
  uint64_t tos {static_cast<uint64_t>(Pop())};
  uint64_t tos1 {static_cast<uint64_t>(Pop())};
  Push(tos1 >> tos);
  DISPATCH();
}
RightShiftArithmeticLong: {
  long tos {Pop()};
  long tos1 {Pop()};
  Push(tos1 >> tos);
  DISPATCH();
}
ImmByte:
  Push(static_cast<int8_t>(instr.param));
  DISPATCH();
Constant: {
  int index = constant_window_base + static_cast<int8_t>(instr.param);
  Value val = chunks[chunk_idx].constants.at(index);
  Push(val);
  DISPATCH();
}
BiasConstantWindow: {
  constant_window_base += static_cast<int8_t>(instr.param);
  DISPATCH();
}
LoadGlobal: {
  auto& globals = executable.globals;
  int index = global_window_base + static_cast<int8_t>(instr.param);
  if (globals.size() <= index) {
    spdlog::error("access to uninitialized global value at index {}", index);
    spdlog::debug("resizing global-cache to {} elements", index + 1);
    globals.resize(index + 1);
    globals[index] = 0;
  }
  Push(globals[index]);
  DISPATCH();
}
BiasGlobalWindow: {
  global_window_base += static_cast<int8_t>(instr.param);
  DISPATCH();
}
StoreGlobal: {
  auto& globals = executable.globals;
  int index = global_window_base + static_cast<int8_t>(instr.param);
  if (globals.size() <= index) {
    spdlog::debug("resizing global-cache to {} elements", index + 1);
    globals.resize(index + 1);
  }
  globals[index] = Pop();
  DISPATCH();
}
Dup:
  Push(Peek());
  DISPATCH();
Dup2: {
  Value tos = Pop();
  Value tos1 = Peek();
  Push(tos);
  Push(tos1);
  Push(tos);
  DISPATCH();
}
Rot2: {
  Value tos {Pop()};
  Value tos1 {Pop()};
  Push(tos);
  Push(tos1);
  DISPATCH();
}
Rot3: {
  Value tos {Pop()};
  Value tos1 {Pop()};
  Value tos2 {Pop()};
  Push(tos1);
  Push(tos);
  Push(tos2);
  DISPATCH();
}
Drop:
  Pop();
  DISPATCH();
LogicalAndLong: {
  long tos {Pop()};
  long tos1 {Pop()};
  Push(tos1 & tos);
  DISPATCH();
}
LogicalOrLong: {
  long tos {Pop()};
  long tos1 {Pop()};
  Push(tos1 | tos);
  DISPATCH();
}
AddFloat: {
  // TODO
  goto BadOpcode;
}
SubFloat: {
  // TODO
  goto BadOpcode;
}
MulFloat: {
  // TODO
  goto BadOpcode;
}
ArithmeticNeg: {
  Push(-Pop());
  DISPATCH();
}
BinaryNeg: {
  Push(~Pop());
  DISPATCH();
}
LogicalNeg: {
  Push(!Pop());
  DISPATCH();
}
DivFloat: {
  // TODO
  goto BadOpcode;
}
Jump: {
  pc += static_cast<int8_t>(instr.param);
  spdlog::debug("jumping to pc {}", pc);
  DISPATCH_NO_INCR_PC();
}
TestAndJump: {
  Value test = Pop();
  if (test) {
    pc += static_cast<int8_t>(instr.param);
    spdlog::debug("test {}: jumping to pc {:0>16x}", test, pc);
  } else {
    spdlog::debug("test {}: not jumping", test);
    pc ++;
  }
  DISPATCH_NO_INCR_PC();
}
Call: {
  return_stack.push({.pc = pc, .chunk_idx = chunk_idx, .constant_window_base = constant_window_base, .global_window_base = global_window_base});
  pc = 0;
  chunk_idx = instr.param;
  constant_window_base = 0;
  global_window_base = 0;
  for (int i = 0; i < kAuxiliaryStackWindowSize; i++) { // better way to do this?
    auxiliary_stack.push_front(0);
  }
  spdlog::debug("Call   => Frame [{:>4}]", chunk_idx);
  DISPATCH_NO_INCR_PC();
}
Return: {
  auto frame = return_stack.top();
  return_stack.pop();
  pc = frame.pc + 1;
  chunk_idx = frame.chunk_idx;
  constant_window_base = frame.constant_window_base;
  global_window_base = frame.global_window_base;
  for (int i = 0; i < kAuxiliaryStackWindowSize; i++) { // better way to do this?
    // TODO fix destructors
    // executable.classes[i -???-> [0]].dtor_chunk.idx);???
    // need type tags in the Value 
    auxiliary_stack.pop_front();
  }
  spdlog::debug("Return => Frame [{:>4}]", chunk_idx);
  DISPATCH_NO_INCR_PC();
}
LoadClassConstructor: {
  Push(executable.classes.at(instr.param).ctor_chunk.idx);
  DISPATCH();
}
LoadClassDestructor: {
  Push(executable.classes.at(instr.param).dtor_chunk.idx);
  DISPATCH();
}
LoadObjectDestructor: {
  intptr_t ptr = Peek();
  if (!is_pointer(ptr)) {
    spdlog::critical("value {} is not a pointer!",
                     reinterpret_cast<void*>(ptr));
    
    std::abort();
  }
  Value* object_frame = reinterpret_cast<Value*>(ptr);
  Push(executable.classes[object_frame[0]].dtor_chunk.idx);
  DISPATCH();
}
LoadObjectField: {
  int index = instr.param;
  intptr_t ptr = Pop();
  Value* object_frame = reinterpret_cast<Value*>(ptr);
  if (index < 0 || index >= executable.classes[object_frame[0]]
                            .fields.size()) {
    spdlog::critical("invalid offset {} in object access at {}",
                     index, reinterpret_cast<void*>(object_frame));
    std::abort();
  }
  Value val = object_frame[index + 1];
  if (is_pointer(val)) {
      spdlog::critical("offset {} in object access at {} is a pointer!",
                     index, reinterpret_cast<void*>(object_frame));
    std::abort();
  }
  Push(object_frame[index + 1]); // + 1 bc slot 0 is the class index
  DISPATCH();
}
MoveOutObjectField: {
  int index = instr.param;
  intptr_t ptr = Pop();
  if (!is_pointer(ptr)) {
    spdlog::critical("value {} is not a pointer!",
                     reinterpret_cast<void*>(ptr));
    
    std::abort();
  }
  Value* object_frame = reinterpret_cast<Value*>(ptr);
  if (index < 0 || index >= executable.classes[object_frame[0]]
                            .fields.size()) {
    spdlog::critical("invalid offset {} in object access at {}",
                     index, reinterpret_cast<void*>(object_frame));
    
    std::abort();
  }
  Value val = object_frame[index + 1];
  object_frame[index + 1] = 0;
  Push(val);
  DISPATCH();
}
StoreObjectField: {
  int index = instr.param;
  Value val = Pop();
  intptr_t ptr = Pop();
  if (!is_pointer(ptr)) {
    spdlog::critical("value {} is not a pointer!",
                     reinterpret_cast<void*>(ptr));
    
    std::abort();
  }
  Value* object_frame = reinterpret_cast<Value*>(ptr);
  if (index < 0 || index >= executable.classes.size()) {
    spdlog::critical("invalid offset {} in object access at {}",
                     index, reinterpret_cast<void*>(object_frame));
    std::abort();
  }
  // TODO type check, but not too harshly because we use this to write a pointer in an arrow op
  object_frame[index + 1] = val; // + 1 bc slot 0 is the class index
  DISPATCH();
}
Deallocate: {
  intptr_t ptr = Pop();
  if (is_pointer(ptr)) {
    spdlog::debug("Deallocated at {}", reinterpret_cast<void*>(ptr));
    delete[] reinterpret_cast<Value*>(ptr);
    free_ptr(ptr);
  } else {
    spdlog::debug("Value {} is not a pointer!", reinterpret_cast<void*>(ptr));
  }
  DISPATCH();
}
Destroy: {
  intptr_t ptr = Pop();
  if (is_pointer(ptr)) {
    // Basically, we call the destructor here
    // First we have to get it
    instr.param = static_cast<int8_t>(executable.classes[*reinterpret_cast<Value*>(ptr)]
                                      .dtor_chunk.idx);
    // Then, we want to CALL, but we have to set our param first
    auxiliary_stack[0] = ptr;
    goto Call;
    // TODO: But then we ALSO want to deallocate it! Otherwise we leak :))
  }
  // Otherwise, simply move on
  DISPATCH();
}
Allocate: {
  size_t size = Pop();
  intptr_t ptr = reinterpret_cast<intptr_t>(new Value[size]);
  spdlog::debug("Allocated {} bytes at {}",
                size, reinterpret_cast<void*>(ptr));
  new_ptr(ptr);
  Push(ptr);
  DISPATCH();
}
AllocateImm: {
  size_t size = instr.param;
  intptr_t ptr = reinterpret_cast<intptr_t>(new Value[size]);
  spdlog::debug("Allocated {} bytes at {}",
                size, reinterpret_cast<void*>(ptr));
  new_ptr(ptr);
  Push(ptr);
  DISPATCH();
}
LoadAuxiliary: {
  auto val = auxiliary_stack.at(instr.param);
  // TODO: Consider adding this back
  // if (is_pointer(val)) {
  //   spdlog::critical("Attempting to load a pointer {}! Move instead.",
  //                   val);
  //   std::abort();
  // }
  Push(val);
  DISPATCH();
}
MoveAuxiliary: {
  auto val = auxiliary_stack.at(instr.param);
  auxiliary_stack.at(instr.param) = 0;
  Push(val);
  DISPATCH();
}
StoreAuxiliary: {
  auxiliary_stack.at(instr.param) = Pop();
  DISPATCH();
}

  __builtin_unreachable();
}
