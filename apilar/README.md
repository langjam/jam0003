# Apilar

## About Apilar

Apilar is a language and alife system. It was created for [langjam
3](https://github.com/langjam/jam0003).

The theme for langjam 3 is "Beautiful Assembly".

Apilar is an stack-machine assembly language that supports self-assembly: a
replicator Apilar program can be written that makes copies of itself in memory
and in space. New programs then evolve.q

The Apilar language is a stack-based assembly language. There's a virtual
machine implementation of this.

Here's a little example:

```
N2
N3
ADD
```

This adds two numbers to the stack, `2` and `3`, and then adds them together.
In the end the stack contains `5`.

A computer has memory and one or more processors. Each processor has its own
stack.

A replicator program can be written in Apilar that causes it:

- to copy itself

- to spawn a new processor for its copy

Processors can disappear if:

- they run out of memory

- they run an explicit "END" instruction.

Computers exist in a 2d world. The world is a grid of locations, and each
location has resources and potentially a computer. Programs can cause a
computer to split into two pieces (into a neighbor), or merge a neighbor into
itself.

Repeated splitting of its memory would make a computer very small. So a
computer can also grow its memory. To do so it needs to eat resources.

When a computer has no more processors, it dies and its resources (including
that bound in its memory) are released to the environment.

So now we have reproduction. Computers may also die.

To introduce a process of evolution, once every while a random address in a
random computer's memory is mutated.

This is usually not very useful, but sometimes a mutation may help a replicator
grow.

Apilar is inspired by the famous alife simulation
[Tierra](<https://en.wikipedia.org/wiki/Tierra_(computer_simulation)>).

## How to build and use

You need to have a recent stable Rust installed. Then:

```
cargo run --release -- run
```

This creates a world, seeds it with a single hard-coded replicator, and then
lets it run. You can see the world evolve in the terminal, so you please make
your terminal window big enough.

Sometimes the worlds are duds and growth stops. Sometimes growth is slow.
Sometimes growth accelerates; it all depends on what mutations occurred. I did
one longer run run for a few hours, and I came back to a world somehow filled
with processors but very few computers; and it looked dead.

If you want to see a new world, just `ctrl-C` to stop and run again.

Is your terminal confused after you break out of the simulation? For me typing
the `reset` command helps.

There are also a lot of command line arguments to configure the simulation, see:

```
cargo run -- run -h
```

What is going on in these worlds? It's a bit of a mystery without more careful
analysis.

To do some analysis, you can cause dump files to be created regularly during a
run using the `--dump true` flag. You can then disassemble the memory in
particular locations using commands like:

```
cargo run -- disassemble apilar-dump2.cbor 35 10
```

## sample code

You can find a few sample programs in the `sample_code` directory.

## langjam 3 documentation link

More documentation may appear here in the next few days:

https://github.com/faassen/apilar
