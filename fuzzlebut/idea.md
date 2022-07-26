# Main idea
Textual 2D stack-based language

# Types
## Int
i64

## Float
f64

## Bool
u1

## String
Rust's String type

## Object
A Rust HashMap of String:Value pairs.

# Functions
Functions start execution from the top-left of the body and go until it hits an edge, causing a panic, or until the function returns.

## Parameters
The function defines how many parameters are passed to the function and they are popped off the stack and pushed to a new stack in the same order.
The function then executes with a clean stack and the passed parameters.

## Return values
Only the top item, if there is one, is returned. The top item is popped from the function's stack and pushed to the previous stack.

# Instructions
## Helpful info
### Popped item indices
When popping multiple items from the stack, they are referred to with the top of the stack being index 0 and the successive indices being the index from the top down.

## Errors
### Not enought instruction arguments
Does not run the program and tells the user why before runtime.

### Incorrect instruction types
Does not run the program and tells the user why before runtime.

## Double quote
Creates a String, puts all characters into the String until another double quote, then pushes the String. Allows these 6 escape sequences: `\n`, `\"`, `\r`, `\t`, `\\`, and `\0`.

## Add, Sub, Mul, Div operations (A S M D)
Uses the top two numbers and performs the operation on them with `left=stack[top]` and `right=stack[top-1]`. Works on Ints and Floats.

### Additional info for Add
Can concat strings, and concat a formatted object.
Can append items to a list.
Acts as logical OR for two Bool values

### Additional info for Sub
Can remove characters from a string; `right.type==Int` and `right>0`.
Can pops the specified number of items off the end of a list; `right.type==Int` and `right>0`.
Can remove fields from objects; `right.type==String`.

### Additional info for Div
Div pushes the result then remainder.

### Additional info for Mul
Acts as logical AND for two Bool values

## 'O'
The O instruction creates a new object with no fields

## 'F'
The F instruction pops the top 3 items with the source object being `popped[0]`, the name being `popped[1]`, and the data being `popped[2]` and creates a new field on the object with the name and data.

## Change direction (^ v < >)
Changes the parse direction.

## '|'
Vertical wire; NOP
Errors if it leads to a horizontal wire.

## '-'
Horizontal wire; NOP
Errors if it leads to a vertical wire.

## '+'
Wire cross; NOP

## 'N'
Negates the top item.

### Float, Int
`value*=-1`

### Bool
Logical NOT

## '%'
Pops one item and ignores it; deletes the top item.

## 'C'
Changes a Float to Int, truncating the fractional part, or Int to Float

## '!'
The ! instruction formats the top item and prints it. If the top item is a string, then it just prints it.

## '#'
Same as '!', but adds a newline.

## 'P'
Start defining procedure with the name `popped[0]` with type String and the argument count `popped[1]` with type Int>=0.
Requires wires to enclose the function body

## 'G'
Greater than with `left=popped[0]`, `right=popped[1]`
Only works with Int, Float, Bool.

## 'L'
Less than with `left=popped[0]`, `right=popped[1]`
Only works with Int, Float, Bool.

## 'E'
Equality with `left=popped[0]`, `right=popped[1]`

## 'T'
Pushes `true`

## '~'
Exit the program

## 'V'
Creates a list (sometimes called Vector or Vec<T>, so this kinda makes sense).

## 'R'
Rotates the stack so the top of the stack is now at the bottom.

## 'r'
Same as R, but the other way.

## '*'
Run the procedure with name `popped[0]`

## 'U'
Read a line from the user and push it to the stack.

## 's'
Swap the top two values on the stack.

## regex"[0-9]+" (Ints)
Pushes an Int with the specified value.

## regex"[0-9]+\.[0-9]+" (Floats)
Pushes a Float with the specified value.

## ' ' (space)
Does nothing. Acts like a wire cross.

## 'd'
Duplicates the top item and pushes it.

## 'c'
Takes the next character in the program and pushes it to the stack.

## '.'
Splits the top string into a list of chars and pushes it.

## 'p'
Pops the last item in a collection and pushes it to the stack.

### Strings
Pops the last char.

### Lists
Pops the last item.

### Other
Does nothing.

## 'B'
Branch if the top item is true.
True turns CCW, and false turns CW.
If the top value is not `Value::Bool(true)`, then it is automatically false.

## 'b'
Same as 'B', but true turns CW and false turns CCW.

## 'l'
Length of the top item.

### String
The amount of characters in the string.

### List
The amount of items.

### Object
The amount of fields.

### Others
Pushes `Value::Int(0)`

## '?'
Debug print the stack.

## '@'
Random Integer between `popped[0]` and `popped[1]`.
If both popped ints are zero, then `min=i64::MIN` and `max=i64::MAX`.

## '&'
Random Floats between `popped[0]` and `popped[1]`.
If both popped floats are zero, then `min=f64::MIN` and `max=f64::MAX`.

## '['
Rotate a list at the top of the stack to the left.

## ']'
Rotate a list at the top of the stack to the right.

## '$'
Changes an int to a char and vice versa.
Throws an error if the int is an invalid char.

## Other, unmentioned characters
Does nothing. Acts like a wire cross.
