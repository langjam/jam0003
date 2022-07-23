#ifndef H_WASM_SRC
#define H_WASM_SRC

#define WASM_EXPORT __attribute__((visibility("default"))) extern "C" 
#define WASM_IMPORT extern "C"

static_assert(sizeof(char) == 1, "sizeof(char) != 1");
typedef char i8;
typedef unsigned char u8;

static_assert(sizeof(short) == 2, "sizeof(short) != 2");
typedef short i16;
typedef unsigned short u16;

static_assert(sizeof(int) == 4, "sizeof(int) != 4");
typedef int i32;
typedef unsigned int u32;

typedef u32 uint;
typedef i32 sint;

enum Wasm_StreamId {
  WASM_STDOUT,
  WASM_STDERR
};

WASM_EXPORT void wasm_main();
WASM_IMPORT void wasm_putchar(Wasm_StreamId stream, int ch);
WASM_IMPORT void wasm_render(u8 *bytes);

#endif // H_WASM_SRC