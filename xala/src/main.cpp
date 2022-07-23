#include "wasm.h"

void putstr(Wasm_StreamId stream, const char *s) {
  while (*s) {
    wasm_putchar(stream, *s++);
  }
}

u8 bytes[256][256] = {};

WASM_EXPORT void wasm_main() {
  putstr(WASM_STDOUT, "Hello world!");
  for (int i = 0; i < 256; ++i) {
    for (int j = 0; j < 256; ++j) {
      bytes[j][i] = (j/32+i/32)%2;
    }
  }
  wasm_render((u8*)bytes);
}