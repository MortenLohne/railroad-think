//! We'll be using dfdx convolutional neural network to evaluate the board.
//! The input is a 7x7x47 tensor.
//!
//! - L rail: 4
//! - T rail: 4
//! - I rail: 2
//! - L road: 4
//! - T road: 4
//! - I road: 2
//! - overpass: 2
//! - I trans: 4
//! - L trans: 8
//! - X T road: 4
//! - X T rail: 4
//! - X road: 1
//! - X rail: 1
//! - X L: 4
//! - X I: 2
//!
//! Add one to mark the current choice
//! In total: 47
//!
//! The output is a single value, representing the predicted score of the board.
//!
#![cfg_attr(feature = "nightly", feature(generic_const_exprs))]

use dfdx::prelude::*;

use crate::{board, game::mv::Move};

type Model = (
    (Conv2D<47, 8, 3, 1, 1>, ReLU, Flatten2D),
    (Linear<392, 1>, ReLU),
);

pub fn create_model() -> Self {
    let mut rng = rand::thread_rng();
    let mut model: Model = Default::default();
    model.reset_params(&mut rng);
}

pub fn state_to_features(board: &Board, mv: &Move) -> Tensor<Rank3<47, 7, 7>> {
    let mut features = Tensor::zeros();
    for (i, row) in board.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            // TODO: one-hot-encode
            features[[cell.piece as usize, i, j]] = 1.0;
        }
    }

    if let Move::Place(placement) = mv {
        // TODO: one-hot-encode
        features[[placement.piece as usize, placement.row, placement.col]] = 1.0;
    }

    features
}

fn one_hot_encode_placement(placement: &Placement) -> Tensor<Rank1<47>> {}

#[allow(clippy::all)]
#[cfg(feature = "nightly")]
fn main() {
    let dev: Cpu = Default::default();
    let m: Model = dev.build_module();

    // single image forward
    let x: Tensor<Rank3<47, 7, 7>> = dev.sample_normal();
    let y = m.forward(x);

    // batched image forward
    let x: Tensor<Rank4<32, 47, 7, 7>> = dev.sample_normal();
    let y = m.forward(x);
}

#[cfg(not(feature = "nightly"))]
fn main() {
    panic!("Run with `cargo +nightly run ...` to run this.");
}
