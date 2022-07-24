# Apilar

## About Apilar

Apilar is a language and alife system.

The Apilar language is a stack-based assembly language. There's a virtual
machine implementation of this.

A computer has memory and one or more processors. Each processor has its own
stack.

A replicator program can be written in Apilar that causes it:

- to copy itself

- to spawn a new processor for its copy

Processors can disappear if:

- they run out of memory

- they run an explicit "END" instruction.

Computers exist in a 2d qworld. The world is a grid of locations, and each
location has resources and potentially a computer. Programs can cause a
computer to split into multiple pieces, or merge multiple computers together.

Repeated splitting of its memory would make a computer very small. So a
computer can also grow its memory. To do so it needs to eat resources.

When a computer has no more processors, it dies and its resources (including
that bound in its memory) are released to the environment.

So now we have reproduction. Computers may also die.

To introduce a process of evolution, once every while a random address in a
random computer's memory is mutated.

This is usually not very useful, but sometimes a mutation may help a replicator
grow.

Apilar is inspired by
[Tierra](<https://en.wikipedia.org/wiki/Tierra_(computer_simulation)>).

## How to build and use

You need to have a recent stable Rust installed. Then:

```
cargo run --release
```

This creates a world, seeds it with a single hard-coded replicator,
and then lets it run. You can see the world evolve in the terminal.
Mysterious? It is mysterious to me as well.

## Documentation link
