# Character
`P`

# Description
Follows this list of operations:
1. Pops the top two items as `name=stack[0]` and `arg_count=stack[1]`.
2. Traces the border of the procedure.
3. Moves right from the bottom left character of the border.

# Behaviour of name types
## String
Continues.

## Other types
Throws an error.

# Tracing the borders
1. The top border is traced until a `|` is reached.
2. The right border is traced until a `-` is reached.
3. The bottom and left borders are traced using the lengths found in the previous two steps.
4. The function border is now known.
