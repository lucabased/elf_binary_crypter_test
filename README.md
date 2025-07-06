# Linux Crypter Test

This project is a simple demonstration of a "crypter" for Linux ELF binaries. A crypter is a tool that modifies an executable file in order to change its signature, which can be used to bypass antivirus detection.


## Resources for further reading
https://www.zscaler.com/blogs/security-research/demystifying-crypter-used-emotet-qbot-and-dridex

## How it Works

The project is split into three main parts:

1.  **`dummy.c`**: A simple C program that serves as the target for our crypter. It can be modified to include various functionalities to test the crypter's capabilities.

2.  **`linux_crypter_test`**: A Rust program that acts as the crypter. It uses a modular, builder-pattern based architecture to apply a series of instruction modifications with a configurable probability.
    *   **`main.rs`**: The main entry point that orchestrates the process.
    *   **`elf_utils.rs`**: Handles parsing the ELF binary to find functions.
    *   **`crypter/`**: A module that contains the core crypter logic.
        *   **`mod.rs`**: Implements a `Crypter` builder that chains together different modification strategies and allows setting a probability for the modifications.
        *   **`substitutions/`**: A module containing various instruction substitution techniques. Each technique is implemented as a `struct` that adheres to the `InstructionModifier` trait.
            *   `test_to_cmp.rs`: Replaces `TEST reg, reg` with `CMP reg, 0`.
            *   `xor_to_mov.rs`: Replaces `XOR reg, reg` with `MOV reg, 0`.
            *   `add_to_inc.rs`: Replaces `ADD reg, 1` with `INC reg`.
            *   `mov_imm.rs`: Changes the immediate value in a `MOV` instruction.

3.  **`run_modified.sh`**: A shell script that automates the entire process. It:
    *   Compiles `dummy.c` into an ELF binary called `dummyelf`.
    *   Runs the `linux_crypter_test` program to create the modified binary.
    *   Prints the SHA256 hashes of both the original and modified binaries to show that the file has been altered.
    *   Makes the modified binary executable.
    *   Runs the modified binary.

## How to Use

1.  Make sure you have `gcc` and `cargo` installed.
2.  Run the `run_modified.sh` script:
    ```bash
    ./run_modified.sh
    ```

This will compile the C code, run the crypter, and execute the modified binary.

## Disclaimer

This project is for educational purposes only. The techniques used here are very basic and are not intended to be used for malicious purposes.
