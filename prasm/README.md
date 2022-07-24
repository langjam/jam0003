# PRASM

Meet practicality of NASM assembler with beauty of presentations!

To use it you need:

- [NASM](https://www.nasm.us/)
- WYSIWYG presentation editor supporting OpenDocumentFormat like [LibreOffice](https://www.libreoffice.org)

See examples in [examples/](./examples/)!

- [examples/exit_42.odp](examples/exit_42.odp) simply calls `exit(42)` syscall
- [examples/hello_world.odp](examples/hello_world.odp) prints "hello, world!" to standard output
- [examples/pics.odp](examples/pics.odp) outputs one of the stored in presentation pictures to the path provided by command line arguments

```console
$ python prasm.py --run examples/exit_42.odp
$ echo $?
42
$ python prasm.py --run examples/hello_world.odp
hello, world!
$ python prasm.py --run examples/pics.odp cat-or-dog.jpg
$ xdg-open cat-or-dog.jpg
```

## How to use it?

To write in this language create presentation in Open Document Format (I tested with [LibreOffice](https://www.libreoffice.org/)). First page is considered title page and will not be included. The following pages create new NASM assembly file where:

- Slide title is considered to be content's label. Following pages with the same slide title create one code piece
- Text nodes in slide are meant for NASM assembly
- Shapes with text or images are included as binary blobs of data.

Additionaly language provides standard library known as [`prelude`](./prelude.nasm), which contains Linux system calls numbers and some common flags for file creation and IO. Review it's [source](./prelude.nasm) to learn what it provides!

After presentation creation transpile it into NASM assembly using `prasm.py` script. Assembling using nasm assembler is done by script.

```console
$ ./prasm.py examples/exit_42.odp
$ ./examples/exit_42.out
$ echo $?
42
```

## Why?

[Lang Jam3 (#0003)](https://github.com/langjam/jam0003) theme is __Beautiful Assembly__. Assembly is itself beautiful but we can improve it by including cat pictures! And one of the most common form of mixing text with pictures and color is presentations!

## Details

Please remember that it's my first time working with ODF documents and this knowladge is not sourced from it's documentation but from observing patterns in XML document and examples of [`odfpy`](https://github.com/eea/odfpy) library. This implies that this language can fall apart at any moment, since it requires output that matches what I observed in some presentation templates that I played with.

We iterate text nodes inside of presentation, except first page.

- If node is in title, then it's considered label, otherwise it's considered part of assembly program:
	- that is appended to current label as text if it's inside list
	- that is appended to current label as binary data if its inside shape
- If page with given title contains pictures, then they are included as binary data.

When images are included into presentation nodes reference them inside ODF document. ODF document is ZIP archive, so those references are paths inside archive.

ODF document stores presentation as XML text, determining meaning by tag names and special attributes like:

- `('urn:oasis:names:tc:opendocument:xmlns:drawing:1.0', 'name')` marking name of page
- `('urn:oasis:names:tc:opendocument:xmlns:presentation:1.0', 'class')` marking meaning of node (either `title` or `outline`)
- `('http://www.w3.org/1999/xlink', 'href')` referencing images inside archive
