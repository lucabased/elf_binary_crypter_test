use super::InstructionModifier;
use iced_x86::{Code, Instruction};
use rand::Rng;

pub struct MovImm;

impl InstructionModifier for MovImm {
    fn apply(&self, instructions: &mut [Instruction], chance: u32) -> bool {
        let mut rng = rand::thread_rng();
        for instruction in instructions {
            if instruction.code() == Code::Mov_r32_imm32 {
                if rng.gen_range(1..=1000) <= chance {
                    println!("Applying MOV immediate substitution.");
                    instruction.set_immediate32(20);
                    return true;
                }
            }
        }
        false
    }
}
