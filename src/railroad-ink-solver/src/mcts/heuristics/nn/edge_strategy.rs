//! This strategy encodes the edge of the board as a feature plane.
//! Each square has four edges, and each edge has three possible states:
//! - rail
//! - road
//! - current
//!
//! Seems to not work that well, maybe because the training data is too small.
//! Another solution is to train the network to value a placement instead.
//!
//! Board => nn => Placement 1hot
//!
//! Value either by score or boolean (whether the move was chosen or not)
#![cfg_attr(feature = "nightly", feature(generic_const_exprs))]

use crate::{
    board::{self, placement::Placement, square::Square},
    game::mv::Move,
    pieces::Connection,
};
use indicatif::ProgressBar;
use std::{io::Read, time::Instant};

use dfdx::{data::SubsetIterator, losses::mse_loss, optim::Adam, prelude::*};

const MODEL_PATH: &str = "./src/mcts/heuristics/nn";

#[cfg(feature = "nightly")]
type Model = (
    (Conv2D<7, 3, 3, 1, 1>, ReLU, Flatten2D),
    (Linear<147, 1>, ReLU),
);

#[cfg(not(feature = "nightly"))]
type Model = (
    (Linear<588, 16>, ReLU),
    // DropoutOneIn<2>,
    (Linear<16, 16>, ReLU),
    (Linear<16, 1>, ReLU),
);

type Device = Cpu;

const BATCH_SIZE: usize = 1024;

#[derive(Clone)]
pub struct EdgeStrategy {
    model: Model,
    device: Device,
}

impl EdgeStrategy {
    #[must_use]
    pub fn create_model() -> Self {
        let device: Cpu = dfdx::tensor::Cpu::default();
        let mut model: Model = device.build_module();
        model.reset_params();
        Self { model, device }
    }

    /// # Panics
    /// If the model cannot be saved
    #[must_use]
    pub fn load(model_name: &str) -> Self {
        let device: Cpu = dfdx::tensor::Cpu::default();
        let mut model: Model = device.build_module();
        model
            .load(format!("{MODEL_PATH}/{model_name}.npz"))
            .expect("Could not load model");

        Self { model, device }
    }

    #[must_use]
    pub fn predict(&self, board: &board::Board, mv: &Move) -> f32 {
        self.model
            .forward(Self::get_features(board, *mv, &self.device))
            .array()[0]
    }

    pub fn train_model(&mut self) {
        self.train_model_path("model");
    }

    /// # Panics
    /// If the model cannot be saved
    pub fn train_model_path(&mut self, model_path: &str) {
        let device: Cpu = dfdx::tensor::Cpu::default();

        let mut rng = rand::thread_rng();

        let mut optimizer: Adam<Model, Cpu> = dfdx::optim::Adam::default();

        let dataset = Dataset::load(&device);

        for i_epoch in 0..5000 {
            let mut total_epoch_loss = 0.0;
            let mut num_batches = 0;
            let start = Instant::now();
            let feature_count = dataset.features.len();
            let bar = ProgressBar::new(feature_count as u64);
            let subsets = SubsetIterator::<BATCH_SIZE>::shuffled(feature_count, &mut rng);

            for (features, labels) in
                subsets.map(|indices| dataset.get_batch::<BATCH_SIZE>(&device, indices))
            {
                let pred = self.model.forward_mut(features.traced());
                let loss = mse_loss(pred, labels.clone());

                total_epoch_loss += loss.array();
                num_batches += 1;
                bar.inc(BATCH_SIZE as u64);

                let gradients = loss.backward();
                optimizer.update(&mut self.model, gradients).unwrap();
            }
            let dur = start.elapsed();
            bar.finish_and_clear();

            println!(
                "Epoch {i_epoch:03} in {:.2} ms ({:.2} batches/s): avg sample loss {:.3}",
                dur.as_millis(),
                num_batches as f32 / dur.as_secs_f32(),
                total_epoch_loss / num_batches as f32,
            );

            if (i_epoch + 1) % 250 == 0 {
                println!("saving");
                self.model
                    .save(format!("{MODEL_PATH}/{model_path}.npz"))
                    .expect("failed to save model");
            }
        }
    }

    #[cfg(feature = "nightly")]
    fn get_features(board: &board::Board, mv: Move, device: &Device) -> Tensor<Rank3<7, 7, 7>> {
        let mut features = device.zeros();
        let mut data = [0.0; 7 * 7 * 7];
        for y in 0..board::BOARD_SIZE {
            for x in 0..board::BOARD_SIZE {
                let ft = board[&Square::<7>::new(x, y)]
                    .map_or([0.0; 7], Self::get_features_for_placement);
                let x = x as usize;
                let y = y as usize;
                for i in 0..ft.len() {
                    data[y * 7 * 7 + x * 7 + i] = ft[i];
                }
            }
        }

        if let Move::Place(placement) = mv {
            let square = placement.square;
            let ft = Self::get_features_for_placement(placement);
            let x = square.x() as usize;
            let y = square.y() as usize;
            let start = y * 7 * 7 + x * 7;

            for i in 0..ft.len() {
                data[start + i] = ft[i];
            }

            for i in 0..4 {
                data[start + i * 3 + 2] = 1.0;
            }
        }
        features.copy_from(&data);
        features
    }

    #[cfg(not(feature = "nightly"))]
    fn get_features(board: &board::Board, mv: Move, device: &Device) -> Tensor<Rank1<588>> {
        let mut features = device.zeros();
        let mut data = [0.0; 588];
        for y in 0..board::BOARD_SIZE {
            for x in 0..board::BOARD_SIZE {
                let ft = board[&Square::<7>::new(x, y)]
                    .map_or([0.0; 12], Self::get_features_for_placement);
                let x = x as usize;
                let y = y as usize;
                let start = y * 7 * 7 + x * 7;

                data[start..(ft.len() + start)].copy_from_slice(&ft[..]);
            }
        }

        if let Move::Place(placement) = mv {
            let square = placement.square;
            let ft = Self::get_features_for_placement(placement);
            let x = square.x() as usize;
            let y = square.y() as usize;
            let start = y * 7 * 7 + x * 7;

            data[start..(ft.len() + start)].copy_from_slice(&ft[..]);

            for i in 0..4 {
                data[start + i * 3 + 2] = 1.0;
            }
        }
        features.copy_from(&data);
        features
    }

    fn get_features_for_placement(placement: Placement) -> [f32; 12] {
        let mut cell = [0.0; 12];
        placement
            .get_networks()
            .iter()
            .flatten()
            .for_each(|network| {
                for direction in 0..4 {
                    match network[direction] {
                        Connection::Road => cell[direction * 3] = 1.0,
                        Connection::Rail => cell[direction * 3 + 1] = 1.0,
                        Connection::None => {}
                    }
                }
            });
        cell
    }
}

struct Dataset {
    #[cfg(feature = "nightly")]
    pub features: Vec<Tensor<Rank3<7, 7, 7>>>,
    #[cfg(not(feature = "nightly"))]
    pub features: Vec<Tensor<Rank1<588>>>,
    pub labels: Vec<Tensor<Rank1<1>>>,
}

impl Dataset {
    pub fn load(device: &Device) -> Self {
        let mut boards = Vec::new();
        let mut moves = Vec::new();
        let mut scores = Vec::new();

        let mut file = std::fs::File::open("./src/mcts/heuristics/nn/training_data.csv").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        for line in contents.lines() {
            let mut parts = line.split(',');
            let board = parts.next().unwrap();
            let mv = parts.next().unwrap();
            let score = parts.next().unwrap().parse::<f32>().unwrap();
            let board = board::Board::decode(board);
            let mv = mv.parse::<Move>().unwrap();
            boards.push(board);
            moves.push(mv);
            scores.push(score);
        }

        let features = boards
            .into_iter()
            .zip(moves)
            .map(|(board, mv)| EdgeStrategy::get_features(&board, mv, device))
            .collect();

        let labels: Vec<Tensor<Rank1<1>>> = scores
            .into_iter()
            .map(|score| device.tensor([score]))
            .collect();

        Self { features, labels }
    }

    #[cfg(feature = "nightly")]
    pub fn get_batch<const B: usize>(
        &self,
        device: &Device,
        indices: [usize; B],
    ) -> (Tensor<Rank4<B, 7, 7, 7>>, Tensor<Rank2<B, 1>>) {
        let mut features_data = Vec::with_capacity(B * 7 * 7 * 7);
        let mut labels_data = Vec::with_capacity(B * 1);

        for &index in indices.iter() {
            features_data.extend(
                self.features
                    .get(index)
                    .unwrap()
                    .array()
                    .iter()
                    .flatten()
                    .flatten()
                    .copied(),
            );
            labels_data.extend(self.labels.get(index).unwrap().array().iter().copied());
        }

        let mut features = device.zeros();
        features.copy_from(&features_data);
        let mut labels = device.zeros();
        labels.copy_from(&labels_data);
        (features, labels)
    }

    #[cfg(not(feature = "nightly"))]
    pub fn get_batch<const B: usize>(
        &self,
        device: &Device,
        indices: [usize; B],
    ) -> (
        Tensor<Rank2<B, 588>, f32, Device>,
        Tensor<Rank2<B, 1>, f32, Device>,
    ) {
        let mut features_data = Vec::with_capacity(B * 588);
        let mut labels_data = Vec::with_capacity(B);

        for &index in &indices {
            features_data.extend(self.features.get(index).unwrap().array().iter().copied());
            labels_data.extend(self.labels.get(index).unwrap().array().iter().copied());
        }

        let mut features = device.zeros();
        features.copy_from(&features_data);
        let mut labels = device.zeros();
        labels.copy_from(&labels_data);
        (features, labels)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::placement::{Orientation, Placement};

    #[test]
    fn test_get_features_for_placement() {
        let placement = Placement {
            square: Square::<7>::new(0, 0),
            piece: 1,
            orientation: Orientation {
                rotation: 0,
                flip: false,
            },
        };

        let features = EdgeStrategy::get_features_for_placement(placement);
        assert_eq!(
            features,
            [0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
        );
    }
}
