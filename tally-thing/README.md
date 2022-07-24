Tally Thing
===========

*Generalist DSL to count ballots for your thing!*

By Daniel Sherlock.

```
TODO: Provide example.
```

Background
----------

Around the world parliaments, councils, senates, things, and **assemblies**
use all manner of jazzy procedure to demoratically elect their members.
While they might be cleanly described in the abstract, they are often
hard to separate from the administrative structure of their country or region.
If you want to see how the election *actually* works, you can:

+ Hope all the details of the system fit in one unambiguous sentence.
  *This does happen sometimes.*
+ Read a long, wordy explanation of the algorithm, if available (e.g on Wikipedia).
  *In my experience your best bet, but often out of date or ambiguous or both or unavailable.*
+ Dig out the relevant elections law, and hope it has a translation.
  *English translations for international observers do sometimes exist.*
	*Disgustingly wordy, everything is confusing, try to define everything, have nonstandard terminology.*
+ Dig out a simulation written in some general-purpose programming language.
  *Often hard to tell what's going on for humans.*
	*It's difficult to directly compare different systems, because details are obscured by implementation.*
	
The hope is, with careful choice of syntax and the right "primitives",
it *might* be possible to have a DSL that avoids most of the implementation
boilerplate and leaves behind a short, **beautiful**, readable description
of the electoral process for a wide range of legislative bodies.

Prior Art
---------

There's a bunch, but usually they take the approch of implementing each family
of electoral system with a highly-parameterised program, providing a collection
of these, and hoping that what you want to do will be covered.

TODO: Link some here.

*Tally Thing* tries to give you an almost capable programming language,
with carefully chosen syntax and useful built-in functions
that act as "tallying primitives" to build your system to fit the
much more complex structures of real-life legislatures.

Running the Interpreter
-----------------------

Written in python, uses the [`pyparsing`](https://pypi.org/project/pyparsing/)
library. If you have it installed then:

```
...\tally-thing> python3 interpreter examples\test.tally
```

(adjust depending on OS, or how you prefer to run things in python environments)
