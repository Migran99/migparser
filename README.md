# migparser
This crates implements a very simple argument parser inspired by the Python one. It allows adding arguments of different types (int, uint, bool, string, float) and customize the behaviour with different options (necessary, store-true, store-false, ...).

## Example
https://github.com/Migran99/migparser/blob/aef2b1ca64f8f2db398c91f5e339fd6b4972cd92/src/bin/demo.rs?plain=1#L1-L39


Run it
```bash
cargo run miguel -na 1 -f 1 -oa 2.3
```
