# Introduction
Core is a simple yet powerful assembly-like programming language. The language design is inspired by assembly language which means when writing program, there are many operations involved.  
  
  
At the beginning of execution, a 64 byte memory is initialized and can be accessed by the program todo different operations.  
```
memory = [None] * 64
```
  
This memory can be accessed by address, ranging from 0 to 63.  
```
memory[3] <-- fourth item in memory
```
  
## Syntax
- In core, there are 4 registers: OP, D0, D1, D2
- **OP** is the operation register, operation is denoted by number. Currently there are 10 operation supported.
  
number | operation
------ | ---------
0      | [store](#store)
1      | [write](#write)
2      | [move](#move)
3      | [plus](#math-operation)
4      | [minus](#math-operation)
5      | [multi](#math-operation)
6      | [div](#math-operation)
7      | [equal](#equal)
8      | [jump](#jump)
9      | [exit](#exit)
  
- D0 to D3 is the data register, each operation depends on and handles these four register differently.  
- After all register have been set, do `call` command to execute the operation.  
- Comment can be done using `//`.  
- There are only 'int' and 'str' in this language.  
```
5      <-- int
"core" <-- str
```

## Usage
Simply pass the file containing the core language to the interpreter.
```
python core.py hello.core
```
    
### STORE
Store a byte of data into memory address  
  
D0 - memory address  
D1 - data to be stored  
```
// store the number 5 to memory 0
op 0
d0 0
d1 5
call
```
```
// store the string "hi" to memory 10
op 0
d0 10
d1 "hi"
call
// of course, string is not stored like how assembly does it.
// this is to make the language simpler and more beautiful.
```
  
### WRITE
Write data from memory to STDOUT  
  
D0 - memory address
```
// store "hello world" to memory 0
op 0
d0 0
d1 "hello world"
call

// write "hello world" from memory 0
op 1 
d0 0
call

// <STDOUT> hello world
// write can only write from memory, this means when something needs to be printed,
// it needs to be stored first.
```
  
### MOVE
Move data from an address to another address  
D0 - from address  
D1 - to address
```
// move data from memory address 1 to 0
op 2
d0 1
d1 0
call
```
  
### MATH OPERATION
Perform math operation between two address and store them to another address  
D0 - first number  
D1 - second number  
D2 - result address
```
// memory[0] = 2
// memory[1] = 3
// add two number from memory address 0 and 1
op 3
d0 0
d1 1
d2 2
call

// the result of this operation will be stored onto memory address 2
// memory[2]  # 5
```
  
### EQUAL
Compares two data and if true, jump to another line of program  
D0 - first data  
D1 - second data  
D2 - number of which line to jump to (if true)
```
memory[0] = 9
memory[1] = 9
memory[2] = "true"
memory[3] = "false"

 1: // compare data at address 0 and 1, if true, jumps to line 18
 2: op 7
 3: d0 0
 4: d1 1
 5: d2 18
 6: call
 7:
 8: // write "false" (this will be skipped if true)
 9: op 1
10: d0 3
11: call
12:
13: // exit, prevent from entering the true block
14: op 9
15: d0 0
16: call
17:
18: // write "true"
19: op 1
20: d0 2
21: call
```
  
### JUMP
Jump to another line of program, this is beneficial for loops.  
D0 - number of which line to jump to
```
// jump to line 1
op 8
d0 1
call
```
  
### EXIT
Exit the execution with exit-code  
D0 - exitcode (number)
```
// exit with 0 code
op 9
d0 0
call
```
  
## Appreciation
I would like to thank the organizer for this amazing jam. I really had fun developing this language and researching the assembly language. This is my first time entering the jam, and I look forward for the next one.
