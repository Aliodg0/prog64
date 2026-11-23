// Stockage de 5 instruction dans un u64 = u55

use crate::instruction::{Instruction, InstructionError};

/// Nombre maximal d'instructions dans un programme.
pub const MAX_INSTRUCTIONS: usize = 5;

/// Nombre de bits utilisés par une instruction.
const INSTRUCTION_BITS: usize = 11;

/// Masque permettant d'extraire 11 bits.
const INSTRUCTION_MASK: u64 = 0x7FF;

/// Masque des 55 bits utilisés par le programme.
const PROGRAM_MASK: u64 = (1u64 << 55) - 1;

/// Erreurs possibles lors de la manipulation d'un programme.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgramError {
    InstructionIndexOutOfRange(usize),
    ProgramBitsOutOfRange(u64),
    InstructionEncodingFailed(InstructionError),
}

impl From<InstructionError> for ProgramError {
    fn from(error: InstructionError) -> Self {
        ProgramError::InstructionEncodingFailed(error)
    }
}

/// Un programme LGP stocké dans un entier de 64 bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Program {
    bits: u64,
}

impl Program {
    /// Construit un programme à partir de cinq instructions.
    pub fn new(instructions: [Instruction; MAX_INSTRUCTIONS]) -> Result<Self, ProgramError> {
        let mut bits = 0u64;

        for (index, instruction) in instructions.iter().enumerate() {
            let encoded = instruction.encode()?;

            bits |= (encoded as u64) << (INSTRUCTION_BITS * index);
        }

        Ok(Self { bits })
    }

    /// Construit un programme depuis sa représentation binaire.
    pub fn from_bits(bits: u64) -> Result<Self, ProgramError> {
        if bits & !PROGRAM_MASK != 0 {
            return Err(ProgramError::ProgramBitsOutOfRange(bits));
        }

        Ok(Self { bits })
    }

    /// Retourne la représentation binaire du programme.
    pub fn bits(&self) -> u64 {
        self.bits
    }

    /// Récupère une instruction à partir de son indice.
    pub fn instruction(&self, index: usize) -> Result<Instruction, ProgramError> {
        if index >= MAX_INSTRUCTIONS {
            return Err(ProgramError::InstructionIndexOutOfRange(index));
        }

        let shift = INSTRUCTION_BITS * index;

        let encoded = ((self.bits >> shift) & INSTRUCTION_MASK) as u16;

        Ok(Instruction::decode(encoded)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::{Operation, Source};

    fn sample_instructions() -> [Instruction; 5] {
        [
            Instruction {
                operation: Operation::Add,
                destination: 0,
                a: Source::Input(0),
                b: Source::Input(1),
            },
            Instruction {
                operation: Operation::Sub,
                destination: 1,
                a: Source::Register(0),
                b: Source::Input(2),
            },
            Instruction {
                operation: Operation::Mul,
                destination: 2,
                a: Source::Register(1),
                b: Source::Register(2),
            },
            Instruction {
                operation: Operation::Max,
                destination: 3,
                a: Source::Input(3),
                b: Source::Register(0),
            },
            Instruction {
                operation: Operation::Min,
                destination: 0,
                a: Source::Register(3),
                b: Source::Input(1),
            },
        ]
    }

    #[test]
    fn create_program() {
        let instructions = sample_instructions();

        let program = Program::new(instructions).unwrap();

        assert_ne!(program.bits(), 0);
    }

    #[test]
    fn retrieve_all_instructions() {
        let instructions = sample_instructions();
        let program = Program::new(instructions).unwrap();

        for (index, expected) in instructions.iter().enumerate() {
            assert_eq!(program.instruction(index).unwrap(), *expected);
        }
    }

    #[test]
    fn program_bits_round_trip() {
        let instructions = sample_instructions();
        let program = Program::new(instructions).unwrap();

        let decoded = Program::from_bits(program.bits()).unwrap();

        assert_eq!(decoded, program);
    }

    #[test]
    fn reject_invalid_instruction_index() {
        let program = Program::new(sample_instructions()).unwrap();

        assert_eq!(
            program.instruction(5),
            Err(ProgramError::InstructionIndexOutOfRange(5))
        );
    }

    #[test]
    fn reject_bits_outside_program() {
        let invalid_bits = 1u64 << 60;

        assert_eq!(
            Program::from_bits(invalid_bits),
            Err(ProgramError::ProgramBitsOutOfRange(invalid_bits))
        );
    }
}
