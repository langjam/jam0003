# StackGrid

- Stacks are grid columns. The first is actually where you write the instructions and every other column is a possible stack.

- Stacks default to B, making one line programs in the A column

Examples:

- Hello world

```
A                                                                         B
Jump if stack-empty (cell number to jump to, column letter of the stack)  33
Output ascii (column letter of the stack)                                 100
Jump (cell number of print loop)                                          108
```

- Chat bot: with questions and answers in other stack

- Take an input program add it to a column, and execute that column

- Fizzbuzz

- Comments

TODO:

- Maybe the instructions can themselves be laid out in the grid as numbers (but then has to be one after the other, operator and operands on different lines.)

- Think about how quotes in the CSV should be handled

- Write Docs

- Deploy docs to GitHub Pages
