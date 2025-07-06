use super::InstructionModifier;
use iced_x86::{Code, Instruction, OpKind};
use rand::Rng;

pub struct AddToInc;

impl InstructionModifier for AddToInc {
    fn apply(&self, instructions: &mut [Instruction], chance: u32) -> bool {
        let mut rng = rand::thread_rng();
        for instruction in instructions {
            if instruction.code() == Code::Add_rm32_imm8
                && instruction.op0_kind() == OpKind::Register
                && instruction.op1_kind() == OpKind::Immediate8
                && instruction.immediate8() == 1
            {
                if rng.gen_range(1..=1000) <= chance {
                    println!("Applying ADD to INC substitution.");
                    instruction.set_code(Code::Inc_r32);
                    return true;
                }
            }
        }
        false
    }
}
