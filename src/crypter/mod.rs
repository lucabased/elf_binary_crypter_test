//! This module defines the `Crypter` builder for applying instruction modifications.

use anyhow::Result;
use iced_x86::{Decoder, DecoderOptions, Encoder, Instruction};

pub mod substitutions;
use substitutions::InstructionModifier;

/// A builder for applying a series of instruction modifications.
pub struct Crypter {
    modifiers: Vec<Box<dyn InstructionModifier>>,
}

impl Crypter {
    /// Creates a new, empty `Crypter` builder.
    pub fn new() -> Self {
        Self {
            modifiers: Vec::new(),
        }
    }

    /// Adds a modifier to the crypter's pipeline.
    ///
    /// # Arguments
    ///
    /// * `modifier` - An object that implements the `InstructionModifier` trait.
    pub fn with(mut self, modifier: Box<dyn InstructionModifier>) -> Self {
        self.modifiers.push(modifier);
        self
    }

    /// Applies the configured modifications to the given byte slice.
    ///
    /// # Arguments
    ///
    /// * `bytes` - A slice of bytes representing the function's machine code.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Option<Vec<u8>>`. If a modification was applied,
    /// it returns `Some` with the new machine code. Otherwise, it returns `None`.
    pub fn apply_modifications(&self, bytes: &[u8]) -> Result<Option<Vec<u8>>> {
        let mut decoder = Decoder::new(64, bytes, DecoderOptions::NONE);
        decoder.set_ip(0);
        let mut instructions: Vec<Instruction> = decoder.into_iter().collect();

        let mut modified = false;
        for modifier in &self.modifiers {
            if modifier.apply(&mut instructions) {
                modified = true;
                break; // Apply only the first successful modification
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
}
