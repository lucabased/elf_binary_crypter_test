use anyhow::Result;
use iced_x86::{Decoder, DecoderOptions, Encoder, Instruction, Code};

pub fn modify_instructions(bytes: &[u8]) -> Result<Option<Vec<u8>>> {
    let mut decoder = Decoder::new(64, bytes, DecoderOptions::NONE);
    decoder.set_ip(0);
    let mut instructions: Vec<Instruction> = decoder.into_iter().collect();

    let mut modified = false;
    for instruction in &mut instructions {
        if instruction.code() == Code::Mov_r32_imm32 {
            instruction.set_immediate32(20);
            modified = true;
            println!("Modified a MOV instruction");
            break;
        }
    }

    if modified {
        let mut encoder = Encoder::new(64);
        for instruction in &instructions {
            encoder.encode(instruction, instruction.ip())?;
        }
        Ok(Some(encoder.take_buffer()))
    } else {
        Ok(None)
    }
}
