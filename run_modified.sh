#!/bin/bash
# This script compiles the dummy C code, runs the crypter,
# and then executes the modified binary.

echo "Compiling dummy.c with optimizations disabled and linking libcurl..."
gcc -O0 -o ../dummyelf ../dummy.c -lcurl
if [ $? -ne 0 ]; then
    echo "Failed to compile dummy.c. Aborting."
    exit 1
fi

echo "Running the crypter..."
cargo run
if [ $? -ne 0 ]; then
    echo "Crypter failed. Aborting."
    exit 1
fi

echo "Crypter ran successfully."

echo "--- File Hashes ---"
echo "Original binary:"
sha256sum ../dummyelf
echo "Modified binary:"
sha256sum ../dummyelf_modified
echo "-------------------"

echo "Now running the modified binary..."
chmod +x ../dummyelf_modified
../dummyelf_modified
