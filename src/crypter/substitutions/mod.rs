//! This module defines the `InstructionModifier` trait and declares the available
//! substitution modules.

use iced_x86::Instruction;
use rand::Rng;

/// A trait for types that can modify a sequence of instructions.
pub trait InstructionModifier {
    /// Applies a modification to the given instructions with a certain probability.
    ///
    /// # Arguments
    ///
    /// * `instructions` - A mutable slice of `Instruction` objects to modify.
    /// * `chance` - The probability (from 1 to 1000) that a modification will be applied.
    ///
    /// # Returns
    ///
    /// `true` if a modification was applied, `false` otherwise.
    fn apply(&self, instructions: &mut [Instruction], chance: u32) -> bool;
}

pub mod add_to_inc;
pub mod mov_imm;
pub mod test_to_cmp;
pub mod xor_to_mov;
