#!/bin/bash
# This script makes the modified binary executable and runs it.

# First, run the crypter to generate the modified binary
cargo run

# Check if the crypter ran successfully
if [ $? -eq 0 ]; then
    echo "Crypter ran successfully. Now running the modified binary..."
    # Make the modified binary executable
    chmod +x ../dummyelf_modified

    # Run the modified binary
    ../dummyelf_modified
else
    echo "Crypter failed to run. Aborting."
fi
