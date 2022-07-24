#!/bin/sh
gcc -c base.c -o base.o
gcc base.c -o base -Xlinker --hash-style=both
llvm-objcopy-6.0 --strip-sections base base.strip
aarch64-linux-gnu-gcc -c base.c -o base-aarch64.o
aarch64-linux-gnu-gcc base.c -o base-aarch64
mips64el-linux-gnuabi64-gcc -c base.c -o base-mips64el.o
mips64el-linux-gnuabi64-gcc base.c -o base-mips64el
loongarch64-unknown-linux-gnu-gcc -c base.c -o base-loongarch64.o
loongarch64-unknown-linux-gnu-gcc base.c -o base-loongarch64
