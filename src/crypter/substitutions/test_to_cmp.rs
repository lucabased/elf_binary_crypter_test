use super::InstructionModifier;
use iced_x86::{Code, Instruction, OpKind};
use rand::Rng;

pub struct TestToCmp;

impl InstructionModifier for TestToCmp {
    fn apply(&self, instructions: &mut [Instruction], chance: u32) -> bool {
        let mut rng = rand::thread_rng();
        for instruction in instructions {
            if instruction.code() == Code::Test_rm32_r32
                && instruction.op0_kind() == OpKind::Register
                && instruction.op1_kind() == OpKind::Register
                && instruction.op0_register() == instruction.op1_register()
            {
                if rng.gen_range(1..=1000) <= chance {
                    println!("Applying TEST to CMP substitution.");
                    instruction.set_code(Code::Cmp_rm32_imm8);
                    instruction.set_op1_kind(OpKind::Immediate8);
                    instruction.set_immediate8(0);
                    return true;
                }
            }
        }
        false
    }
}
