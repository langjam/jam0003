# arrow-asm
the name is perfect and shall never be changed

## TODO

- [ ] find a single flaw whatsoever (very challenging!)

## Docs
### Generating compilation database
```sh
$ ./tools/gencompdb.py
```
### Building documentation
```sh
$ cargo install mdbook
$ cd $PROJ_ROOT/doc
$ mdbook build
```
### Running `.aasm`
```sh
$ bazel run //:aasm -- $THEFILE
```
