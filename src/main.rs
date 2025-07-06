//! The main entry point for the crypter application.
mod crypter;
mod elf_utils;

use anyhow::Result;
use crypter::substitutions::{
    add_to_inc::AddToInc, mov_imm::MovImm, test_to_cmp::TestToCmp, xor_to_mov::XorToMov,
};
use crypter::Crypter;
use goblin::elf::Elf;
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    // Define the path to the binary we want to modify.
    let path = Path::new("../dummyelf");
    let mut buffer = fs::read(path)?;

    // Parse the ELF file to find the location of the 'main' function.
    let (main_func_offset, main_func_size) = {
        let elf = Elf::parse(&buffer)?;
        elf_utils::find_function(&elf, "main")?
    };

    println!(
        "Found 'main' function at offset {} with size {}",
        main_func_offset, main_func_size
    );

    // Get the machine code of the 'main' function.
    let main_func_bytes = &buffer[main_func_offset..main_func_offset + main_func_size];

    // Create a crypter and add the desired modifications.
    let crypter = Crypter::new()
        .with(Box::new(TestToCmp))
        .with(Box::new(AddToInc))
        .with(Box::new(XorToMov))
        .with(Box::new(MovImm));

    // Apply the modifications.
    if let Some(new_code) = crypter.apply_modifications(main_func_bytes)? {
        // If the modification was successful, write the new code back to the buffer.
        if new_code.len() <= main_func_size {
            buffer[main_func_offset..main_func_offset + new_code.len()].copy_from_slice(&new_code);
            // If the new code is smaller than the original, fill the rest with NOPs.
            for i in (main_func_offset + new_code.len())..(main_func_offset + main_func_size) {
                buffer[i] = 0x90;
            }
            // Write the modified buffer to a new file.
            fs::write("../dummyelf_modified", &buffer)?;
            println!("Modified binary written to ../dummyelf_modified");
        } else {
            println!("New code is larger than original function, cannot write.");
        }
    } else {
        println!("No suitable instruction found to modify.");
    }

    Ok(())
}
