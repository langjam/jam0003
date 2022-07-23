#ifndef COMMON_H
#define COMMON_H
#include "wasm.h"

void putstr(Wasm_StreamId stream, const char *s);

struct Span {
  const char *s;
  uint l;
};

bool span_equal(Span a, Span b);

#endif
