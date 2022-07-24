#ifndef VM_H
#define VM_H
#include "vmintrin.h"

#define MEMORY_SIZE 256
struct VM {
  Program prog;
  CallStack csk;
  Stack sk;

  uint ip;

  float regs[REG_COUNT];
	float mem[256];
};

VM vm_init(Program prog);
int vm_run_scr(VM *vm, u8 screen[256][256]);
int vm_run(VM *vm);

#endif
