# Requirements

You need to install the following two dependencies to run the simulator:

* `portmidi` for the client side of the midi connection
* `fluidsynth` for the server side of the midi connection
  In the future you will be allowed to change fluidsynth to a different midi synthesizer.

Both of these can be installed through your package manager

Next you can run it with cargo in order of advancedness:
```bash
cargo run -- examples/test.asm
cargo run -- examples/instrument.asm examples/std.asm
cargo run -- examples/chord.asm examples/std.asm
cargo run -- examples/little_star.asm
cargo run -- examples/feel.asm examples/std.asm
cargo run -- examples/piano-man.asm examples/std.asm
```

# Idea
Assembly With A Time Signature (AWATS) is an assembly language and cpu architecture to make sound! The emulator's backend 
is midi-based. With the available instructions you can make songs. Some example programs are provided in the examples 
directory.

AWATS uses a custom emulator, custom parser (by using a custom-made parser framework from before the jam) and can
output object files in ELF format (with a custom ELF architecture). It can also link these object files 
together using a custom linker. However, due to time constraints it cannot output an executable elf file. After linking
the emulator is directly invoked.

The language is turing complete. Conditional branches and loops are supported. However, instead of only having first-class
bytes, this assembly language and architecture also has first-class support for musical notes a through g.

A note can be played using the play or play-for instruction. Notes are batched up until a beat instruction is
encountered and then all notes are played at once. 

Timing of beats can be influenced by the time signature (using the time instruction) and the bpm instruction.

# Architecture

This instruction set is a hybrid CISC/RISC architecture, with
variable length instructions but some quite low level operations
such as separate load/store instructions instead of an almighty mov.

The memory space is 64 bits large but practically bounded
by your machine

# registers

* `br` (bar counter, 64 bit)
* `bt` (beat counter, 64 bit, resets after each bar)
* `pc` (program counter, 64 bits)
* `sp` (stack pointer)
* `ir` (instrument register, 64 bits and basically a track)
* `oc` octave register
* `a#{a-g}` (accidentals of a through f. Negative numbers are flat, positive sharp)
* `r{0-7}` general purpose registers (64 bit)
* `n{0-7}` general purpose note registers (1 note)

Note that values on the stack can either be:

* numbers (64 bit)
* notes (#a through #f)

This is not how many CPUs work. There the stack can have
only one value. However, we didn't want this language to
be completely esoteric and to be sort-of usable. We
concluded that allowing this is a lot more usable.

# instructions

`%a` means a placeholder register a
`{b}` means a placeholder for a number b or a note b.
Note that if an instruction expects a note, a number is converted to a note by using the remainder modulo 7
and if an instruction expecting a number gets a note, the note is converted to a number from 0-12 where the current
accidental is taken into account. If the accidentals cause the note to roll over (#f --> #a) the octave register is
incremented (and respectively decremented on underflow)

* `call lbl` pushes
* `ret`
* `jmp lbl` jumps to another place in the program
* `bpm %a` (change beats per minute)
* `bpm {a}` (change beats per minute with immediate)
* `time %a %b` (change the time signature to %a/%b)
* `time {a} {b}` (change the time signature to a/b with immediate)
* `beat` finish a beat (same as `wait #1`)
* `wait {a}/%a` desugars to repeated `beat` commands
* `play-for [n] {a}` play a note n for a beats on the instrument currently in the instrument pointer at the current octave
  and accidental
* `play-for [n] %a` play a note n for a beats ^
* `play-for %n {a}` play a note n for a beats ^
* `play-for %n %a` play a note n for a beats ^
* `play %n` play a note for 1 beat
* `flat [n]` alias for `sub a[n] #1`
* `sharp [n]` alias for `add a[n] #1`
* `push %r`
* `pop %r`
* `lbl:` create a label. Labels can be used in place of immediates and represent an address in memory there
* `.[n]:` create a local label scoped within the previous real label
* `add %dst %src1 %src2/{a}` add registers. Notes are seen as numbers from 1-12 respecting current accidentals modulo 12
  but adjusts the octave register on overflow or underflow
* `sub %dst %src1 %src2/{a}` subtract registers (see add)
* `inc, dec` (aliases of add and sub 1)
* `j{g,l,ge,le,ne,eq,} %a %b lbl` jumps if a op b is true
* `mov %dst %src|{a}` move registers around
* `st %a %b/{b}` store %a at address %b
* `ld %a %b/{b}` load from address %b to %a
* `define [instrument name]` in the instrument section creates a new instrument id with a label. `ld %ir instr_lbl` sets
  that as the current instrument.
* `lir lbl` "load instrument register". Alias for `st %ir lbl`
* `go lbl` like call, but starts a parallel execution track (to have multiple pieces of music playing alongside one another)
* `exit` stop the current parallel music track


# example program

```
.instrument
p1: define piano
p2: define flute
p3: define piano

.song

verse:
    play #a
    beat
    play #b
    beat
    play #c
    beat
    play #d
    beat
    play #e
    beat
    play #f

    ret

chorus:
    ret

main:
    bpm 120
    time 3 4

    lir p1
    call verse

    ; from here, all as played are sharp by 1
    sharp #a

    call chorus

    ; from here, all as played are flat by 1.
    ; However, they were already sharp so this
    ; effectively restores the #a
    flat #a

    call verse
    call verse
    call chorus

    stop
    
```


