#include "wasm.h"

void putstr(Wasm_StreamId stream, const char *s) {
  while (*s) {
    wasm_putchar(stream, *s++);
  }
}

void wasm_main() {
  putstr(WASM_STDOUT, "Hello world from wasm!\nNew line.");
}