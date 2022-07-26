# BASIS

Welcome to BASIS (Beginners ASsembly Instruction Set), a fantasy assembly language intended for beginners. BASIS takes a lot of inspiration for the MOS 6502 assembly language. The language has full, non-abbreviated, keywords where it makes sense to make it seem less intimidating as well as loops in some limited form.

Unfortunately, I wasn't fast enough to finish the project. So you can't really try it unless you finish the code yourself :-(
What's worse is that I'm already busy with projects that have higher priority on my list, so I may never come back to it :-(

### Overview

Just like in the 6502, the language comes with registers that can be used to access and modify memory, sections of which are reserved for special uses and commands that affect the registers or set flags and states in the interpreter which also acts as a fictional computer.

#### Registers
You start with 0 registers. This may seem peculiar, but the reason is to allow you to define the register names and their initial values yourself. You can have up to 5 registers. Registers are unsigned and only hold 8 bits each.

to set a register, you type `set name, value`. value is a decimal number. If you want to reference to a memory location, you have to prefix the value with a `#`.

You can also set multiple registers on one line like so: `set name1, name2, name3, value1, value2, value3`

#### Commands

Command usage for most commands is in the form of `command param1,param2,etc`. In most cases, there is no upper limit to the number of params supplied. In certains cases, the number of params changes the behaviour of the command.

* `store`, this command accepts only 2 parameters of which one must be a non reserved memory address and another must be a register. This takes the value of the register and stores it at the specified memory address.
* `load`, This command is actually three commands:
 * In the case where there are only registers as params, this copies the value of the first register into the other register(s)
 * If the number of register and values params are equal, each register is set to each value in order of appearance.
 * Else, each register's value is set to the only value param present.
* `add`, this command adds the sum of the parameters to the last register on that line.
* `sub`, this command substracts the difference of the parameters from the last register on that line.
* `inc`, increment the values of all registers on that line by 1
* `dec`, decrement the values of all registers on that line by 1
* `times`, this command multiplies by the result of the multiplication of the parameters the last register on that line.
* `div`, this command divides by the result of the division of the parameters the last register on that line.
* `left`, this command accepts only 2 parameters; one of which must be a register. It has the effect of left-shifting the last register's value on that line by the other parameter.
* `right`, this command accepts only 2 parameters; one of which must be a register. It has the effect of right-shifting the last register's value on that line by the other parameter.
* `or`, this command accepts only 2 parameters; one of which must be a register. It has the effect of doing a bitwise or on the last register's value on that line by the other parameter.
* `and`, this command accepts only 2 parameters; one of which must be a register. It has the effect of doing a bitwise and on the last register's value on that line by the other parameter.
* `comp`, this command accepts only 2 parameters. This compares the two values of the params and sets a special environment variable to not (!=), same (==), more (>), less (<), smore(>=) and sless (<=) depending on the result of the comparison
* `while`, this command needs 3 parameters: 2 values and 1 compare result name (see above). This is really just a macro for a combination of label, jump and compare.
* `forever`, this command accepts no parameters. It's a forever loop. It's really just a macro that uses a jump and a label.
* `loop`, this indicates where the forever/while loop ends. It accepts no params.
* `jump`, basically the assembly equivalent of goto. accepts labels only. No line numbers. Also accepts the names of comparison results (see `comp` above) to have conditional jumping.
* `screen` this is actually two commands. In both cases, the second argument is optional, in which case the sole argument is used as both parameters.
 * When it's the first command at the top of the file, it sets the window width and height. Default value is 0, so don't forget to put it there. It also has the effect of defining the memory location range used for pixels. Pixels are 8 bits and work exactly like on the gameboy advance.
 * When it is used later, it redefines pixel size. Default pixel size is 1x1.

#### Memory
Some memory addresses are reserved for special uses. These are:
 * #0 to #1 are combined as the stack pointer and point to the current instruction.
 * #2 is read-only and is the ascii value of whatever character the user typed on their keyboard save for the arrow keys which are represented thusly: left: 0, right: 1, up: 2 and down: 3
 * #3 to #n where n is the result of screen width times screen height + 3. Changing the values there alter the colour of the corresponding pixels.

The rest is free to use as you please. You have an almost infinite memory range, so don't worry about what limit I have set up in the interpreter. Not that you could try anyways ha!

