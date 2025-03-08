#!/bin/bash

# Build the project
echo "Building the project..."
cargo build

# Run tests
echo "Running tests..."
cargo test

# Test with examples
echo "Testing with examples..."

# Simple example
echo "Testing simple example..."
cargo run -- examples/simple.txt -k k1
cargo run -- examples/simple.txt -k k2

# Complex example
echo "Testing complex example..."
cargo run -- examples/complex.txt -k k1
cargo run -- examples/complex.txt -k k2

echo "All tests completed!"