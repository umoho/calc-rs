# Calc

A simple calculator.

## Usage

If the name of the binary is `calculator`,

To calculate:

```bash
$ calculator -c <EXPRESSION>
# or
$ calculator --calc <EXPRESSION>
```

`<EXPRESSION>` is recommended to add quotes,
such as `'42'`, `"1 + 1"`

You can view token stream by

```bash
$ calculator -l <EXPRESSION>
# or
$ calculator --lexeme <EXPRESSION>
```

And tree by

```bash
$ calculator -t <EXPRESSION>
# or
$ calculator --tree <EXPRESSION>
```

## Build

No `dependencies` yet, just simply use

```bash
cargo build --release
```

to build the binary
