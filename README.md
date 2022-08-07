# Calc

A simple calculator.

## Usage

If the name of the binary is `calculator`,

To calculate:

```bash
$ calculator -c <EXPRESSION>
# or
$ calculator --calc <EXPRESSION>
# or simply input expressions in calculator's shell:
$ calculator
# and type 'quit' to exit
```

`<EXPRESSION>` is recommended to add quotes,
such as `'42'`, `"1 + 1"`.

You can view token stream by:

```bash
$ calculator -l <EXPRESSION>
# or
$ calculator --lexeme <EXPRESSION>
```

And show the tree by:

```bash
$ calculator -t <EXPRESSION>
# or
$ calculator --tree <EXPRESSION>
```

## Build

No `dependencies` yet, just simply use:

```bash
cargo build --release
```

to build the binary.

## FFI

Foreign function interface for the C programming language, has 4 functions.

For more information, please view `src/lib.rs`.
