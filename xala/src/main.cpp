#include "vmintrin.h"
#include "wasm.h"
#include "debug.h"
#include "parser.h"

bool span_equal(Span a, Span b) {
  if (a.l != b.l) {
    return false;
  }
  while (a.l && *a.s && *b.s) {
    if (*a.s++ != *b.s++) {
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
u8 source[1 << 16];
uint source_len;

bool in_bounds(int x, int y) {
  return x >= 0 && y >= 0 && x < 256 && y < 256;
}

void dither() {
  const u8 thresh[16][16] = {  
    {     0, 191,  48, 239,  12, 203,  60, 251,   3, 194,  51, 242,  15, 206,  63, 254  }, 
    {   127,  64, 175, 112, 139,  76, 187, 124, 130,  67, 178, 115, 142,  79, 190, 127  },
    {    32, 223,  16, 207,  44, 235,  28, 219,  35, 226,  19, 210,  47, 238,  31, 222  },
    {   159,  96, 143,  80, 171, 108, 155,  92, 162,  99, 146,  83, 174, 111, 158,  95  },
    {     8, 199,  56, 247,   4, 195,  52, 243,  11, 202,  59, 250,   7, 198,  55, 246  },
    {   135,  72, 183, 120, 131,  68, 179, 116, 138,  75, 186, 123, 134,  71, 182, 119  },
    {    40, 231,  24, 215,  36, 227,  20, 211,  43, 234,  27, 218,  39, 230,  23, 214  },
    {   167, 104, 151,  88, 163, 100, 147,  84, 170, 107, 154,  91, 166, 103, 150,  87  },
    {     2, 193,  50, 241,  14, 205,  62, 253,   1, 192,  49, 240,  13, 204,  61, 252  },
    {   129,  66, 177, 114, 141,  78, 189, 126, 128,  65, 176, 113, 140,  77, 188, 125  },
    {    34, 225,  18, 209,  46, 237,  30, 221,  33, 224,  17, 208,  45, 236,  29, 220  },
    {   161,  98, 145,  82, 173, 110, 157,  94, 160,  97, 144,  81, 172, 109, 156,  93  },
    {    10, 201,  58, 249,   6, 197,  54, 245,   9, 200,  57, 248,   5, 196,  53, 244  },
    {   137,  74, 185, 122, 133,  70, 181, 118, 136,  73, 184, 121, 132,  69, 180, 117  },
    {    42, 233,  26, 217,  38, 229,  22, 213,  41, 232,  25, 216,  37, 228,  21, 212  },
    {   169, 106, 153,  90, 165, 102, 149,  86, 168, 105, 152,  89, 164, 101, 148,  85  }
  };

  for (int i = 0; i < 256; ++i) {
    for (int j = 0; j < 256; ++j) {
      uint col = i % 16;
      uint row = j % 16;

      if (bytes[i][j] > thresh[col][row]) {
        new_bytes[i][j] = 255;
      } else {
        new_bytes[i][j] = 0;
      }
    }
  }

  for (int i = 0; i < 256; ++i) {
    for (int j = 0; j < 256; ++j) {
      bytes[i][j] = new_bytes[i][j];
    }
  }
}

WASM_EXPORT void wasm_main() {
  const char *source_code = 
    "%X ADD %Y MOD 2 MUL 255 INTO %OUT; add numbers bruh\n";
  Program prog;
  if (parser_parse(&prog, source_code)) {
    tprintf("Error!");
  } else {
    tprintf("{}", prog);
  }

  for (int i = 0; i < 256; ++i) {
    for (int j = 0; j < 256; ++j) {
      bytes[j][i] = i;
    }
  }
  dither();
  wasm_render((u8*)bytes);
}