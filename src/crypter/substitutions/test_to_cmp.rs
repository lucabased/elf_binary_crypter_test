use super::InstructionModifier;
use iced_x86::{Code, Instruction, OpKind};

pub struct TestToCmp;

impl InstructionModifier for TestToCmp {
    fn apply(&self, instructions: &mut [Instruction]) -> bool {
        for instruction in instructions {
            // Check for `TEST r32, r32`
            if instruction.code() == Code::Test_rm32_r32
                && instruction.op0_kind() == OpKind::Register
                && instruction.op1_kind() == OpKind::Register
                && instruction.op0_register() == instruction.op1_register()
            {
                println!("Applying TEST to CMP substitution.");
                // Replace with `CMP r32, 0`
                instruction.set_code(Code::Cmp_rm32_imm8);
                instruction.set_op1_kind(OpKind::Immediate8);
                instruction.set_immediate8(0);
                return true;
            }
        }
        false
    }
}
