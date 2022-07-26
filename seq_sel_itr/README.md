It is said that all that is needed to create a Turing complete computer is Sequence, Selection, and Iteration. I've attempted to make a "Beautiful Assembly" language with only the commands SEQ, SEL, and ITR.

The following are the rules for the commands:

```
SEQ 5 // Just returns an i64 with the value of 5
SEQ 2 4 3 // Does what's on line 2, then what is on line 4, then what is on line 3.and adds what each branch returns.

SEL -3 5 // Skips next in SEQ if current value is less than 5
SEL -2 5 // Skips next in SEQ if current value is less than 5
SEL -1 5 // Skips next in SEQ if current value is less than or equal to 5
SEL 0 5 // Skips next in SEQ if current value is equal to 5
SEL 1 5 // Skips next in SEQ if current value is greater than or equal to 5
SEL 2 5 // Skips next in SEQ if current value is greater than to 5
SEL 3 5 // Skips next in SEQ if current value is greater than to 5

ITR 2 6 // Line 2 will be repeated 6 times
```


Unfortunately, I wasn't able to get my implemenation working by the Lang Jam's deadline.