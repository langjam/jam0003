# Factory

A programming language with a flow similar to an assembly line, where each machine will take the input of another machine and apply a single transformation to it.

Possibly in the future with more time I will add some more features to this language, specifically more transformations which drive the functionality of the language. I ended up being a little late to the start of the jam, and ended up only having about 24 hours to do everything.

## Building / Running
The **Factory** interpreter runs on python, so all programs should be ran in this format:
```
python factory.py <assembly_line_file>
```

## Basics

### Hello World
One of the most basic programs is the hello world program. The program is fairly readable, however there are many specifics to get into later.

```
product machine print: print "Hello World!"

# Output:
Hello World!
```
A machine is defined in this format:
```
<tags> machine <name>(<dependencies>): <transformation> <transformation_inputs>
```

In **Factory**, machines are used to apply transformations to data. Similar to a typical function, they can accept accept multiple inputs, and only output a single value, however there still are some quirks. In the hello world example, note the `product` tag for the `print` machine, this lets the language know that it is the tail of the program, so everything should be ran based on the needs on that machine, and once the `product` machine has finished execution, the program has completed.

### Addition

In a more involved example where simple addition is performed, a second machine needs to be included because each machine can only apply a single transformation. In this case, the `print` machine depends on the output of the `add` machine, which means the `add` machine must be ran first. Following the execution of the `add` machine, the `print` machine uses `[0]` to access the output of the `add` machine.

`print` and `add` (as used after the :), are examples of *transformations*. The `print` transformation uses 1 parameter, which the `add` transformation uses 2.

```
machine add: add 1 1
product machine print(add): print [0]

# Output:
2
```
> Parenthesis are ommitted for machines with no dependencies.

### User Input

The examples so far have been boring, linear, programs, so let's explore some more "advanced" systems.
Here, note that the `print` machine also has the `input` tag, along with the `product` tag, the `input` tag signifies that this machine accepts user input from the command line. The command line input is accessed similar to how the outputs from machine dependencies are accessed, via `[]` with a number inside to denote which index of parameter.
```
input product machine print: print [0]
# Input:
Hello World
# Output:
Hello World
```

### Loops

In this example the third and final tag for a machine, `external` is used, which is a tag used to identify machines not run via the traditional dependency system. In this case, the dependency is ran through the `repeat` transformation, hence it needs to be external. The `repeat` transformation will run the `loop` machine 5 times, as specified by the `fib` machine.

The `nothing` transformation is what it sounds like, essentially a blank line, and is used for code organization other than a functional purpose.

```
external machine loop(): print 0
machine fib: repeat loop 5
product machine done(fib): nothing

# Output:
0
0
0
0
0
```

### Others
There are a couple other examples that I did not have enough time to document, but hopefully they are able to be understood using the documentation and examples already explained.

## Documentation

### Tags
`product`: denotes the final machine in an assembly line
`input`: denotes that a machine takes input from the command line as opposed to from another machine
`external`: denotes that a machine should be called only through transformations

### Transformations
`add <num1> <num2>`: add the two numbers and output the result
`subtract <num1> <num2>`: add the second number from the first and output the result
`equal <value1> <value2>`: check for equality between the values and output the result
`print <value>`: print the value to stdout
`passthrough <value>`: does nothing, only allows the use the use of a value without a transformation
`repeat <machine> <count>`: runs `machine` `count` times
`repeat_count <transformation> <args> <count> <initial>`: runs `transformation` `count` times with the given inputs, while keeping track of an internal state that can be accessed via `{0}` and `{-1]` (to access the previous state).
`if <condition> <machine>`: runs `machine` if `condition` is true
`nothing`: does literally nothing


