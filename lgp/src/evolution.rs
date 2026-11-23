// Conserver les meilleurs individu d'une génération à la suivante

use env::pcgrng::PCGRng;

use crate::{
    evaluation::{self, EvaluationError},
    program::Program,
};

const PROGRAM_BITS: usize = 55;
const PROGRAM_MASK: u64 = (1u64 << PROGRAM_BITS) - 1;

/// Paramètres de l'algorithme génétique.
#[derive(Debug, Clone, Copy)]
pub struct EvolutionConfig {
    /// Nombre de programmes dans la population.
    pub population_size: usize,

    /// Nombre de générations à exécuter.
    pub generations: usize,

    /// Proportion de la population conservée pour la reproduction.
    pub survival_rate: f32,

    /// Probabilité de mutation de chaque bit.
    pub mutation_rate: f32,

    /// Nombre d'épisodes utilisés pour évaluer un programme.
    pub episodes: usize,

    /// Graine du générateur pseudo-aléatoire.
    pub seed: u64,
}

/// Meilleur résultat obtenu pendant l'évolution.
#[derive(Debug, Clone, Copy)]
pub struct EvolutionResult {
    pub best_program: Program,
    pub best_score: f32,
    pub best_generation: usize,
    pub evaluations: usize,
}

/// Erreurs possibles pendant l'évolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvolutionError {
    InvalidPopulationSize,
    InvalidGenerationCount,
    InvalidSurvivalRate,
    InvalidMutationRate,
    InvalidEpisodeCount,
    Evaluation(EvaluationError),
}

impl From<EvaluationError> for EvolutionError {
    fn from(error: EvaluationError) -> Self {
        EvolutionError::Evaluation(error)
    }
}

/// Un individu de la population.
#[derive(Debug, Clone, Copy)]
struct Individual {
    program: Program,
    score: f32,
}

/// Génère un programme aléatoire valide.
fn random_program(rng: &mut PCGRng) -> Program {
    let high = rng.next_u32() as u64;
    let low = rng.next_u32() as u64;

    let bits = ((high << 32) | low) & PROGRAM_MASK;

    Program::from_bits(bits).expect("Le masque garantit un programme valide")
}

/// Applique une mutation bit à bit à un programme.
fn mutate(program: Program, mutation_rate: f32, rng: &mut PCGRng) -> Program {
    let mut bits = program.bits();

    for bit in 0..PROGRAM_BITS {
        let random_value = rng.next_u32() as f64 / (u32::MAX as f64 + 1.0);

        if random_value < mutation_rate as f64 {
            bits ^= 1u64 << bit;
        }
    }

    Program::from_bits(bits).expect("La mutation ne modifie que les 55 bits utiles")
}

/// Évalue tous les individus d'une population.
fn evaluate_population(
    population: &mut [Individual],
    episodes: usize,
    seed: u64,
) -> Result<(), EvolutionError> {
    for individual in population.iter_mut() {
        individual.score = evaluation::evaluate(&individual.program, episodes, seed)?;
    }

    Ok(())
}

/// Trie les individus du meilleur au moins bon.
fn sort_population(population: &mut [Individual]) {
    population.sort_by(|a, b| {
        b.score
            .total_cmp(&a.score)
            .then_with(|| a.program.bits().cmp(&b.program.bits()))
    });
}

/// Exécute l'algorithme génétique.
pub fn run(config: EvolutionConfig) -> Result<EvolutionResult, EvolutionError> {
    if config.population_size < 2 {
        return Err(EvolutionError::InvalidPopulationSize);
    }

    if config.generations == 0 {
        return Err(EvolutionError::InvalidGenerationCount);
    }

    if !config.survival_rate.is_finite()
        || config.survival_rate <= 0.0
        || config.survival_rate > 1.0
    {
        return Err(EvolutionError::InvalidSurvivalRate);
    }

    if !config.mutation_rate.is_finite() || config.mutation_rate < 0.0 || config.mutation_rate > 1.0
    {
        return Err(EvolutionError::InvalidMutationRate);
    }

    if config.episodes == 0 {
        return Err(EvolutionError::InvalidEpisodeCount);
    }

    let mut rng = PCGRng::seed(config.seed);

    // Création de la population initiale.
    let mut population: Vec<Individual> = (0..config.population_size)
        .map(|_| Individual {
            program: random_program(&mut rng),
            score: 0.0,
        })
        .collect();

    let survivor_count = (config.population_size as f32 * config.survival_rate).ceil() as usize;

    let survivor_count = survivor_count.max(1).min(config.population_size);

    let mut best_program = population[0].program;
    let mut best_score = f32::NEG_INFINITY;
    let mut best_generation = 0;

    let mut evaluations = 0;

    // Boucle principale de l'évolution.
    for generation in 0..config.generations {
        evaluate_population(&mut population, config.episodes, config.seed)?;

        evaluations += population.len();

        sort_population(&mut population);

        // Le meilleur individu de cette génération.
        let current_best = population[0];

        if current_best.score > best_score {
            best_score = current_best.score;
            best_program = current_best.program;
            best_generation = generation;
        }

        // La dernière génération n'a pas besoin de reproduction.
        if generation + 1 == config.generations {
            break;
        }

        // Conservation des meilleurs individus (élitisme).
        let mut next_population: Vec<Individual> =
            population.iter().take(survivor_count).copied().collect();

        // Reproduction et mutation jusqu'à remplir la population.
        while next_population.len() < config.population_size {
            let parent_index = (rng.next_u32() as usize) % survivor_count;

            let parent = population[parent_index];

            let child = mutate(parent.program, config.mutation_rate, &mut rng);

            next_population.push(Individual {
                program: child,
                score: 0.0,
            });
        }

        population = next_population;
    }

    Ok(EvolutionResult {
        best_program,
        best_score,
        best_generation,
        evaluations,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> EvolutionConfig {
        EvolutionConfig {
            population_size: 8,
            generations: 3,
            survival_rate: 0.25,
            mutation_rate: 0.02,
            episodes: 1,
            seed: 42,
        }
    }

    #[test]
    fn evolution_returns_valid_result() {
        let result = run(test_config()).unwrap();

        assert!(result.best_score > 0.0);
        assert!(result.best_score <= 500.0);
        assert!(result.best_generation < 3);
        assert_eq!(result.evaluations, 24);
    }

    #[test]
    fn evolution_is_deterministic() {
        let first = run(test_config()).unwrap();
        let second = run(test_config()).unwrap();

        assert_eq!(first.best_score, second.best_score);
        assert_eq!(first.best_program, second.best_program);
        assert_eq!(first.best_generation, second.best_generation);
    }

    #[test]
    fn reject_invalid_population_size() {
        let config = EvolutionConfig {
            population_size: 1,
            ..test_config()
        };

        assert_eq!(
            run(config).unwrap_err(),
            EvolutionError::InvalidPopulationSize
        );
    }

    #[test]
    fn reject_invalid_mutation_rate() {
        let config = EvolutionConfig {
            mutation_rate: 1.5,
            ..test_config()
        };

        assert_eq!(
            run(config).unwrap_err(),
            EvolutionError::InvalidMutationRate
        );
    }
}
