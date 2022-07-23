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

#endif
