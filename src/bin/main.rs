#![feature(stdsimd)]

use clap::{Args, Parser};
use game::Game;
use mcts::heuristics::Heuristics;
use mcts::MonteCarloTree;
// use railroad_ink_solver::mcts::trainer::simulated_annealing;
use railroad_ink_solver::*;
use std::thread;

#[derive(Parser)] // requires `derive` feature
enum Cli {
    NN(NeuralNetworkArgs),
    Play(PlayArgs),
}

#[derive(Args)]
struct NeuralNetworkArgs {
    #[arg(short, long, default_value_t = true)]
    train: bool,

    #[arg(short, long)]
    generate_training_data: bool,

    #[arg(short, long)]
    loop_training: bool,
}

#[derive(Args, Debug)]
struct PlayArgs {
    #[arg(long, default_value = "1")]
    count: u32,

    #[arg(short, long, default_value = "1000")]
    duration: Option<u128>,

    #[arg(short, long)]
    iterations: Option<u64>,
}

fn main() {
    match Cli::parse() {
        Cli::NN(args) => {
            let mut initial_run = true;

            while args.loop_training || initial_run {
                initial_run = false;

                if args.generate_training_data {
                    let samples = 5;
                    let iterations = 200;
                    mcts::trainer::generate_training_data(samples, iterations);
                }

                if args.train {
                    let mut model =
                        mcts::heuristics::nn::edge_strategy::EdgeStrategy::load("model-2");
                    model.train_model_path("model-2", 100);
                }
            }
        }
        Cli::Play(args) => {
            let play_mode = if let Some(iterations) = args.iterations {
                PlayMode::Iterations(iterations)
            } else {
                PlayMode::Duration(args.duration.unwrap())
            };

            let handles = run(args.count as u8, play_mode);
            for handle in handles {
                let (n, score) = handle.join().unwrap();

                match play_mode {
                    PlayMode::Iterations(_) => println!("iterations: {n}, score: {score}"),
                    PlayMode::Duration(_) => println!("duration: {n}, score: {score}"),
                }
            }
        }
    }

    // // Run the simulated annealing algorithm_
    // println!("Running simulated annealing...");
    // let max_iterations = 10_000;
    // let initial_temperature = 1.0;
    // let initial_score = 53.279;
    // let temperature_decay_rate = 0.95;
    // let variable = Some(4);
    // let heuristics = simulated_annealing(
    //     max_iterations,
    //     initial_temperature,
    //     initial_score,
    //     temperature_decay_rate,
    //     variable,
    // );
    // println!("heuristics: {heuristics:?}");

    // let path = "./src/mcts/heuristics/heuristics.csv";
    // let heuristics = Heuristics::from_csv(path);

    // use mcts::heuristics::nn::edge_strategy::EdgeStrategy;
    // let mut nn = EdgeStrategy::create_model();
    // nn.train_model_path("model-16-16");

    // play(1000);
}

/// Play `n` games
/// Spawns a thread for each `n`
/// Each thread returns play mode stats and score
fn run(n: u8, play_mode: PlayMode) -> Vec<thread::JoinHandle<(u64, i32)>> {
    (0..n)
        .map(|_| thread::spawn(move || play(play_mode)))
        .collect()
}

#[derive(Copy, Clone)]
enum PlayMode {
    Iterations(u64),
    Duration(u128),
}

/// Play single game
/// Returns duration or iteration and score
fn play(play_mode: PlayMode) -> (u64, i32) {
    let mut game = Game::new();

    let heuristics = Heuristics::default();
    let mut mcts = MonteCarloTree::new_with_heuristics(game.clone(), heuristics);

    use mcts::heuristics::nn::edge_strategy::EdgeStrategy;
    let nn = EdgeStrategy::load("model-2");

    while !game.ended {
        let mv = match play_mode {
            PlayMode::Iterations(iterations) => mcts.search_iterations(iterations).best_move(),
            PlayMode::Duration(duration) => mcts.search_duration(duration).best_move(),
        };

        println!("{mv}, pred: {:.1}", nn.predict(&game.board, &mv));
        mcts = MonteCarloTree::progress(mcts, mv, &mut game);
    }

    println!("Score: {}", game.board.score());

    match play_mode {
        PlayMode::Iterations(iterations) => (iterations, game.board.score()),
        PlayMode::Duration(duration) => (duration as u64, game.board.score()),
    }
}

/// Play single game
/// Returns iterations and score
pub fn play_and_dump_rave_heuristics(iterations: u64, _i: u64) -> (u64, i32) {
    let mut game = Game::new();
    game.roll();

    let heuristics = Heuristics {
        // use_rave: false,
        ..Default::default()
    };
    // heuristics.load_local_rave("./src/mcts/heuristics/by_turn.csv");
    // heuristics.uniform_local_rave(4);

    let mut mcts = MonteCarloTree::new_with_heuristics(game.clone(), heuristics);

    while !game.ended {
        let mv = mcts.search_iterations(iterations).best_move();
        println!("best move: {mv:?}");
        mcts = MonteCarloTree::progress(mcts, mv, &mut game);
    }

    // mcts.heuristics
    //     .dump_local_rave(format!("./data/rave/post_special_heuristics/iter_{i}.csv").as_str())
    //     .unwrap();

    (iterations, game.board.score())
}
