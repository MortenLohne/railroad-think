use clap::{Args, Parser};
use game::Game;
use mcts::heuristics::Heuristics;
use mcts::MonteCarloTree;
use railroad_ink_solver::mcts::trainer::simulated_annealing;
use railroad_ink_solver::*;
use std::thread;

#[derive(Parser)] // requires `derive` feature
enum Cli {
    NN(NeuralNetworkArgs),
}

#[derive(Args)]
struct NeuralNetworkArgs {
    /// Name of the person to greet
    #[arg(short, long)]
    train: bool,
}

fn main() {
    let Cli::NN(args) = Cli::parse();
    if args.train {
        // let path = "./src/mcts/heuristics/heuristics.csv";
        // let heuristics = Heuristics::from_csv(path);
        // generate_training_data(100, 1000);
        let mut model = mcts::heuristics::nn::edge_strategy::EdgeStrategy::create_model();
        model.train_model_path("2024-test");
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
    // generate_training_data(100, 1000);

    // use mcts::heuristics::nn::edge_strategy::EdgeStrategy;
    // let mut nn = EdgeStrategy::create_model();
    // nn.train_model_path("model-16-16");

    // play(1000);
}

/// Play `n` games
/// Spawns a thread for each `n`
/// Each thread returns iterations and score
pub fn run(n: u8, duration: u128) -> Vec<thread::JoinHandle<(u128, i32)>> {
    (0..n)
        .map(|_| thread::spawn(move || play(duration)))
        .collect()
}

/// Play single game
/// Returns iterations and score
pub fn play(duration: u128) -> (u128, i32) {
    let mut game = Game::new();

    let heuristics = Heuristics::default();
    let mut mcts = MonteCarloTree::new_with_heuristics(game.clone(), heuristics);

    use mcts::heuristics::nn::edge_strategy::EdgeStrategy;
    let nn = EdgeStrategy::load("model-dropout");

    while !game.ended {
        let mv = mcts.search_duration(duration).best_move();

        println!("{mv}, pred: {:.1}", nn.predict(&game.board, &mv));
        mcts = MonteCarloTree::progress(mcts, mv, &mut game);
    }

    println!("Score: {}", game.board.score());

    (duration, game.board.score())
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
