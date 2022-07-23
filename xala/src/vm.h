#ifndef VM_H
#define VM_H
#include "vmintrin.h"

struct VM {
  Program prog;
  CallStack csk;
  Stack sk;

  uint ip;

  float regs[REG_COUNT];
};

VM vm_init(Program prog);
void vm_run_for_pixel(VM *vm, u8 screen[256][256], sint x, sint y);
int vm_run(VM *vm);

#endif
