use super::InstructionModifier;
use iced_x86::{Code, Instruction};

pub struct MovImm;

impl InstructionModifier for MovImm {
    fn apply(&self, instructions: &mut [Instruction]) -> bool {
        for instruction in instructions {
            if instruction.code() == Code::Mov_r32_imm32 {
                println!("Applying MOV immediate substitution.");
                instruction.set_immediate32(20);
                return true; // Stop after the first modification
            }
        }
        false
    }
}
