use anyhow::{Result, anyhow};
use goblin::elf::{Elf, sym};

pub fn find_main_function(elf: &Elf, buffer: &[u8]) -> Result<(usize, usize)> {
    let text_section = elf.section_headers.iter().find(|sh| {
        if let Some(Ok(name)) = elf.shdr_strtab.get(sh.sh_name) {
            name == ".text"
        } else {
            false
        }
    }).ok_or_else(|| anyhow!(".text section not found"))?;

    let main_symbol = elf.syms.iter().find(|sym| {
        if let Some(Ok(name)) = elf.strtab.get(sym.st_name) {
            name == "main" && sym.st_type() == sym::STT_FUNC
        } else {
            false
        }
    }).ok_or_else(|| anyhow!("'main' symbol not found"))?;

    let offset_in_section = main_symbol.st_value - text_section.sh_addr;
    let file_offset = text_section.sh_offset + offset_in_section;
    Ok((file_offset as usize, main_symbol.st_size as usize))
}
