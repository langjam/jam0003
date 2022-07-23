#ifndef H_DEBUG_SRC
#define H_DEBUG_SRC

#include "wasm.h"
#include "common.h"

void putval(char c);
void putval(const char *s);
void putval(sint v);
void putval(uint v);
void putval(double v);
void putval(float v);
void putval(Span v);
void tprintf(const char* format);

template<typename T, typename... Targs>
void tprintf(const char* format, T value, Targs... Fargs) {
  for (; *format != '\0'; format++) {
    if (*format == '{' && format[1] == '{') {
      format += 1;
    } else if (*format == '}' && format[1] == '}') {
      format += 1;
    } else if (*format == '{') {
      putval(value);
      tprintf(format + 2, Fargs...); 
      return;
    } 
    putval(*format);
  }
}

#endif // H_DEBUG_SRC