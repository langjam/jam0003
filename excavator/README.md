# The Excavator language
The Excavator graphics programming language.

An easy-to-use drawing programming language targeted at making our world better.

With Excavator, one can generate black n' white imagery in a simple way that would make
even beginners instantly comfortable.

Its easy-of-read syntax makes it the best tool in the market for what it does,
which is create small cat images gradually using our command-assembly strategy.

### WhAT dO WE Do WItH AsSemBLy and BeAUTiFul?
We use the latest and greatest mathematical operators (+ and *)
to generate drawing instructions (like left, right, up and down)
which at the end are run gradually by the user to create a hopefully
beautiful drawing.

### Features
* [*] Running interpreter!
* [*] Interactive run screen!
* [~] Correct execution
* [ ] Control flow instructions
* [ ] Runtime errors

### Build 
Dependencies:

* *nix system (tested only on Linux)
* CMake 3.16>=
* NCurses5>=
* GNU make
* GCC (Any version would probably work)

To download dependencies on Ubuntu:

```
$ sudo apt install g++-11 cmake build-essential libncurses5-dev
```

You must be in the project directory for this part.

```
cmake .
make
```

### Quick start
Wow you actually got here! Congrats!

Now, can you see a file in you current directory called "jam"? That's our programming language!

Now you can run a file using "./jam <name-of-file>", try it on the file `example.excv`!

Now press the spacebar many times.
When the execution finishes, press `q`.

You can press `q` anytime you want to exit the program.

You can find examples in the `examples/` subdirectory.

Wanna try writing a file yourself? Head over to the next part!

### Syntax

Now we're here...

#### Keywords

In assembly tradition, keywords aren't case-sensitive,
so you can write both `is` and `iS`.

#### Variables
Variable names are similar to variable names in php:
They have a `$` prefix so for a variable named `var` we'll
do `$what`.

That's fun and all, but how do we define them?

`$var is 5`

#### Values
You have two types of values: commands and numbers.

Numbers are written as numbers, and there are four commands specifying where to go next:
`<` meaning left, `>` meaning right, `^` meaning up and `,` meaning down.

All of the values can be stored in variables.

#### Operations
You can multiply numbers by a command to assemble a new command that when run
will run multiple times. You can also chain commands: `< + ^ * 8 + < * 2 * $var`.

Basically you can do most of what you think is possible using the `*` and `+` commands,
and if it isn't, you'll get an error.


### How it works
The assembled instructions are converted into commands that are then fed into the screen
writer, which shows them one after the other.

The writer draws in the middle of the screen.

For example, a `<` command will move the writer left and draw on it.

`< * 4 + ^ * 4 + > * 4 + , * 4` would draw a rectangle.
