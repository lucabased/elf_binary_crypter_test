//! This module defines the `InstructionModifier` trait and declares the available
//! substitution modules.

use iced_x86::Instruction;

/// A trait for types that can modify a sequence of instructions.
pub trait InstructionModifier {
    /// Applies a modification to the given instructions.
    ///
    /// # Arguments
    ///
    /// * `instructions` - A mutable slice of `Instruction` objects to modify.
    ///
    /// # Returns
    ///
    /// `true` if a modification was applied, `false` otherwise.
    fn apply(&self, instructions: &mut [Instruction]) -> bool;
}

pub mod add_to_inc;
pub mod mov_imm;
pub mod test_to_cmp;
pub mod xor_to_mov;
