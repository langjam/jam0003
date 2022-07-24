#include <stdio.h>

static void preinit() {
    printf("preinit\n");
}
static void (*const preinit_array[])()
    __attribute__((section(".preinit_array"), aligned(sizeof(void *))))
    = { preinit };

static void init() {
    printf("init\n");
}
static void (*const init_array[])()
    __attribute__((section(".init_array"), aligned(sizeof(void *))))
    = { init };

static void fini() {
    printf("fini\n");
}
static void (*const fini_array[])()
    __attribute__((section(".fini_array"), aligned(sizeof(void *))))
    = { fini };

static void ctor() {
    printf("ctor\n");
}
static void (*const ctors[])()
    __attribute__((section(".ctors"), aligned(sizeof(void *))))
    = { ctor };

static void dtor() {
    printf("dtor\n");
}
static void (*const dtors[])()
    __attribute__((section(".dtors"), aligned(sizeof(void *))))
    = { dtor };

int main() {
    return 0;
}
