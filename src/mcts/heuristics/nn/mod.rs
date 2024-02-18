use crate::{board::Board, game::mv::Move};

pub mod edge_strategy;
// pub mod face_strategy;

trait NNHeuristic {
    fn predict(&self, board: &Board, mv: Vec<Move>) -> Vec<f32>;
}
