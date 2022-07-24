# Xala
### Introduction
Welcome to Xala (pronounced shala, from SHader Assembly LAnguage), an assembly language designed to make beautiful monochrome animations.

XALA was developed by [skejeton](https://github.com/skejeton) and [mrms](https://github.com/marekmaskarinec) with some miscellaneous assistance from [Doigt](https://github.com/RealDoigt).

### Installing
Clone this repository then type this in the terminal:
```shell
cd xala
./build.sh
```
If you get an `clang: error: unable to execute command: Executable "wasm-ld" doesn't exist!`error message, try updating your packages or try this temporary [fix discussed in the clang issue](https://github.com/actions/virtual-environments/issues/5366).

### Running Scripts

You may now play around with the language in your browser. To do that you'll first need to install a web server and use that. If you have python installed, you can use the command `python -m http.server`, otherwise if you don't know what to install, we recommend to install php-cli if it's not already on your system then you can type these commands:
```shell
cd build
php -S localhost:8080
```

Once the server is running, don't close the terminal window! Open your favourite browser and type `localhost:8080` in the addess bar and press enter. If all went well, you should see something like this appear on screen:

![](https://i.imgur.com/AQ68y2E.png)

We'll get into how to use the sandbox and what each command does next.

### Overview

Before starting, you should learn how to interact with the environment. Luckily, it's very easy to navigate. You type the code in the text box on your left and when you're done, you click on the button labeled `Run` which is centered atop the text box. You'll see the result in the box to your right. In the box below, error messages will appear if your code doesn't work properly.

#### Registers
As with other assembly languages, Xala has registers, of which there are 7. Those are:

* `%X`, floating point value of the X coordinate. The value range is between 0 and 1 inclusively.
* `%Y`, floating point value of the Y coordinate. The value range is between 0 and 1 inclusively.
* `%RET`, a return value. It is not implemented yet. But may be implemented in a future version.
* `%OUT` is the floating point value of the opacity of the colour on the screen. Since Xala only works in monochrome, this is achieved with dithering. The value range is between 0 and 1, with 1 being completely light and 0 being completely dark. Using the default colours, this would mean that 1 is completely white and 0 completely blue, with values in between being a mix of both colours.
* `%TIME` is the number of seconds program has been running for.
* `%MEMORY ` is a special register which holds more than one value at a time. But only one may be accessed at once. It returns the value stored at the memory address that `%BASE` points to.
* `%BASE` holds the current memory address of the value that the `%MEMORY` register will return.

#### Arithmetic
Xala supports some basic mathematical operations with the following keywords which act as operators:

* `ADD`, for addition
* `SUB`, for substraction
* `MUL`, for multiplication
* `DIV`, for division
* `MOD`, for modulo

These operators require two parameters and can be used in two ways; using either the polish notation like so: `operator param1 param2` or the more traditional infix notation as such: `param1 operator param2`. Here is a more concrete example:
```asm
  %X ADD %Y
  ; is equivalent to
  ADD %X %Y
```
You must use the `INTO` keyword which to store the result in a register. If you store the result in the `%OUT` register, this will update the appearance of the screen.

### Code Samples
Here are some animations that have already been written for your enjoyment:
```asm
0 INTO %BASE
%TIME INTO %MEMORY
%MEMORY MUL %MEMORY INTO %MEMORY
%MEMORY ADD %X MOD 1.0 INTO %OUT
```
