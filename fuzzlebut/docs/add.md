# Character
`A`

# Description
Pops the top two items from the stack, adds them together, and pushes the result.

# Behaviour of left-hand types
## String
Formats the right side and appends it to the string.

## List
Adds the right side to the list.

## Int
If the right side is not an Int, then an error is thrown.

## Float
If the right side is not an Float, then an error is thrown.

## Other types
Throws an error.
