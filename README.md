# Tisp (**T**yped L**isp**)

A Lisp-like programming language that is typed and compiled. It aims to 
support multiple processor architectures by being built upon LLVM. It takes
inspiration from programming languages like Rust, Lisp and Elixir.

## Current working example

A program to compute first 5 fibonacci numbers:

```lisp
(let first 0)
(let second 1)
(let fib)
(let n 0)

(while (< n 5)
    (let fib (+ first second))
    (let second first)
    (let first fib)

    (let n (+ n 1))
    (print fib)
)

```

## Features to build

- [x] Convert raw code into token stream
- [x] Convert token stream into Expression tree
- [x] Handle multiple levels of nested expressions
- [x] Have multiple (independent) expressions per file
- [x] Generate LLVM IR for the currently supported features
- [x] add CLI flag to emit llvm
- [x] add while loop
- [x] Declare variables
- [ ] add nested while loops
- [ ] Add types for function and variable declaration
- [ ] Define functions
- [ ] Support types in code

## Setup working environment

This project uses the following requirements:
* LLVM version 10.0 (https://llvm.org)
* Latest stable version Rust (https://rust-lang.org)

## How to build and run Tisp compiler

1. Download and install all dependencies.
2. Clone the repo and `cd` into it
3. Run `cargo build` to build it from source
4. Then you execute the compiled binary using the command below:

```bash
target/debug/tispc -h
```

### Compiling a Tisp source file

Write some valid Tisp code like the following:

```lisp
(let x 300)
(print "Hello world" (+ 2 (- 1 x)))
```

and save it in a file somewhere, for eg. `~/test.tp`. Now compile it using 
```bash
target/debug/tispc -i ~/test.tp
```

Then you can run the output generated using `lli` command, like so:
```bash
lli ~/output.ll
```

**NOTE**: Tisp doesn't generate executable binaries yet, it generates LLVM IR
in a file called `output.ll` that you can run with the `lli` command that comes
with your LLVM installation.
