#include "vm.h"
#include "vmintrin.h"
#include "wasm.h"
#include "debug.h"
#include "parser.h"

char tolower(char c) {
  if (c >= 'A' && c <= 'Z') {
    c = c-'A'+'a';
  }
  return c;
}

bool span_equal(Span a, Span b) {
  if (a.l != b.l) {
    return false;
  }
  while (a.l && *a.s && *b.s) {
    if (tolower(*a.s++) != tolower(*b.s++)) {
      return false;
    }
    a.l -= 1;
  }
  return true;
}

void putstr(Wasm_StreamId stream, const char *s) {
  while (*s) {
    wasm_putchar(stream, *s++);
  }
}

u8 bytes[256][256] = {};
u8 new_bytes[256][256] = {};
u8 thresh[256][256];
u8 source[1 << 16];
uint source_len;

bool in_bounds(int x, int y) {
  return x >= 0 && y >= 0 && x < 256 && y < 256;
}

float bayer_factor(sint x, sint y, sint s) {
  const float bayer_base[2][2] = {{0, 2}, {3, 1}};
  float factor = 0;
  sint stg = s;

  while (stg > 1) {
    factor *= 4;
    factor += bayer_base[x%2][y%2];
    x /= 2;
    y /= 2;
    stg /= 2;
  }

  return factor;
}

// fill out threshold matrix
void make_bayer() {
  for (int i = 0; i < 256; ++i) {
    for (int j = 0; j < 256; ++j) {
      int fac = bayer_factor(i, j, 256);
      int b = int(255.0*(fac/(256.0*256.0)));
      thresh[j][i] = b;
    }
  }
}

void dither() {
  for (int i = 0; i < 256; ++i) {
    for (int j = 0; j < 256; ++j) {
      if (bytes[i][j] > thresh[i][j]) {
        new_bytes[i][j] = 255;
      } else {
        new_bytes[i][j] = 0;
      }
    }
  }
}

VM vm;
bool running = false;

WASM_EXPORT void wasm_init() {
  Program prog;
  parser_parse(&prog, "");
  vm = vm_init(prog);
  make_bayer();
}

WASM_EXPORT void wasm_accept(u8 c) {
  source[source_len++] = c;
  source[source_len] = 0; 
}

WASM_EXPORT void wasm_run() {
  source_len = 0;
  Program prog = {};
  tprintf("\n");
  if (parser_parse(&prog, (const char *)source)) {
    tprintf("Error!\n");
  } else {
    //tprintf("{}\n", prog.start);
    //tprintf("{}", prog);
    vm = vm_init(prog);
    running = true;
  }
}

int mx = -1, my = -1;

WASM_EXPORT void wasm_setmouse(float x, float y) {
  mx = x;
  my = y;
}

WASM_EXPORT void wasm_frame(float dt) {
  if (running) {
    if (vm_run_scr(&vm, bytes)) {
      running = false;
    }
    dither();

    wasm_render((u8*)new_bytes);
    vm.regs[Reg_Time] += dt;
  }
}