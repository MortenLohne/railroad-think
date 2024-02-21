use rand::Rng;

use crate::game::Game;
use crate::mcts::heuristics::{HeuristicOptions, Heuristics};
use crate::mcts::MonteCarloTree;
use core::panic;
use indicatif::ProgressBar;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::thread;

use super::heuristics::nn::edge_strategy::EdgeStrategy;
#[must_use]
/// Test random heuristic values until we find good ones.
/// Log the results of each option somwhere so we have history.
///
/// # Panics
/// Panics if the file cannot be opened to save the heuristics
pub fn simulated_annealing(
    max_iterations: u32,
    initial_temperature: f64,
    initial_score: f64,
    temperature_decay_rate: f64,
    variable: Option<usize>,
) -> HeuristicOptions {
    let path = "./config/heuristics.json";
    let mut heuristics = Heuristics::from_json(path).expect("Error: Could not load heuristics");
    heuristics.rave = None;
    heuristics.move_nn = None;
    let heuristics = heuristics;

    let mut options = heuristics.parameters.as_array();

    let mut score = initial_score;
    let mut temperature = initial_temperature;

    for i in 0..max_iterations {
        println!();
        println!("Round: {i}. Score: {score}. Temperature: {temperature}.");
        println!("{options:?}");
        println!();

        let variable = variable.unwrap_or_else(|| rand::random::<usize>() % options.len());
        let index = rand::random::<usize>() % options[variable].len();
        let mut new_options = options;

        let random_value = rand::random::<f64>().mul_add(2.0, -1.0);
        let alpha = 0.5;
        new_options[variable][index] += (alpha * new_options[variable][index] * random_value)
            .mul_add(temperature, (1.0 - alpha) * random_value * temperature);

        println!(
            "Changed heuristic {variable} at index {index} to {}",
            new_options[variable][index]
        );
        // Test the new heuristic
        let new_score = test_heuristic(new_options);

        let accept_change = new_score > score || {
            let delta_score = new_score - score;
            let random = rand::random::<f64>();
            let accept_probability = (-delta_score / temperature).exp();
            let accept = random < accept_probability;
            if accept {
                temperature *= temperature_decay_rate;
            }
            accept
        };

        if accept_change {
            panic!("Accept change");
            // score = (new_score + score) / 2.0;
            // options[variable] = new_options[variable];
            // Heuristics::new(options.into())
            //     .to_csv(format!("./src/mcts/heuristics/training/heuristics_{i:03}.csv").as_str())
            //     .expect("Error: Could not save heuristics");
        }
    }
    options
}

/// Play 20 * 10 games with this heuristic and return the mean value
#[must_use]
pub fn test_heuristic(heuristics: HeuristicOptions) -> f64 {
    let mut total = 0.0;
    let iterations = 20;
    let batch_size = 10;

    for i in 0..iterations {
        total += f64::from(run(batch_size, heuristics));
        println!(
            " - partial mean: {:.2}",
            total / f64::from((i + 1) * batch_size)
        );
    }

    total / f64::from(iterations * batch_size)
}

/// Play `n` games with the given heuristic options
/// Spawns a thread for each `n`
/// Returns the sum of scores
#[must_use]
pub fn run(n: u8, heuristics: HeuristicOptions) -> i32 {
    let threads = (0..n)
        .map(|_| thread::spawn(move || play(Heuristics::new(heuristics.into()))))
        .collect::<Vec<thread::JoinHandle<_>>>();

    let mut sum = 0;
    for thread in threads {
        if let Ok(score) = thread.join() {
            sum += score;
        } else {
            unreachable!();
        }
    }

    sum
}

/// Play a single game with given heuristics
#[must_use]
pub fn play(heuristics: Heuristics) -> i32 {
    let mut rng = rand::thread_rng();
    let duration = 1000;
    let game_seed = rng.gen();
    let mcts_seed = rng.gen();

    // println!("{game_seed:?} {mcts_seed:?}");

    let mut game = Game::new_from_seed(game_seed);
    let mut mcts = MonteCarloTree::new_from_seed(game.clone(), mcts_seed);

    mcts.heuristics = heuristics;

    while !game.ended {
        mcts.search_duration(duration);
        let mv = mcts.best_move();
        mcts = MonteCarloTree::progress(mcts, mv, &mut game);
    }
    game.board.score()
}

/// Generate training data for the neural network
/// # Panics
/// Panics if the file cannot be opened
pub fn generate_training_data(samples: u64, iterations: u64) {
    let mut rng = rand::thread_rng();

    let bar = ProgressBar::new(samples);
    bar.inc(0);
    for _ in 0..samples {
        let game_seed = rng.gen();
        let mcts_seed = rng.gen();

        let mut game = Game::new_from_seed(game_seed);
        let mut mcts = MonteCarloTree::new_from_seed(game.clone(), mcts_seed);
        mcts.heuristics.move_nn = Some(EdgeStrategy::load("model-2"));

        let mut data: Vec<(String, String)> = Vec::new();

        while !game.ended {
            mcts.search_duration(iterations as u128);
            let mv = mcts.best_move();
            data.push((game.board.encode(), format!("{mv:?}")));
            mcts = MonteCarloTree::progress(mcts, mv, &mut game);
        }

        let score = game.board.score();

        let data = data
            .iter()
            .map(|(board, mv)| format!("{board},{mv},{score}"))
            .collect::<Vec<String>>()
            .join("\n");

        let path = "./src/mcts/heuristics/nn/training_data.csv";
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(path)
            .unwrap();

        if let Err(e) = writeln!(file, "{data}") {
            eprintln!("Couldn't write to file: {e}");
        }

        bar.println(format!("Score: {score}"));
        bar.inc(1);
    }
    bar.finish_and_clear();
}
