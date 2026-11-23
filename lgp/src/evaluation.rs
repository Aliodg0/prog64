// Évaluation CartPole : calcul du score moyen sur plusieurs épisode, avec un résultat déterministe pour une même graine.

use env::{Environment, cartpole::CartPole, pcgrng::PCGRng};

use crate::{
    interpreter,
    program::{Program, ProgramError},
};

/// Nombre maximal de pas dans un épisode CartPole.
const MAX_STEPS: usize = 500;

/// Erreurs possibles pendant l'évaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvaluationError {
    EmptyEpisodes,
    Program(ProgramError),
}

impl From<ProgramError> for EvaluationError {
    fn from(error: ProgramError) -> Self {
        EvaluationError::Program(error)
    }
}

/// Évalue un programme sur un épisode CartPole.
///
/// La graine permet de reproduire exactement l'épisode.
pub fn evaluate_episode(program: &Program, seed: u64) -> Result<f32, EvaluationError> {
    let mut rng = PCGRng::seed(seed);
    let mut environment = CartPole::default();

    // Initialise l'environnement avec la graine fournie.
    let mut state = environment.reset(&mut rng);

    let mut total_reward = 0.0;

    for _ in 0..MAX_STEPS {
        // Exécute le programme sur l'état courant.
        let output = interpreter::execute(program, state)?;

        // Résultat positif : droite.
        // Résultat nul ou négatif : gauche.
        let action = if output > 0.0 { 1.0 } else { 0.0 };

        let (next_state, reward, done) = environment.step(action, &mut rng);

        total_reward += reward;
        state = next_state;

        if done {
            break;
        }
    }

    Ok(total_reward)
}

/// Calcule le score moyen sur plusieurs épisodes.
///
/// Chaque épisode utilise une graine différente.
/// Les mêmes paramètres produisent les mêmes résultats.
pub fn evaluate(program: &Program, episodes: usize, seed: u64) -> Result<f32, EvaluationError> {
    if episodes == 0 {
        return Err(EvaluationError::EmptyEpisodes);
    }

    let mut total_score = 0.0;

    for episode in 0..episodes {
        let episode_seed = seed.wrapping_add(episode as u64);

        total_score += evaluate_episode(program, episode_seed)?;
    }

    Ok(total_score / episodes as f32)
}

/// Statistiques détaillées obtenues pendant une validation indépendante.
#[derive(Debug, Clone, Copy)]
pub struct EvaluationStats {
    pub episodes: usize,
    pub mean_score: f32,
    pub min_score: f32,
    pub max_score: f32,
    pub standard_deviation: f32,
    pub perfect_episodes: usize,
}

/// Évalue un programme sur plusieurs épisodes et calcule ses statistiques.
///
/// Chaque épisode utilise une graine différente.
pub fn evaluate_statistics(
    program: &Program,
    episodes: usize,
    seed: u64,
) -> Result<EvaluationStats, EvaluationError> {
    if episodes == 0 {
        return Err(EvaluationError::EmptyEpisodes);
    }

    let mut scores = Vec::with_capacity(episodes);

    for episode in 0..episodes {
        let episode_seed = seed.wrapping_add(episode as u64);
        let score = evaluate_episode(program, episode_seed)?;
        scores.push(score);
    }

    let total_score: f32 = scores.iter().sum();
    let mean_score = total_score / episodes as f32;

    let min_score = scores.iter().copied().fold(f32::INFINITY, f32::min);
    let max_score = scores.iter().copied().fold(f32::NEG_INFINITY, f32::max);

    // Écart-type calculé sur l'ensemble des épisodes.
    let variance = scores
        .iter()
        .map(|score| (score - mean_score).powi(2))
        .sum::<f32>()
        / episodes as f32;

    let standard_deviation = variance.sqrt();

    let perfect_episodes = scores.iter().filter(|&&score| score >= 500.0).count();

    Ok(EvaluationStats {
        episodes,
        mean_score,
        min_score,
        max_score,
        standard_deviation,
        perfect_episodes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::{Instruction, Operation, Source};

    fn sample_program() -> Program {
        let instruction = Instruction {
            operation: Operation::Add,
            destination: 0,
            a: Source::Input(0),
            b: Source::Input(2),
        };

        Program::new([instruction; 5]).unwrap()
    }

    #[test]
    fn evaluate_one_episode() {
        let program = sample_program();

        let score = evaluate_episode(&program, 42).unwrap();

        assert!(score > 0.0);
        assert!(score <= MAX_STEPS as f32);
    }

    #[test]
    fn evaluate_multiple_episodes() {
        let program = sample_program();

        let score = evaluate(&program, 5, 42).unwrap();

        assert!(score > 0.0);
        assert!(score <= MAX_STEPS as f32);
    }

    #[test]
    fn evaluation_is_deterministic() {
        let program = sample_program();

        let first = evaluate(&program, 5, 42).unwrap();
        let second = evaluate(&program, 5, 42).unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn reject_zero_episodes() {
        let program = sample_program();

        assert_eq!(
            evaluate(&program, 0, 42),
            Err(EvaluationError::EmptyEpisodes)
        );
    }
}
