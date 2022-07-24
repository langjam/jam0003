#!/bin/sh
clang -target bpf -fno-addrsig -c bpf.c -o bpf.o.elf
