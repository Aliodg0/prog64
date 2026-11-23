// Exécution des instructions sur les entrés et les registres.

use crate::instruction::{Instruction, Operation, Source};
use crate::program::{Program, ProgramError};

/// Valeur minimale utilisée pour protéger les divisions.
const EPSILON: f32 = 1.0e-6;

/// Limite les valeurs intermédiaires pour éviter les débordements.
const MAX_VALUE: f32 = 1.0e6;

/// Récupère la valeur d'une opérande.
fn read_operand(source: Source, inputs: &[f32; 4], registers: &[f32; 4]) -> f32 {
    match source {
        Source::Input(index) => inputs[index as usize],
        Source::Register(index) => registers[index as usize],
    }
}

/// Exécute une opération mathématique.
fn calculate(operation: Operation, a: f32, b: f32) -> f32 {
    let result = match operation {
        Operation::Add => a + b,
        Operation::Sub => a - b,
        Operation::Mul => a * b,

        // Division protégée contre les divisions par zéro.
        Operation::Div => {
            if b.abs() < EPSILON {
                0.0
            } else {
                a / b
            }
        }

        // Logarithme naturel protégé.
        Operation::Ln => a.abs().max(EPSILON).ln(),

        // Exponentielle limitée pour éviter les débordements.
        Operation::Exp => a.clamp(-10.0, 10.0).exp(),

        Operation::Max => a.max(b),
        Operation::Min => a.min(b),
    };

    // Évite de propager les valeurs infinies ou NaN.
    if result.is_finite() {
        result.clamp(-MAX_VALUE, MAX_VALUE)
    } else {
        0.0
    }
}

/// Exécute une instruction sur les registres.
fn execute_instruction(instruction: Instruction, inputs: &[f32; 4], registers: &mut [f32; 4]) {
    let a = read_operand(instruction.a, inputs, registers);

    let b = read_operand(instruction.b, inputs, registers);

    let result = calculate(instruction.operation, a, b);

    registers[instruction.destination as usize] = result;
}

/// Exécute un programme LGP complet.
///
/// Les quatre registres sont initialisés à zéro.
/// Le résultat final est le contenu du registre R[0].
pub fn execute(program: &Program, inputs: [f32; 4]) -> Result<f32, ProgramError> {
    let mut registers = [0.0_f32; 4];

    for index in 0..5 {
        let instruction = program.instruction(index)?;

        execute_instruction(instruction, &inputs, &mut registers);
    }

    Ok(registers[0])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::{Operation, Source};

    fn make_program(instructions: [Instruction; 5]) -> Program {
        Program::new(instructions).unwrap()
    }

    #[test]
    fn execute_addition() {
        let program = make_program([
            Instruction {
                operation: Operation::Add,
                destination: 0,
                a: Source::Input(0),
                b: Source::Input(1),
            },
            Instruction {
                operation: Operation::Add,
                destination: 1,
                a: Source::Input(0),
                b: Source::Input(1),
            },
            Instruction {
                operation: Operation::Add,
                destination: 1,
                a: Source::Input(0),
                b: Source::Input(1),
            },
            Instruction {
                operation: Operation::Add,
                destination: 1,
                a: Source::Input(0),
                b: Source::Input(1),
            },
            Instruction {
                operation: Operation::Add,
                destination: 1,
                a: Source::Input(0),
                b: Source::Input(1),
            },
        ]);

        let result = execute(&program, [2.0, 3.0, 0.0, 0.0]).unwrap();

        assert_eq!(result, 5.0);
    }

    #[test]
    fn execute_register_dependency() {
        let program = make_program([
            Instruction {
                operation: Operation::Add,
                destination: 1,
                a: Source::Input(0),
                b: Source::Input(1),
            },
            Instruction {
                operation: Operation::Mul,
                destination: 0,
                a: Source::Register(1),
                b: Source::Input(2),
            },
            Instruction {
                operation: Operation::Add,
                destination: 2,
                a: Source::Input(0),
                b: Source::Input(1),
            },
            Instruction {
                operation: Operation::Add,
                destination: 2,
                a: Source::Input(0),
                b: Source::Input(1),
            },
            Instruction {
                operation: Operation::Add,
                destination: 2,
                a: Source::Input(0),
                b: Source::Input(1),
            },
        ]);

        let result = execute(&program, [2.0, 3.0, 4.0, 0.0]).unwrap();

        assert_eq!(result, 20.0);
    }

    #[test]
    fn division_by_zero_is_protected() {
        let result = calculate(Operation::Div, 10.0, 0.0);

        assert_eq!(result, 0.0);
    }

    #[test]
    fn logarithm_of_negative_value_is_protected() {
        let result = calculate(Operation::Ln, -1.0, 0.0);

        assert!(result.is_finite());
    }

    #[test]
    fn exponential_does_not_overflow() {
        let result = calculate(Operation::Exp, 1000.0, 0.0);

        assert!(result.is_finite());
        assert!(result <= MAX_VALUE);
    }

    #[test]
    fn registers_start_at_zero() {
        let program = make_program([
            Instruction {
                operation: Operation::Add,
                destination: 1,
                a: Source::Input(0),
                b: Source::Input(1),
            },
            Instruction {
                operation: Operation::Sub,
                destination: 1,
                a: Source::Input(0),
                b: Source::Input(1),
            },
            Instruction {
                operation: Operation::Mul,
                destination: 1,
                a: Source::Input(0),
                b: Source::Input(1),
            },
            Instruction {
                operation: Operation::Div,
                destination: 1,
                a: Source::Input(0),
                b: Source::Input(1),
            },
            Instruction {
                operation: Operation::Ln,
                destination: 1,
                a: Source::Input(0),
                b: Source::Input(1),
            },
        ]);

        let result = execute(&program, [2.0, 3.0, 0.0, 0.0]).unwrap();

        assert_eq!(result, 0.0);
    }
}
