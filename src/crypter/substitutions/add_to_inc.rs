use super::InstructionModifier;
use iced_x86::{Code, Instruction, OpKind};

pub struct AddToInc;

impl InstructionModifier for AddToInc {
    fn apply(&self, instructions: &mut [Instruction]) -> bool {
        for instruction in instructions {
            // Check for `ADD r32, 1`
            if instruction.code() == Code::Add_rm32_imm8
                && instruction.op0_kind() == OpKind::Register
                && instruction.op1_kind() == OpKind::Immediate8
                && instruction.immediate8() == 1
            {
                println!("Applying ADD to INC substitution.");
                // Replace with `INC r32`
                instruction.set_code(Code::Inc_r32);
                // `INC` has only one operand, so we remove the second one.
                // While iced-x86 doesn't have a direct `remove_operand` method,
                // changing the code to INC effectively makes it a single-operand instruction.
                // The encoder will handle this correctly.
                return true;
            }
        }
        false
    }
}
