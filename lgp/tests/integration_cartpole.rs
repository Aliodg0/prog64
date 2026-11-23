use prog64::{
    evaluation::evaluate_statistics,
    instruction::{Instruction, Operation, Source},
    interpreter,
    program::{MAX_INSTRUCTIONS, Program},
};

fn make_program(a: Source, b: Source) -> Program {
    let instruction = Instruction {
        operation: Operation::Add,
        destination: 0,
        a,
        b,
    };

    Program::new([instruction; MAX_INSTRUCTIONS]).unwrap()
}

#[test]
fn public_api_builds_encodes_and_executes_program() {
    let program = make_program(Source::Input(0), Source::Input(1));

    // The program must fit in the 55 bits reserved for its instructions.
    assert!(program.bits() < (1u64 << 55));

    // Reconstruct the program from its binary representation.
    let reconstructed = Program::from_bits(program.bits()).unwrap();

    assert_eq!(reconstructed.bits(), program.bits());

    // Execute the reconstructed program through the public interpreter API.
    let result = interpreter::execute(&reconstructed, [2.0, 3.0, 4.0, 5.0]).unwrap();

    assert_eq!(result, 5.0);
}

#[test]
fn cartpole_evaluation_statistics_are_reproducible() {
    // The program uses cart position and pole angle as inputs.
    let program = make_program(Source::Input(0), Source::Input(2));

    let first = evaluate_statistics(&program, 5, 42).unwrap();
    let second = evaluate_statistics(&program, 5, 42).unwrap();

    assert_eq!(first.episodes, 5);
    assert_eq!(first.mean_score, second.mean_score);
    assert_eq!(first.min_score, second.min_score);
    assert_eq!(first.max_score, second.max_score);
    assert_eq!(first.standard_deviation, second.standard_deviation);
    assert_eq!(first.perfect_episodes, second.perfect_episodes);

    assert!(first.mean_score.is_finite());
    assert!(first.min_score > 0.0);
    assert!(first.max_score <= 500.0);
    assert!(first.min_score <= first.mean_score);
    assert!(first.mean_score <= first.max_score);
    assert!(first.standard_deviation.is_finite());
    assert!(first.perfect_episodes <= first.episodes);
}
