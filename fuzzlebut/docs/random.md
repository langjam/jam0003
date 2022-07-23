# Character
## Random Int
`@`

## Random Float
`&`

# Description
Pops the top two items where `min=stack[0]` and `max=stack[1]` then generates a random number between those two numbers.
If both min and max are zero, then a completely random number is generated.

# Behaviour of min types
## Int
If max is not an Int, then throws an error.

## Float
If max is not an Float, then throws an error.

## Other types
Throws an error.
