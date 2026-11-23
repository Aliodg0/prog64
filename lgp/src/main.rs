use prog64::{
    evaluation,
    evolution::{self, EvolutionConfig},
    program,
};

fn main() {
    // Configuration de l'algorithme génétique.
    let config = EvolutionConfig {
        population_size: 120,
        generations: 40,
        survival_rate: 0.20,
        mutation_rate: 0.02,
        episodes: 100,
        seed: 10099,
    };

    // Configuration de la validation indépendante.
    let validation_episodes = 100;
    let validation_seed = 1000;

    println!("========================================");
    println!("      😊 QLOG INSA RENNES 😊");
    println!("========================================");

    println!("========================================");
    println!("       PROG64 - LGP CARTPOLE");
    println!("========================================");
    println!("Population       : {}", config.population_size);
    println!("Generations      : {}", config.generations);
    println!("Taux de survie   : {:.0} %", config.survival_rate * 100.0);
    println!("Taux de mutation : {:.2} %", config.mutation_rate * 100.0);
    println!("Episodes         : {}", config.episodes);
    println!("Graine            : {}", config.seed);
    println!("----------------------------------------");

    println!("Demarrage de l'evolution...\n");

    match evolution::run(config) {
        Ok(result) => {
            println!("\n========================================");
            println!("          RESULTATS DE L'EVOLUTION");
            println!("========================================");

            println!("Meilleur score       : {:.2} / 500", result.best_score);
            println!("Generation trouvee   : {}", result.best_generation + 1);
            println!("Nombre d'evaluations : {}", result.evaluations);
            println!("Programme (u64)      : {}", result.best_program.bits());

            println!("\nInstructions du meilleur programme :");

            for index in 0..program::MAX_INSTRUCTIONS {
                match result.best_program.instruction(index) {
                    Ok(instruction) => {
                        println!("Instruction {} : {:?}", index + 1, instruction);
                    }
                    Err(error) => {
                        eprintln!(
                            "Erreur lors de la lecture de l'instruction {} : {:?}",
                            index + 1,
                            error
                        );
                    }
                }
            }

            // Validation indépendante du meilleur programme.
            println!("\n========================================");
            println!("       VALIDATION INDEPENDANTE");
            println!("========================================");

            println!("Nombre d'episodes : {}", validation_episodes);
            println!("Graine initiale   : {}", validation_seed);
            println!(
                "Graines utilisees : {} a {}",
                validation_seed,
                validation_seed + validation_episodes as u64 - 1
            );

            println!("\nEvaluation en cours...\n");

            match evaluation::evaluate_statistics(
                &result.best_program,
                validation_episodes,
                validation_seed,
            ) {
                Ok(stats) => {
                    let success_rate =
                        stats.perfect_episodes as f32 / stats.episodes as f32 * 100.0;

                    println!("========================================");
                    println!("       RESULTATS DE VALIDATION");
                    println!("========================================");

                    println!("Score moyen          : {:.2} / 500", stats.mean_score);
                    println!("Score minimum        : {:.2} / 500", stats.min_score);
                    println!("Score maximum        : {:.2} / 500", stats.max_score);
                    println!("Ecart-type           : {:.2}", stats.standard_deviation);
                    println!(
                        "Episodes parfaits    : {} / {}",
                        stats.perfect_episodes, stats.episodes
                    );
                    println!("Taux de reussite     : {:.2} %", success_rate);

                    println!("========================================");
                }
                Err(error) => {
                    eprintln!("Erreur pendant la validation : {:?}", error);
                    std::process::exit(1);
                }
            }
        }

        Err(error) => {
            eprintln!("Erreur pendant l'evolution : {:?}", error);
            std::process::exit(1);
        }
    }
}

/*

use prog64::{
    evaluation,
    evolution::{self, EvolutionConfig},
    program,
};

// Programmes obtenus lors des huit expériences avec 100 épisodes.
const PROGRAMS: [u64; 8] = [
    606426271526940,   // Graine 20000
    370215627865214,   // Graine 20005
    8773295682354747,  // Graine 20015
    31297001543616723, // Graine 20025
    19175800728890977, // Graine 20050
    1126003067387466,  // Graine 20075
    1020347844308452,  // Graine 20090
    469797559977199,   // Graine 20099
];

const VALIDATION_EPISODES: usize = 100;
const VALIDATION_SEED: u64 = 30000;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|arg| arg == "--validate") {
        validate_fixed_programs();
    } else {
        run_evolution();
    }
}

// --------------------------------------------------
// MODE 1 : EVOLUTION GENETIQUE
// --------------------------------------------------

fn run_evolution() {
    let config = EvolutionConfig {
        population_size: 120,
        generations: 40,
        survival_rate: 0.20,
        mutation_rate: 0.02,
        episodes: 100,
        seed: 300,
    };

    let validation_episodes = 100;
    let validation_seed = 1000;

    println!("========================================");
    println!("          QLOG INSA RENNES");
    println!("========================================");
    println!("       PROG64 - LGP CARTPOLE");
    println!("========================================");

    println!("Population       : {}", config.population_size);
    println!("Generations      : {}", config.generations);
    println!("Taux de survie   : {:.0} %", config.survival_rate * 100.0);
    println!("Taux de mutation : {:.2} %", config.mutation_rate * 100.0);
    println!("Episodes         : {}", config.episodes);
    println!("Graine           : {}", config.seed);

    println!("----------------------------------------");
    println!("Demarrage de l'evolution...\n");

    match evolution::run(config) {
        Ok(result) => {
            println!("\n========================================");
            println!("          RESULTATS DE L'EVOLUTION");
            println!("========================================");

            println!("Meilleur score       : {:.2} / 500", result.best_score);
            println!("Generation trouvee   : {}", result.best_generation + 1);
            println!("Nombre d'evaluations : {}", result.evaluations);
            println!("Programme (u64)      : {}", result.best_program.bits());

            println!("\nInstructions du meilleur programme :");

            for index in 0..program::MAX_INSTRUCTIONS {
                match result.best_program.instruction(index) {
                    Ok(instruction) => {
                        println!("Instruction {} : {:?}", index + 1, instruction);
                    }
                    Err(error) => {
                        eprintln!("Erreur de lecture : {:?}", error);
                    }
                }
            }

            println!("\n========================================");
            println!("       VALIDATION INDEPENDANTE");
            println!("========================================");

            println!("Nombre d'episodes : {}", validation_episodes);
            println!("Graine initiale   : {}", validation_seed);
            println!(
                "Graines utilisees : {} a {}",
                validation_seed,
                validation_seed + validation_episodes as u64 - 1
            );

            match evaluation::evaluate_statistics(
                &result.best_program,
                validation_episodes,
                validation_seed,
            ) {
                Ok(stats) => {
                    let success_rate =
                        stats.perfect_episodes as f32 / stats.episodes as f32 * 100.0;

                    println!("\n========================================");
                    println!("       RESULTATS DE VALIDATION");
                    println!("========================================");

                    println!("Score moyen       : {:.2} / 500", stats.mean_score);
                    println!("Score minimum     : {:.2} / 500", stats.min_score);
                    println!("Score maximum     : {:.2} / 500", stats.max_score);
                    println!("Ecart-type        : {:.2}", stats.standard_deviation);
                    println!(
                        "Episodes parfaits : {} / {}",
                        stats.perfect_episodes, stats.episodes
                    );
                    println!("Taux de reussite  : {:.2} %", success_rate);
                    println!("========================================");
                }
                Err(error) => {
                    eprintln!("Erreur pendant la validation : {:?}", error);
                    std::process::exit(1);
                }
            }
        }
        Err(error) => {
            eprintln!("Erreur pendant l'evolution : {:?}", error);
            std::process::exit(1);
        }
    }
}

// --------------------------------------------------
// MODE 2 : VALIDATION DES PROGRAMMES ENREGISTRES
// --------------------------------------------------

fn validate_fixed_programs() {
    println!("========================================");
    println!("          QLOG INSA RENNES");
    println!("========================================");
    println!("    VALIDATION DES PROGRAMMES LGP");
    println!("========================================");

    println!("Nombre de programmes : {}", PROGRAMS.len());
    println!("Nombre d'episodes    : {}", VALIDATION_EPISODES);
    println!("Graine initiale      : {}", VALIDATION_SEED);
    println!(
        "Graines utilisees    : {} a {}",
        VALIDATION_SEED,
        VALIDATION_SEED + VALIDATION_EPISODES as u64 - 1
    );

    println!("\nEvaluation en cours...\n");

    println!(
        "{:<10} {:<20} {:>12} {:>12} {:>12} {:>12} {:>12}",
        "Graine", "Programme u64", "Moyenne", "Minimum", "Maximum", "Ecart-type", "Reussite"
    );

    println!("{}", "-".repeat(105));

    for (index, bits) in PROGRAMS.iter().copied().enumerate() {
        let seed_label = [20000, 20005, 20015, 20025, 20050, 20075, 20090, 20099][index];

        let candidate = match program::Program::from_bits(bits) {
            Ok(candidate) => candidate,
            Err(error) => {
                eprintln!(
                    "Impossible de decoder le programme {} : {:?}",
                    seed_label, error
                );
                continue;
            }
        };

        match evaluation::evaluate_statistics(&candidate, VALIDATION_EPISODES, VALIDATION_SEED) {
            Ok(stats) => {
                let success_rate = stats.perfect_episodes as f32 / stats.episodes as f32 * 100.0;

                println!(
                    "{:<10} {:<20} {:>12.2} {:>12.2} {:>12.2} {:>12.2} {:>11.2} %",
                    seed_label,
                    bits,
                    stats.mean_score,
                    stats.min_score,
                    stats.max_score,
                    stats.standard_deviation,
                    success_rate
                );
            }
            Err(error) => {
                eprintln!(
                    "Erreur pendant l'evaluation du programme {} : {:?}",
                    seed_label, error
                );
            }
        }
    }

    println!("{}", "-".repeat(105));
    println!("Validation terminee.");
}
*/
