#include "debug.h"
#include "vmintrin.h"
#include "wasm.h"

void putval(char c) {
  if (c == '\n') {
    putval("<br/>");
  } else {
    wasm_putchar(WASM_STDOUT, c);
  }
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

  putval(sint(v));
  putval('.');
  putval(sint((v-sint(v)) * 1000000));
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
  // classes
  // 0 = no args
  // 1 = int arg
  // 2 = float arg
  // 3 = reg arg
  int classes[] = {
    0, 0, 0, 0,
    0, 1, 2, 3, 3
  };
  static const char *names[] = {
    "ADD", "SUB", "MUL", "DIV",
    "MOD", "POP", "IMM", "LOAD", "STORE"
  };
  static const char *regs[] = {
    "X", "Y", "RET", "OUT"
  };

  tprintf("<pre>");
  const char *fmt = "<span style=\"color: blue;\">{}</span> <span style=\"color: red;\">{}</span> {}\n";
  const char *fmt2 = "<span style=\"color: blue;\">{}</span> <span style=\"color: red;\">{}</span> %{}\n";

  for (sint i=0; i < prog.instrs_len; i++) {
    switch (classes[prog.instrs[i].type]) {
      case 0:
        tprintf(fmt, i, names[prog.instrs[i].type], "");
        break;
      case 1:
        tprintf(fmt, i, names[prog.instrs[i].type], (sint)prog.instrs[i].argument);
        break;
      case 2:
        tprintf(fmt, i, names[prog.instrs[i].type], *(float*)&prog.instrs[i].argument);
        break;
      case 3:
        tprintf(fmt2, i, names[prog.instrs[i].type], regs[prog.instrs[i].argument]);
        break;
      default:
        tprintf("[INVALID]\n");
    }
  }
  tprintf("</pre>");
}
