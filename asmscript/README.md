# asmscript (asms)

asmscript (aka asms) is a JIT-compiled programming language made in 48h for the
third langjam hosted at [langjam/jam0003](https://github.com/langjam/jam0003).
The theme was *Beautiful Assembly*.

This is a low-level language, which attempts to make `x86_64` assembly look more
structured and easier to read. The scope of this language is very limited
because of the time constraints, but it is capable of doing some basic
computations.

Example program that prints integers from 1 to 100:

```
// A procedure named "main", which is the entry point
proc main {
    rax = 1;

    loop (rax <= 100) {
        << rax;   // Print value in rax to stdout
        << "\n";  // Print newline to stdout
        rax += 1;
    }
}
```

Here is a fragment of disassembly that asms compiler generated:

```
(start of "main" procedure)
0:  48 b8 01 00 00 00 00 00 00 00    movabs rax,0x1
a:  48 83 f8 64                      cmp    rax,0x64
e:  0f 8f 97 00 00 00                jg     0xab
(printing code omitted)
a2: 48 83 c0 01                      add    rax,0x1
a6: e9 5f ff ff ff                   jmp    0xa
ab: c3                               ret
```

## Building

This project targets only Linux/WSL and x86_64. You can compile it with `g++`,
like so:

```
g++ -std=c++17 -pedantic -Wall -Wextra -g -o asms Asms.cpp
```

(or run `./build.sh`, which does the same thing)

After building run `./asms` to see additional flags or `./asms FILE` to read,
compile and execute a script in `FILE`. Look for examples in
[Example](./Examples) directory.

## Details

More detailed documentation can be found at the
[GitHub page](https://iszn11.github.io/asmscript) for this project.
