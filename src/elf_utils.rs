//! This module provides utility functions for parsing ELF files.
use anyhow::{anyhow, Result};
use goblin::elf::{Elf, sym};

/// Finds the file offset and size of a specified function in an ELF binary.
///
/// # Arguments
///
/// * `elf` - A parsed ELF file object from the `goblin` crate.
/// * `function_name` - The name of the function to find (e.g., "main").
///
/// # Returns
///
/// A `Result` containing a tuple with the file offset and size of the function,
/// or an error if the function or `.text` section cannot be found.
pub fn find_function(elf: &Elf, function_name: &str) -> Result<(usize, usize)> {
    // Find the .text section, which contains the executable code.
    let text_section = elf
        .section_headers
        .iter()
        .find(|sh| {
            if let Some(Ok(name)) = elf.shdr_strtab.get(sh.sh_name) {
                name == ".text"
            } else {
                false
            }
        })
        .ok_or_else(|| anyhow!(".text section not found"))?;

    // Find the symbol for the specified function in the symbol table.
    let main_symbol = elf
        .syms
        .iter()
        .find(|sym| {
            if let Some(Ok(name)) = elf.strtab.get(sym.st_name) {
                name == function_name && sym.st_type() == sym::STT_FUNC
            } else {
                false
            }
        })
        .ok_or_else(|| anyhow!("'{}' symbol not found", function_name))?;

    // Calculate the file offset of the function.
    // This is done by finding the function's address relative to the .text section's
    // address, and then adding that to the .text section's file offset.
    let offset_in_section = main_symbol.st_value - text_section.sh_addr;
    let file_offset = text_section.sh_offset + offset_in_section;
    Ok((file_offset as usize, main_symbol.st_size as usize))
}
