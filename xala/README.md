# Xala
## Introduction
Welcome to Xala (pronounced shala, from SHader Assembly LAnguage), an assembly language designed to make beautiful monochrome animations.

XALA was developed by [skejeton](https://github.com/skejeton) and [Marek Maskarinec](https://github.com/marekmaskarinec) with some miscellaneous assistance from [Doigt](https://github.com/RealDoigt).

Instead of installing and running scripts locally, you can try the language at the online playground here: https://mrms.cz/xala/playground

You can find more documentation here: https://mrms.cz/xala/

## Installing
Clone this repository then type this in the terminal:
```shell
cd xala
./build.sh
```
If you get an `clang: error: unable to execute command: Executable "wasm-ld" doesn't exist!` error message, try updating your packages or try this temporary [fix discussed in the clang issue](https://github.com/actions/virtual-environments/issues/5366).

## Running Scripts

You may now play around with the language in your browser. To do that you'll first need to install a web server and use that. If you have python installed, you can use the command `python -m http.server`, otherwise if you don't know what to install, we recommend to install php-cli if it's not already on your system then you can type these commands:
```shell
cd build
php -S localhost:8080
```

Once the server is running, don't close the terminal window! Open your favourite browser and type `localhost:8080` in the addess bar and press enter. If all went well, you should see something like this appear on screen:

![](https://i.imgur.com/AQ68y2E.png)

We'll get into how to use the sandbox and what each command does next.

## Overview

Before starting, you should learn how to interact with the environment. Luckily, it's very easy to navigate. You type the code in the text box on your left and when you're done, you click on the button labeled `Run` which is centered atop the text box. You'll see the result in the box to your right. In the box below, error messages will appear if your code doesn't work properly.

### Registers
As with other assembly languages, Xala has registers, of which there are 7. Those are:

* `%X`, floating point value of the X coordinate. The value range is between 0 and 1 inclusively.
* `%Y`, floating point value of the Y coordinate. The value range is between 0 and 1 inclusively. 0 is at the bottom of screen rather than the top.
* `%RET`, the return value of a function.
* `%OUT` is the floating point value of the opacity of the colour on the screen. Since Xala only works in monochrome, this is achieved with dithering. The value range is between 0 and 1, with 1 being completely light and 0 being completely dark. Using the default colours, this would mean that 1 is completely white and 0 completely blue, with values in between being a mix of both colours.
* `%TIME` is the number of seconds program has been running for.
* `%MEMORY ` is a special register which holds more than one value at a time. But only one may be accessed at once. It returns the value stored at the memory address that `%BASE` points to.
* `%BASE` holds the current memory address of the value that the `%MEMORY` register will return.

### Arithmetic
Xala supports some basic mathematical operations with the following keywords which act as operators:

* `ADD`, for addition
* `SUB`, for substraction
* `MUL`, for multiplication
* `DIV`, for division
* `MOD`, for modulo
* `COS`, for cosine
* `SIN`, for sine

With the exception of `COS` and `SIN`, these operators require two parameters and can be used in two ways; using either the polish notation like so: `operator param1 param2` or the more traditional infix notation as such: `param1 operator param2`. Here is a more concrete example:
```asm
  %X ADD %Y
  ; is equivalent to
  ADD %X %Y
```
You must use the `INTO` keyword to store the result in a register. If you store the result in the `%OUT` register, this will update the appearance of the screen.

### Functions
Now time to spice things up with some functions. You can declare a function by first typing out its name, however, it must be prefixed with an `@`. If you're using functions, you should have a `@MAIN` function so that the interpreter may know where to start.

Parameters are called like registers in a numbered fashion starting at 0.

A function ends with the RET keyword and is called in another function by typing out its name without the `@`. The `RET` keyword shouldn't be confused with the `%RET` keyword. the former is what ends the function and the latter is the register where the return value is stored. It's important not to mix them up!

Heres a few examples of functions:
```asm
@SQRT
  %0 POW 0.5 INTO %RET
  RET

@MAIN
  SQRT %X INTO %OUT
```
```asm
@SQUARED
  %0 MUL %0 INTO %RET
  RET

@MAIN
  %X SQUARED INTO %OUT
```

### Code Samples
Here are some animations that have already been written for your enjoyment:
```asm
%X ADD %Y ADD %TIME MOD 0.5 INTO %OUT ; adds x and y outputs into the screen
```
```asm
0 INTO %BASE
%TIME INTO %MEMORY
%MEMORY MUL %MEMORY INTO %MEMORY
%MEMORY ADD %X MOD 1.0 INTO %OUT
```
```asm
@V2MAK
  %0 INTO %BASE
  %1 INTO %MEMORY
  %BASE ADD 1 INTO %BASE
  %2 INTO %MEMORY
  %0 INTO %RET
  RET

@V2ADD
  %1 INTO %BASE
  %MEMORY INTO %A
  %0 INTO %BASE
  %A ADD %MEMORY INTO %MEMORY

  %1 ADD 1 INTO %BASE
  %MEMORY INTO %A
  %0 ADD 1 INTO %BASE
  %A ADD %MEMORY INTO %MEMORY

  %0 INTO %RET
  RET

@V2MAG
  %0 INTO %BASE
  %MEMORY MUL %MEMORY INTO %A
  %0 ADD 1 INTO %BASE
  %MEMORY MUL %MEMORY INTO %MEMORY
  %A ADD %MEMORY INTO %RET
  RET

@MAIN
  SIN %TIME INTO %A
  COS %TIME INTO %MEMORY
  V2MAK 4 %A %MEMORY INTO %A
  V2MAK 2 %X %Y INTO %A
  V2MAK 0 -0.5 -0.5 V2ADD 2 V2ADD 4 V2MAG INTO %OUT
  ```
