use super::InstructionModifier;
use iced_x86::{Code, Instruction, OpKind};
use rand::Rng;

pub struct XorToMov;

impl InstructionModifier for XorToMov {
    fn apply(&self, instructions: &mut [Instruction], chance: u32) -> bool {
        let mut rng = rand::thread_rng();
        for instruction in instructions {
            if instruction.code() == Code::Xor_r32_rm32
                && instruction.op0_kind() == OpKind::Register
                && instruction.op1_kind() == OpKind::Register
                && instruction.op0_register() == instruction.op1_register()
            {
                if rng.gen_range(1..=1000) <= chance {
                    println!("Applying XOR to MOV substitution.");
                    instruction.set_code(Code::Mov_r32_imm32);
                    instruction.set_op1_kind(OpKind::Immediate32);
                    instruction.set_immediate32(0);
                    return true;
                }
            }
        }
        false
    }
}
