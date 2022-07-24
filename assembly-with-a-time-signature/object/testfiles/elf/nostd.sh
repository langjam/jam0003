#!/bin/sh
clang -c nostd.c --target=mipsel-unknown-elf -mabi=o32
mipsel-unknown-elf-ld --entry=0 nostd.o -o nostd
