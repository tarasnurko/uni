#!/bin/bash

# Check if the script received an argument
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 path/to/rust_file.rs"
    exit 1
fi

# Get the path to the Rust file
RUST_FILE=$1

# Get the filename without extension
FILE_NAME=$(basename "$RUST_FILE" .rs)

# Print the filename
echo "Running Rust file: $FILE_NAME.rs"

# Compile, run, and remove the generated executable
rustc "$RUST_FILE" && ./"$FILE_NAME" && rm "$FILE_NAME"
