# Rust-GKAT WebAssembly Demo

This directory contains a WebAssembly version of the rust-gkat project, allowing you to run GKAT equivalence checking directly in your browser.

## Overview

GKAT (Guarded Kleene Algebra with Tests) is a formal system for reasoning about programs with conditionals and loops. This demo allows you to check the equivalence of GKAT expressions using two different algorithms:

1. **k1 (Symbolic derivative)** - Uses symbolic derivatives to check equivalence
2. **k2 (Thompson's construction)** - Uses Thompson's construction to build automata

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (for running the server)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) (for building the WebAssembly module)

### Building the WebAssembly Module

```bash
# From the web_wasm directory
wasm-pack build --target web
```

### Running the Server

```bash
# From the web_wasm directory
node server.js
```

Then open your browser to http://localhost:8080

## Usage

1. Enter two expressions to check for equivalence in the format `expr1 == expr2`
2. Click one of the "Check" buttons to verify equivalence using the selected algorithm
3. View the result, which will show whether the expressions are equivalent and how long the check took

## Syntax

The following syntax is supported:

- `p1, p2, p3, ...` - Basic program variables
- `b1, b2, b3, ...` - Boolean test variables
- `e1 ; e2` - Sequential composition
- `e1 + e2` - Choice (non-deterministic)
- `if b then e1 else e2` - Conditional
- `while b do e` - Loop

## Examples

- Simple identity: `p1 == p1`
- Sequence: `p1 ; p2 == p1 ; p2`
- While loop: `while b1 do p2 ; p3 == while b1 do p2 ; p3`
- If-Then-Else: `if b1 then p2 else p3 == if b1 then p2 else p3`
- Associativity: `p1 ; (p2 ; p3) == (p1 ; p2) ; p3`
- Distributivity: `p1 ; (p2 + p3) == (p1 ; p2) + (p1 ; p3)`

## Project Structure

- `index.html` - The main web interface
- `server.js` - A simple Node.js server for serving the application
- `src/` - Rust source code for the WebAssembly module
- `pkg/` - Compiled WebAssembly module (generated after building)

## Troubleshooting

If you encounter issues with the server already running on port 8080, you can modify the `PORT` constant in `server.js` to use a different port.

## License

This project is licensed under the same license as the main rust-gkat project.