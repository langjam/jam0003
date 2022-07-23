# Character
`S`

# Description
Pops the top two items from the stack, subtracts them, and pushes the result.

# Behaviour of left-hand types
## String
If right side is an Int N where `N>0`, removes at least N chars from the end of the String.

## List
If right side is an Int N where `N>0`, removes at least N items from the end of the List.

## Int/Float
Performs the operation.

## Other types
Throws an error.
