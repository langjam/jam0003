#ifndef COMMON_H
#define COMMON_H
#include "wasm.h"

#define CHECKOUT(x) if (x) { tprintf("LAST SEEN: {}:{}\n", __FILE__, __LINE__); return 1; }
//#define CHECKOUT(x) {(x);}

void putstr(Wasm_StreamId stream, const char *s);

struct Span {
  const char *s;
  uint l;
};

bool span_equal(Span a, Span b);

#endif
