#!/bin/bash

# Initialize a new Rust project
cargo new my-rust-project

# Navigate into the project directory
cd my-rust-project

# Create the necessary directory structure
mkdir -p src

# Create a README.md file
echo "# My Rust Project" > README.md
echo "This is a Rust project." >> README.md

# Create a main.rs file
echo "fn main() {" > src/main.rs
echo "    println!(\"Hello, world!\");" >> src/main.rs
echo "}" >> src/main.rs

# Build the project
cargo build

# Print a success message
echo "Rust project initialized successfully."