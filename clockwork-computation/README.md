# clockwork computation

A language for specifying clockwork. 

I've always thought clockwork assemblies were incredibly beautiful. Hundreds, maybe thousands of tiny, perfectly engineered
springs and gears, all moving in perfect synchronicity.

```
                    ^--- me explaining why this follows the theme
```

## website

There _should_ be a cool demo available at https://langjam3.kylepereira.me. Unless I wasn't very productive. 
In which case it'll just be, idk, some cute cats.

## installation

To install the interpreter, you'll need node.js. 
Clone the repository, run `npm ci` and then `npm start -- --help` should help you out with the 
available options.

Try `npm start -- example.cc` to try out some of the examples.

## so what's this about

If you haven't heard of the [Analytical Engine](https://en.wikipedia.org/wiki/Analytical_Engine),
it may surprise you to learn that it's possible to build a full, Turing complete, general purpose computer using just mechanical parts.

This little language gives you three components: Rods, gears, and springs. Now, if we want some digital logic, we need some definitions.
A rod can either be pushed (to the right) or pulled (to the left). We can use these like 0 and 1 in binary.
Let's say a pushed rod is 1 or true and a pulled rod is 0 or false.

Here's what a mechanical not gate looks like:

![A mechanical not gate showing two rods attached to either end of a central spinning rod.](./not-gate.png)

Clearly, if the input rod is pushed to the right, (so a one by our definition), the output rod is pulled to the left, (so a zero by our definition).
And, if the rod is pulled to the left, (a zero by our definition), the output rod is pushed to the right, (a one by our definiton).

And what do you know, that's the truth table for a NOT gate!

| X | !X |
| - | -- |
| 0 | 1  |
| 1 |  0 |

Here's how that's implemented in the language

```js
component not {  // define a resuable component not
    input x: rod {  // define x, a rod that is an input
        x -> my_gear {
            gearOffset: 0 // attach x to the first tooth of my_gear
        }
    }

    my_gear: gear { // define my_gear, a gear with two teeth, i.e a spinning rod.
        teeth: 2
        gear -> y {
            gearOffset: 1 // attach y to the second tooth of the gear
        }
    }

    output y: rod {} // define y, an output rod
}
```

Trying that out in the interpreter:

```js
> not(push)
Outputs:  [ 'pull' ]
After execution states were: 
Map(3) { 'x' => 'push', 'my_gear' => 1, 'y' => 'pull' }

> not(pull)
Outputs:  [ 'push' ]
After execution states were: 
Map(3) { 'x' => 'pull', 'my_gear' => 0, 'y' => 'push' }
```

Looks pretty good.

Here's an OR gate. Mechanically, it's just two rods that push a single rod.
If either of them pushes it, the ouput is true. Although, we also need a spring
to make the default state to pulled.

```js
component or {
    input x: rod {
        x -> joiner {
            rodAttachment: push // If x is pushed, it pushes joiner, but if x is pulled, joiner remains where it is
        }
    }

    input y: rod {
        y -> joiner {
            rodAttachment: push
        }
    }

    output joiner: rod {
        spring: pull // If nothing else pushes or pulls on joiner, a spring pulls it to the pull state. 
    }
}
```
Let's give that a run:

```js
> or(push, pull)
Outputs:  [ 'push' ]
After execution states were: 
Map(3) { 'x' => 'push', 'y' => 'pull', 'joiner' => 'push' }

> or(pull, pull)
Outputs:  [ 'pull' ]
After execution states were: 
Map(3) { 'x' => 'pull', 'y' => 'pull', 'joiner' => 'pull' }
```
Reasonable.

Now this is very exciting, because if we put those together we've got a NOR gate!

```js
component nor {
    input x: rod {}
    input y: rod {}
    
    use or(x, y) -> p
    use not(p) -> q

    output q: rod {}
}
```

And try that out:
```js
> nor(pull, pull)
Outputs:  [ 'push' ]
After execution states were: 
Map(4) { 'x' => 'pull', 'y' => 'pull', 'p' => 'pull', 'q' => 'push' }

> nor(pull, push)
Outputs:  [ 'pull' ]
After execution states were: 
Map(4) { 'x' => 'pull', 'y' => 'push', 'p' => 'push', 'q' => 'pull' }

> nor(push, push)
Outputs:  [ 'pull' ]
After execution states were: 
Map(4) { 'x' => 'push', 'y' => 'push', 'p' => 'push', 'q' => 'pull' }
```
Perfect!

As we all know, a NOR gate is a [universal gate](https://en.wikipedia.org/wiki/NOR_logic), meaning you can make _any_ logic circuit using just NOR gates.

Anyway. Say we want to do some maths. We _could_ go the boring way and make all the
regular logic gates, then a half adder, then a full adder, then put a bunch of them
together to make an eight-bit adder or whatever. 

But that's lame, we've got a whole bunch of gears! If we have a gear with say, 256 gears, we could have each tooth represent a number, so each 1/256th clockwise turn of the gear represents a number between 0 and 256.

And, because of my ~~lazy coding~~ smart thinking, these gears have a ratchet-y mechanism on them. That means two input gears can turn independently, even if they're 
attached to the same gear.

> We do actually let you (properly) connect multiple gears to each other like
> ```
> [input gear 1] --> [gear 2] --> [gear 3]
>                        \
>                         \-----> [gear 4]
>```
> However, because this is very much a one-way
> kinda deal, `[gear 3]` couldn't, for example, spin `[gear 1]` or another input gear.

So here's our magnificent analog adder: 

```js
component analog_eight_bit_adder {
    input a: gear {
        teeth: 256
        a -> main_rod {}
    }

    input b: gear {
        teeth: 256
        b -> main_rod {}
    }

    main_rod: gear {
        teeth: 512
        main_rod -> sum {} // because a & b spin clockwise, main_rod spins anti-clockwise
                           // As a result we need another `sum` gear so that our final output
                           // is clockwise as well.
    }

    output sum: gear {
        teeth: 512
    }
}
```

And here we go: 

```js
> analog_eight_bit_adder(69, 69)
Outputs:  [ 138 ]
After execution states were: 
Map(4) { 'a' => 69, 'b' => 69, 'main_rod' => 374, 'sum' => 138 }
```

Checks out.

Here's one last example: a digital to analog converter. We put in a digital binary number,
and we convert that to an analog spinning gear.

```js
component four_bit_digital_to_analog {
    input a3: rod {
        a3 -> a3_gear{
            gearOffset: 0
        }
    }
    input a2: rod {
        a2 -> a2_gear{
            gearOffset: 0
        }
    }
    input a1: rod {
        a1 -> a1_gear{
            gearOffset: 0
        }
    }
    input a0: rod {
        a0 -> a0_gear {
            gearOffset: 0
        }
    }

    a0_gear: gear {
        teeth: 2
        a0_gear -> main_rod{}
    }
    a1_gear: gear {
        teeth: 4
        a1_gear -> main_rod{}
    }
    a2_gear: gear {
        teeth: 8
        a2_gear -> main_rod{}
    }
    a3_gear: gear {
        teeth: 16
        a3_gear -> main_rod{}
    }

    main_rod: gear {
        teeth: 32
        main_rod -> final {}
    }

    output final: gear {
        teeth: 32
    }
}
```
Moving a rod causes a half revolution, and by using power of 2 sized gears, we turn the main rod
the right amount.

It's a bit wordy, but whatever, let's give it a spin (literally).

```js
> four_bit_digital_to_analog(push, push, pull, pull)
Outputs:  [ 12 ]
After execution states were:
Map(10) {
  'a3' => 'push',
  'a2' => 'push',
  'a1' => 'pull',
  'a0' => 'pull',
  'a3_gear' => 8,
  'a2_gear' => 4,
  'a1_gear' => 0,
  'a0_gear' => 0,
  'main_rod' => 20,
  'final' => 12
}
```
Yep! 1100 is indeed 12!


Anyway, that's my languge. I've waffled on long enough. Go enjoy your life.

## some specifics that no one will read

The grammar for the language is in lang.g4. It should be very straightforward.
The possible options within a part are:
    - teeth: <n> for gears
    - spring: <push | pull | none> for rods

The possible options within a connection are:
    - rodAttachment: <push | pull | attach>
    - gearOffset: <n>

