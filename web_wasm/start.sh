#!/bin/bash

# Default port
PORT=8080

# Check if a port was provided as an argument
if [ $# -eq 1 ]; then
  PORT=$1
fi

# Function to check if a port is in use
is_port_in_use() {
  if command -v lsof >/dev/null 2>&1; then
    lsof -i:"$1" >/dev/null 2>&1
    return $?
  elif command -v netstat >/dev/null 2>&1; then
    netstat -tuln | grep -q ":$1 "
    return $?
  else
    # If neither lsof nor netstat is available, assume port is free
    return 1
  fi
}

# Try to find an available port if the specified one is in use
if is_port_in_use "$PORT"; then
  echo "Port $PORT is already in use."

  # Try the next 10 ports
  for i in {1..10}; do
    NEW_PORT=$((PORT + i))
    if ! is_port_in_use "$NEW_PORT"; then
      echo "Using port $NEW_PORT instead."
      PORT=$NEW_PORT
      break
    fi
  done

  if [ "$PORT" -ne "$NEW_PORT" ]; then
    echo "Could not find an available port. Please specify a different port as an argument."
    exit 1
  fi
fi

# Update the port in server.js temporarily
sed -i.bak "s/const PORT = [0-9]\+;/const PORT = $PORT;/" server.js

echo "Starting Rust-GKAT WebAssembly Demo server on port $PORT..."
node server.js

# Restore the original server.js
mv server.js.bak server.js