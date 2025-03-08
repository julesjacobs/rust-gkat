# Rust-GKAT WebAssembly

This is a WebAssembly-compatible version of the rust-gkat project. It provides a pure Rust implementation of Binary Decision Diagrams (BDDs) for use in web browsers.

## Features

- Pure Rust implementation of BDDs (no C dependencies)
- WebAssembly compatibility
- Support for both kernel methods:
  - K1: Symbolic derivative method
  - K2: Symbolic Thompson's construction

## Project Structure

- `src/`: Rust source code
  - `syntax/`: Data structures and traits
    - `gkat.rs`: Common interfaces
    - `gkat_pure_bdd.rs`: Pure Rust BDD implementation
  - `kernel1/`: Symbolic derivative method
  - `kernel2/`: Symbolic Thompson's construction
  - `parsing/`: Parser for the input language
  - `lib.rs`: WebAssembly bindings
  - `main.rs`: Command-line interface
- `pkg/`: WebAssembly output (generated)
- `examples/`: Example input files
- `tests/`: Test files
- `index.html`: Web interface
- `server.js`: Simple Node.js server for testing

## Building

To build the WebAssembly package, you'll need [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/):

```bash
wasm-pack build --target web
```

This will generate a `pkg` directory containing the WebAssembly module and JavaScript bindings.

## Running

To run the web interface locally:

```bash
node server.js
```

Then open a browser and navigate to http://localhost:8080/.

## Testing

Run the tests with:

```bash
cargo test
```

## Input Format

The input format is a simple language for expressing guarded Kleene algebra with tests (GKAT) expressions. Examples:

- `p1 ; p2 == p1 ; p2 ? true`: Simple sequence
- `while p1 do (p2 ; p3) == while p1 do (p2 ; p3) ? true`: While loop
- `if p1 then p2 else p3 == if p1 then p2 else p3 ? true`: If-then-else
- `p1 ; (p2 + p3) == (p1 ; p2) + (p1 ; p3) ? true`: Distributivity

## Implementation Details

This implementation replaces the C-based CUDD library and SAT solver with a pure Rust implementation of Binary Decision Diagrams (BDDs). This makes it compatible with WebAssembly, allowing it to run in web browsers.