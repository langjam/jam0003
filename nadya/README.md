# Nadya

## Running examples

All of the examples can be found in their own folder in the `examples/`
directory.

There are several examples you can run. Nadya is build in Rust, and *should*
work by just compiling (Might require stable 1.61 or above). To run an example:

`cargo run -- --example rain`

There available examples are:

- `addition`: Add 1 and 2 together
- `file`: Multiply each number in a file by 9
- `rain` Lots of falling numbers!
- `maze` Changing directions a little


### Execution notes

- To exit the program, press `q`.
- If the code crashes, you'll likely need to close your terminal.
- Make sure your terminal is tall enough to see the full program running.

## Language Details

Nadya is a simple language that can currently add and multiply numbers together.
Variables move around the program with the `O` symbol, and will wait at
intersections until another variable gets there to "merge" with it.

This ~~game~~ language is meant to simulate items moving around some assembly
line. The inspiration for this language is factory games like Satisfactory or
Factorio. The name comes from my friend Aidan (backwards Nadia) who I taught to
play Factorio and has since greatly outskilled me.

## More Language Details

- A file `input.txt` will be loaded if there is any `F` character around the
  program. This file must contain integers on each line, and these will be the
  numbers that spawn from this place.

## Credits

Thanks to my brother for making bigger examples 30 minutes before the jam ended
~~and I had not yet started on my submission~~

## Documentation

Documentation can be found here:
https://github.com/AngelOnFira/nadya#documentation

This will likely link out to a website and other documentation.
