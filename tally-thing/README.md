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

Technical PL Details and Identity Crisis
----------------------------------------

This is my first time writing something like this from scratch, so it's been
slow going - not helped by having to remember how to use python all the time.
I also didn't look at `resources.md` until halfway through...

I'm calling it a DSL because of its narrow scope,
but also because given the time and (lack of) expertise I have
I'm giving it very limited functionality:

  + Definitions at the top level only
	+ No sum types
	+ Only good for processing data handed to in in JSON form

It's inspired syntactically by
[_Lawvere_](https://github.com/jameshaydon/lawvere), but skips the nice type
system, and the sum types (which are lovely in _Lawvere_, but didn't seem
immediately useful for this appliction). The product types are backed by JSON
objects to try and take some work off my hands. Finally, since the application
is ballot-counting, I added a heavily built-in bag/multiset datatype, ideal for
storing all the ballots or various intermediate values. These are backed by
JSON arrays.

Unfortunately (and I can't believe I only realised this with 8 hours left to go)
this makes it effectively an incomplete, highly buggy implementation of
[_jq_](https://stedolan.github.io/jq/) (or rather: `jq -f`) with a subtly
different syntax (different declarations, way fewer `|`, a few fewer `.`,
a less flexible mapping syntax, ...). Oh well.

The upshot of this is that I know what I'll be doing on Monday:
I'll be installing _jq_ and seeing how easy it is to port my draft example
programs to _jq_ scripts. If it works nicely then I'll just have to put up with
all the extra `|` symbols in the syntax, and see if I can write a few
election-specific functions.

Running the Interpreter
-----------------------

Written in python, uses the [`pyparsing`](https://pypi.org/project/pyparsing/)
library. If you have it installed then:

```
...\tally-thing> python3 interpreter examples\test.tally
```

(adjust depending on OS, or how you prefer to run things in python environments)
