# wasm-spreadsheet

This project implements a web-based spreadsheet application that uses Lisp as its expression language. The underlying spreadsheet engine is implemented in Rust, compiled to WebAssembly, and consumed by a TypeScript/React frontend. [Bazel](https://bazel.build/) is used to build Rust and bundle JavaScript.

## Features

- Bytecode Lisp compiler/interpreter, based on [this article](https://bernsteinbear.com/blog/bytecode-interpreters/).
- S-expression parser using [Nom](https://crates.io/crates/nom).
- Bazel build system allows us to build the entire app with one command, and express a dependency from the frontend JavaScript to the Rust WASM bundle.
- [Forthcoming] Use of immutable datastructures to enable easy undo/redo.
- Maintains a dependency graph between cells so that cells can be updated if any of their dependencies change.
