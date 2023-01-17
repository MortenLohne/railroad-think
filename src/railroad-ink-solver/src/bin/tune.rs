use railroad_ink_solver::mcts::trainer::simulated_annealing;

fn main() {
    // Run the simulated annealing algorithm_
    println!("Running simulated annealing...");

    let heuristics = simulated_annealing(100, 1.0, 46.279, 0.98);
    println!("heuristics: {heuristics:?}");
}
