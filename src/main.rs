use anyhow::Result;
use goblin::elf::{Elf, sym};
use std::fs;
use std::path::Path;
use iced_x86::{Decoder, DecoderOptions, Encoder, Instruction, Code};

fn main() -> Result<()> {
    let path = Path::new("../dummyelf");
    let mut buffer = fs::read(path)?;

    let (main_func_offset, main_func_size) = {
        let elf = Elf::parse(&buffer)?;
        let text_section = elf.section_headers.iter().find(|sh| {
            if let Some(Ok(name)) = elf.shdr_strtab.get(sh.sh_name) {
                name == ".text"
            } else {
                false
            }
        }).ok_or_else(|| anyhow::anyhow!(".text section not found"))?;

        let main_symbol = elf.syms.iter().find(|sym| {
            if let Some(Ok(name)) = elf.strtab.get(sym.st_name) {
                name == "main" && sym.st_type() == sym::STT_FUNC
            } else {
                false
            }
        }).ok_or_else(|| anyhow::anyhow!("'main' symbol not found"))?;

        let offset_in_section = main_symbol.st_value - text_section.sh_addr;
        let file_offset = text_section.sh_offset + offset_in_section;
        (file_offset as usize, main_symbol.st_size as usize)
    };

    println!("Found 'main' function at offset {} with size {}", main_func_offset, main_func_size);

    let main_func_bytes = &buffer[main_func_offset..main_func_offset + main_func_size];

    // Disassemble the function's instructions
    let mut decoder = Decoder::new(64, main_func_bytes, DecoderOptions::NONE);
    decoder.set_ip(0); // We don't need the real IP for this
    let mut instructions: Vec<Instruction> = decoder.into_iter().collect();

    // Find and modify an instruction
    let mut modified = false;
    for instruction in &mut instructions {
        if instruction.code() == Code::Mov_r32_imm32 {
            // Example: Change `mov r32, 10` to `mov r32, 20`
            instruction.set_immediate32(20);
            modified = true;
            println!("Modified a MOV instruction");
            break;
        }
    }

    if modified {
        // Assemble the modified instructions
        let mut encoder = Encoder::new(64);
        for instruction in &instructions {
            encoder.encode(instruction, instruction.ip())?;
        }
        let new_code = encoder.take_buffer();

        // Overwrite the original function with the new code
        if new_code.len() <= main_func_size {
            buffer[main_func_offset..main_func_offset + new_code.len()].copy_from_slice(&new_code);
            // If the new code is smaller, fill the rest with NOPs
            for i in (main_func_offset + new_code.len())..(main_func_offset + main_func_size) {
                buffer[i] = 0x90;
            }
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
