# Tisp (**T**yped L**isp**)

A Lisp-like programming language that is typed and compiled. It aims to 
support multiple processor architectures by being built upon LLVM. It takes
inspiration from programming languages like Rust, Lisp and Elixir.

Here's an example function declaration in Tisp:
```lisp
(def (add_one_and_double num:i32):i32
    (num)
    (then (+ 1))
    (then (* 2))
)
```

## Features to build

- [x] Convert raw code into token stream
- [x] Convert token stream into Expression tree
- [x] Handle multiple levels of nested expressions
- [x] Have multiple (independent) expressions per file
- [x] Generate LLVM IR for the currently supported features
- [x] add CLI flag to emit llvm
- [ ] Add types for function and variable declaration
- [ ] Define functions
- [ ] Support types in code
- [x] Declare variables
