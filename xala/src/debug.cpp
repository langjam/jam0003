#include "debug.h"
#include "vmintrin.h"
#include "wasm.h"

void putval(char c) {
  wasm_putchar(WASM_STDOUT, c);
}

void putval(const char *s) {
  while (*s) {
    putval(*s++);
  }
}

void putval(sint v) {
  char buf[16] = { 0 };
  int i = 0;

  if (v == 0) {
    putval('0');
    return;
  }

  if (v < 0) {
    v *= -1;
    putval('-');
  }

  while (v > 0) {
    buf[i++] = v % 10 + '0';
    v /= 10;
  }

  while (i--) {
    putval(buf[i]);
  }
}

void putval(uint v) {
  putval(sint(v));
}

void putval(double v) {
  if (v < 0) {
    v *= -1;
    putval('-');
  }

  putval(int(v));
  putval('.');
  putval(int((v-int(v)) * 1000000));
}

void putval(float v) {
  putval(double(v));
}

void putval(Span v) {
  while (v.l > 0 && *v.s) {
    v.l -= 1;
    putval(*v.s++);
  }
}

void tprintf(const char* format) {
  putval(format);
}

void putval(Program prog) {
  static const char *names[] = {
    "ADD", "SUB", "MUL", "DIV",
    "MOD", "POP", "IMM", "LOAD", "STORE"
  };

  tprintf("<pre>");
  const char *fmt = "<span style=\"color: blue;\">{}</span> <span style=\"color: red;\">{}</span> {}\n";

  for (sint i=0; i < prog.instrs_len; i++) {
    if (prog.instrs[i].type == InstrType_Imm) {
      tprintf(fmt, i, names[prog.instrs[i].type], *(float*)&prog.instrs[i].argument);
    } else {
      tprintf(fmt, i, names[prog.instrs[i].type], (sint)prog.instrs[i].argument);
    }
  }
  tprintf("</pre>");
}
