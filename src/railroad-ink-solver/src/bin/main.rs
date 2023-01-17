use game::Game;
use mcts::heuristics::Heuristics;
use mcts::MonteCarloTree;
use railroad_ink_solver::mcts::trainer::simulated_annealing;
use railroad_ink_solver::*;
use std::thread;

fn main() {
    // Run the simulated annealing algorithm_
    println!("Running simulated annealing...");

    let heuristics = simulated_annealing(100, 1.0, 46.279, 0.98);
    println!("heuristics: {heuristics:?}");
}

/// Play `n` games
/// Spawns a thread for each `n`
/// Each thread returns iterations and score
pub fn run(n: u8, iterations: u64) -> Vec<thread::JoinHandle<(u64, i32)>> {
    (0..n)
        .map(|_| thread::spawn(move || play(iterations)))
        .collect()
}

/// Play single game
/// Returns iterations and score
pub fn play(iterations: u64) -> (u64, i32) {
    let mut game = Game::new();
    game.roll();

    let mut heuristics = Heuristics::default();
    heuristics.load_local_rave("./src/mcts/heuristics/by_turn.csv");
    heuristics.uniform_local_rave(4);
    let mut mcts = MonteCarloTree::new_with_heuristics(game.clone(), heuristics);

    while !game.ended {
        let mv = mcts.search_iterations(iterations).best_move();
        mcts = MonteCarloTree::progress(mcts, mv, &mut game);
    }

    (iterations, game.board.score())
}

/// Play single game
/// Returns iterations and score
pub fn play_and_dump_rave_heuristics(iterations: u64, i: u64) -> (u64, i32) {
    let mut game = Game::new();
    game.roll();

    let heuristics = Heuristics {
        use_rave: false,
        ..Default::default()
    };
    // heuristics.load_local_rave("./src/mcts/heuristics/by_turn.csv");
    // heuristics.uniform_local_rave(4);

    let mut mcts = MonteCarloTree::new_with_heuristics(game.clone(), heuristics);

    while !game.ended {
        let mv = mcts.search_iterations(iterations).best_move();
        println!("best move: {:?}", mv);
        mcts = MonteCarloTree::progress(mcts, mv, &mut game);
    }

    mcts.heuristics
        .dump_local_rave(format!("./data/rave/post_special_heuristics/iter_{}.csv", i).as_str())
        .unwrap();

    (iterations, game.board.score())
}
