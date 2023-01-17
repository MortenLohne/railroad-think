use crate::game::Game;
use crate::mcts::heuristics::{HeuristicOptions, Heuristics};
use crate::mcts::MonteCarloTree;
use std::thread;

#[must_use]
/// Test random heuristic values until we find good ones.
/// Log the results of each option somwhere so we have history.
pub fn simulated_annealing(
    max_iterations: u32,
    initial_temperature: f64,
    initial_score: f64,
    temperature_decay_rate: f64,
) -> HeuristicOptions {
    let path = "./src/mcts/heuristics/heuristics.csv";
    let heuristics = Heuristics::from_csv(path);

    let mut options = [
        heuristics.exploration_variables,
        heuristics.special_cost,
        heuristics.frontier_size,
    ];

    let mut score = initial_score;
    let mut temperature = initial_temperature;

    for i in 0..max_iterations {
        println!();
        println!("Round: {i}. Score: {score}. Temperature: {temperature}.");
        println!("{options:?}");
        println!();

        let variable = rand::random::<usize>() % 2;
        let index = rand::random::<usize>() % 7;
        let mut new_options = options;

        let random_value = rand::random::<f64>() * 2.0 - 1.0;
        let alpha = 0.5;
        new_options[variable][index] +=
            (alpha * new_options[variable][index] * random_value * temperature)
                + (1.0 - alpha) * random_value * temperature;

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
            score = (new_score + score) / 2.0;
            options[variable] = new_options[variable];
            Heuristics::new(options)
                .to_csv(format!("./src/mcts/heuristics/training/heuristics_{i:03}.csv").as_str())
                .expect("Error: Could not save heuristics");
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
            " - partial mean: {}",
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
        .map(|_| thread::spawn(move || play(Heuristics::new(heuristics))))
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
    let duration = 1000;

    let mut game = Game::new();
    game.roll();

    let mut mcts = MonteCarloTree::new_with_heuristics(game.clone(), heuristics);
    while !game.ended {
        mcts.search_duration(duration);
        let mv = mcts.best_move();
        mcts = MonteCarloTree::progress(mcts, mv, &mut game);
    }
    game.board.score()
}
