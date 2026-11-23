//Encodage et décodage sur 11 bits

/// Les huit opérations disponibles dans notre langage LGP.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Add, // 000
    Sub, // 001
    Mul, // 010
    Div, // 011
    Ln,  // 100
    Exp, // 101
    Max, // 110
    Min, // 111
}

impl Operation {
    fn encode(self) -> u16 {
        match self {
            Operation::Add => 0b000,
            Operation::Sub => 0b001,
            Operation::Mul => 0b010,
            Operation::Div => 0b011,
            Operation::Ln => 0b100,
            Operation::Exp => 0b101,
            Operation::Max => 0b110,
            Operation::Min => 0b111,
        }
    }

    fn decode(bits: u16) -> Self {
        match bits {
            0b000 => Operation::Add,
            0b001 => Operation::Sub,
            0b010 => Operation::Mul,
            0b011 => Operation::Div,
            0b100 => Operation::Ln,
            0b101 => Operation::Exp,
            0b110 => Operation::Max,
            0b111 => Operation::Min,
            _ => unreachable!("Une opération est codée sur 3 bits"),
        }
    }
}

/// Une opérande peut provenir d'une entrée ou d'un registre.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    Input(u8),
    Register(u8),
}

impl Source {
    fn encode(self) -> Result<u16, InstructionError> {
        match self {
            Source::Input(index) if index < 4 => Ok(index as u16),
            Source::Register(index) if index < 4 => Ok(0b100 | index as u16),
            Source::Input(index) | Source::Register(index) => {
                Err(InstructionError::InvalidOperandIndex(index))
            }
        }
    }

    fn decode(bits: u16) -> Self {
        let index = (bits & 0b011) as u8;

        if bits & 0b100 == 0 {
            Source::Input(index)
        } else {
            Source::Register(index)
        }
    }
}

/// Erreurs possibles lors de l'encodage ou du décodage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionError {
    InvalidDestination(u8),
    InvalidOperandIndex(u8),
    EncodedValueTooLarge(u16),
}

/// Une instruction LGP composée de 11 bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction {
    pub operation: Operation,
    pub destination: u8,
    pub a: Source,
    pub b: Source,
}

impl Instruction {
    /// Encode une instruction sur 11 bits.
    pub fn encode(self) -> Result<u16, InstructionError> {
        if self.destination >= 4 {
            return Err(InstructionError::InvalidDestination(self.destination));
        }

        let operation = self.operation.encode();
        let destination = self.destination as u16;
        let a = self.a.encode()?;
        let b = self.b.encode()?;

        let encoded = (operation << 8) | (destination << 6) | (a << 3) | b;

        Ok(encoded)
    }

    /// Décode une valeur binaire de 11 bits.
    pub fn decode(bits: u16) -> Result<Self, InstructionError> {
        if bits > 0b111_1111_1111 {
            return Err(InstructionError::EncodedValueTooLarge(bits));
        }

        let operation = Operation::decode((bits >> 8) & 0b111);
        let destination = ((bits >> 6) & 0b11) as u8;
        let a = Source::decode((bits >> 3) & 0b111);
        let b = Source::decode(bits & 0b111);

        Ok(Instruction {
            operation,
            destination,
            a,
            b,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_instruction() {
        let instruction = Instruction {
            operation: Operation::Max,
            destination: 0,
            a: Source::Input(1),
            b: Source::Register(2),
        };

        assert_eq!(instruction.encode().unwrap(), 0b110_0000_1110);
    }

    #[test]
    fn decode_instruction() {
        let decoded = Instruction::decode(0b110_0000_1110).unwrap();

        let expected = Instruction {
            operation: Operation::Max,
            destination: 0,
            a: Source::Input(1),
            b: Source::Register(2),
        };

        assert_eq!(decoded, expected);
    }

    #[test]
    fn encode_decode_round_trip() {
        let instruction = Instruction {
            operation: Operation::Add,
            destination: 2,
            a: Source::Register(1),
            b: Source::Input(3),
        };

        let encoded = instruction.encode().unwrap();
        let decoded = Instruction::decode(encoded).unwrap();

        assert_eq!(decoded, instruction);
    }

    #[test]
    fn reject_invalid_destination() {
        let instruction = Instruction {
            operation: Operation::Add,
            destination: 4,
            a: Source::Input(0),
            b: Source::Input(1),
        };

        assert_eq!(
            instruction.encode(),
            Err(InstructionError::InvalidDestination(4))
        );
    }

    #[test]
    fn reject_invalid_operand() {
        let instruction = Instruction {
            operation: Operation::Add,
            destination: 0,
            a: Source::Input(4),
            b: Source::Input(1),
        };

        assert_eq!(
            instruction.encode(),
            Err(InstructionError::InvalidOperandIndex(4))
        );
    }

    #[test]
    fn reject_value_larger_than_11_bits() {
        assert_eq!(
            Instruction::decode(0b1000_0000_0000),
            Err(InstructionError::EncodedValueTooLarge(0b1000_0000_0000))
        );
    }
}
