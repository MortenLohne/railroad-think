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

use crate::{
    board::{self, placement::Placement, square::Square, BOARD_SIZE},
    game::mv::Move,
    mcts::heuristics,
    pieces::{Connection, Piece},
};
use indicatif::ProgressBar;
use std::{io::Read, time::Instant};

use dfdx::{data::IteratorBatchExt, losses::mse_loss, optim::Adam, prelude::*, tensor::Cpu};
use rand::prelude::*;
use rand::Rng;
const MODEL_PATH: &str = "./src/mcts/heuristics/nn";

// #[cfg(feature = "nightly")]
// type Model = (
//     (Conv2D<7, 3, 3, 1, 1>, ReLU, Flatten2D),
//     (Linear<147, 1>, ReLU),
// );

// #[cfg(not(feature = "nightly"))]
type Model = (
    // Linear<588, 16>,
    Linear<595, 16>,
    ReLU,
    Linear<16, 16>,
    ReLU,
    Linear<16, 1>,
    ReLU,
);

// type ConvModel = (
//     Conv2D<7, 3, 3>,
//     Tanh,
//     Conv2D<3, 3, 3>,
//     Flatten2D,
//     Linear<147, 16>,
//     ReLU,
// );

// type HeuristicsModel = (Linear<7, 16>, ReLU, Linear<16, 16>, ReLU);

type F = f32;
type DS = Cpu;

mod builder {
    use super::*;

    pub struct ModelTwo;

    impl BuildOnDevice<DS, F> for ModelTwo {
        type Built = super::ModelTwo<F, DS>;
    }
}

pub struct ModelTwo<F: Dtype, DS: Device<F>> {
    convolutional: (
        modules::Conv2D<7, 3, 3, 1, 0, 0, 1, F, DS>,
        ReLU,
        modules::Conv2D<3, 3, 3, 1, 0, 0, 1, F, DS>,
        Flatten2D,
        modules::Linear<147, 16, F, DS>,
        ReLU,
    ),
    heuristic: (modules::Linear<7, 16, F, DS>, ReLU),
    shared: modules::Linear<32, 1, F, DS>,
}

impl TensorCollection<F, DS> for ModelTwo<F, DS> {
    type To<F2: Dtype, DS2: Device<F2>> = ModelTwo<F2, DS2>;

    fn iter_tensors<V: ModuleVisitor<Self, F, DS>>(
        visitor: &mut V,
    ) -> Result<Option<Self::To<V::E2, V::D2>>, V::Err> {
        todo!();
        // visitor.visit_fields(
        //     Self::module(
        //         "0", //
        //         |s| &s.shared,
        //         |s| &mut s.shared,
        //     ),
        //     builder::ModelTwo,
        // )
    }
}

// also impl Module as well as batched Module and ModuleMut
impl<T: Tape<F, DS>> ModuleMut<Tensor<Rank1<588>, F, DS, T>> for ModelTwo<F, DS> {
    type Output = Tensor<Rank1<1>, F, DS, T>;
    type Error = <DS as HasErr>::Err;

    fn try_forward_mut(
        &mut self,
        x: Tensor<Rank1<588>, F, DS, T>,
    ) -> Result<Self::Output, Self::Error> {
        let conv_in: Tensor<Rank1<588>, _, _> = x.copy_into().slice((0, 588)).try_realize()?;
        let conv_in: Tensor<Rank3<7, 7, 12>, _, _> = conv_in.try_reshape()?;
        let heuristics_in = x
            .slice((588, 595))
            .try_realize::<Tensor<Rank1<7>, _, _>>()?;

        let x1: Tensor<Rank1<16>, _, _> = self.convolutional.try_forward_mut(conv_in)?;
        let x2: Tensor<Rank1<16>, _, _> = self.heuristic.try_forward_mut(heuristics_in)?;
        let x: Tensor<Rank1<32>, _, _> = (x1, x2).try_concat_along(Axis::<0>)?;
        let x = self.shared.try_forward_mut(x)?;
        Ok(x)
    }
}

impl Module<Tensor<Rank1<512>, F, DS>> for ModelTwo<F, DS> {
    type Output = Tensor<Rank1<512>, F, DS>;
    type Error = <DS as HasErr>::Err;

    fn try_forward(&self, x: Tensor<Rank1<512>, F, DS>) -> Result<Self::Output, Self::Error> {
        Ok(x)
    }
}

// type Model = (ConvModel, TryConcat<HeuristicsModel>, Linear<32, 1>);
// type Output = (Linear<32, 1>, ReLU);
// struct Concat {
//     conv_model: ConvModel,
//     heuristics_model: HeuristicsModel,
//     output: Output,
// }

//  impl trait BuildOnDevice:
// impl BuildOnDevice<Cpu, f32> for Concat {
//     type Built = BuildModule<Cpu, f32>;

//     fn build_on_device(device: &Cpu) -> Self {
//         let conv_model = ConvModel::build_on_device(device);
//         let heuristics_model = HeuristicsModel::build_on_device(device);
//         let output = Output::build_on_device(device);
//         Self {
//             conv_model,
//             heuristics_model,
//             output,
//         }
//     }
// }

// impl Module<(Tensor3D<7, 7, 12>, Tensor1D<7>)> for Concat {
//     type Output = Rank1<1>;
//     type Error = Option<usize>;

//     fn try_forward(
//         &self,
//         (c, h): (Tensor3D<7, 7, 12>, Tensor1D<7>),
//     ) -> Result<Self::Output, Self::Error> {
//         let c = self.conv_model.forward(c);
//         let h = self.heuristics_model.forward(h);
//         let x = (c, h).concat_along(Axis::<0>);
//         let x = self.output.forward(x);
//         Ok(x)
//     }
// }

// #[derive(Debug, Default, Clone)]
// pub struct AddInto<T>(pub T);

// impl<T: BuildOnDevice<D, E>, D: Device<E>, E: Dtype> BuildOnDevice<D, E> for AddInto<T> {
//     type Built = AddInto<T::Built>;
// }

// impl<E: Dtype, D: Device<E>, T: TensorCollection<E, D>> TensorCollection<E, D> for AddInto<T> {
//     type To<E2: Dtype, D2: Device<E2>> = AddInto<T::To<E2, D2>>;

//     fn iter_tensors<V: ModuleVisitor<Self, E, D>>(
//         visitor: &mut V,
//     ) -> Result<Option<Self::To<V::E2, V::D2>>, V::Err> {
//         visitor.visit_fields(
//             Self::module(
//                 "0", //
//                 |s| &s.0,
//                 |s| &mut s.0,
//             ),
//             AddInto,
//         )
//     }
// }

// struct SomeModule;

// impl Module<(TupleType1, TupleType2)> for SomeModule {
//     type Output = ModuleOutput;
//     fn try_forward(&self, (t1, t2): (TupleType1, TupleType2)) -> Result<Self::Output, Error> {
//         // logic here
//         Ok(ModuleOutput)
//     }
// }

type BuildModel = (
    modules::Linear<595, 16, f32, Cpu>,
    ReLU,
    modules::Linear<16, 16, f32, Cpu>,
    ReLU,
    modules::Linear<16, 1, f32, Cpu>,
    ReLU,
);

const BATCH_SIZE: usize = 1024;

#[derive(Clone)]
pub struct EdgeStrategy {
    model: BuildModel,
    device: Cpu,
}

impl EdgeStrategy {
    #[must_use]
    pub fn create_model() -> Self {
        let device: Cpu = Cpu::default();
        // let mut model = Model::build_on_device(&device);
        let mut model = Model::build_on_device(&device);
        model.reset_params();
        Self { model, device }
    }

    /// # Panics
    /// If the model cannot be saved
    #[must_use]
    pub fn load(model_name: &str) -> Self {
        let device: Cpu = Cpu::default();
        let mut model = Model::build_on_device(&device);
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
        self.train_model_path("model", 5000);
    }

    /// # Panics
    /// If the model cannot be saved
    pub fn train_model_path(&mut self, model_path: &str, epochs: u64) {
        let device: Cpu = Cpu::default();

        let mut optimizer: Adam<BuildModel, f32, Cpu> =
            dfdx::optim::Adam::new(&self.model, Default::default());
        let mut grads = self.model.alloc_grads();
        let dataset = Dataset::load(&device);

        let total_bar = ProgressBar::new(epochs);
        for i_epoch in 0..epochs {
            total_bar.inc(1);
            let mut total_epoch_loss = 0.0;
            let mut num_batches = 0;
            let start = Instant::now();
            let feature_count = dataset.features.len();
            // let bar = ProgressBar::new(feature_count as u64);
            let subsets = (0..feature_count).batch_exact(Const::<BATCH_SIZE>);

            for (features, labels) in
                subsets.map(|indices| dataset.get_batch::<BATCH_SIZE>(&device, indices))
            {
                let pred = self.model.forward_mut(features.trace(grads));
                let loss = mse_loss(pred, labels.clone());

                total_epoch_loss += loss.array();
                num_batches += 1;
                // bar.inc(BATCH_SIZE as u64);

                grads = loss.backward();
                optimizer.update(&mut self.model, &grads).unwrap();
                self.model.zero_grads(&mut grads);
            }
            let dur = start.elapsed();

            // println!(
            //     "Epoch {i_epoch:03} in {:.2} ms ({:.2} batches/s): avg sample loss {:.3}",
            //     dur.as_millis(),
            //     num_batches as f32 / dur.as_secs_f32(),
            //     total_epoch_loss / num_batches as f32,
            // );

            total_bar.println(format!(
                "Epoch {i_epoch:03} in {:.2} ms ({:.2} batches/s): avg sample loss {:.3}",
                dur.as_millis(),
                num_batches as f32 / dur.as_secs_f32(),
                total_epoch_loss / num_batches as f32,
            ));

            if (i_epoch + 1) % 250 == 0 || i_epoch == epochs - 1 {
                println!("saving");
                self.model
                    .save(format!("{MODEL_PATH}/{model_path}.npz"))
                    .expect("failed to save model");
            }
        }
        total_bar.finish_and_clear();
    }

    // #[cfg(feature = "nightly")]
    // fn get_features(board: &board::Board, mv: Move, device: &Device) -> Tensor<Rank3<7, 7, 7>> {
    //     let mut features = device.zeros();
    //     let mut data = [0.0; 7 * 7 * 7];
    //     for y in 0..board::BOARD_SIZE {
    //         for x in 0..board::BOARD_SIZE {
    //             let ft = board[&Square::<7>::new(x, y)]
    //                 .map_or([0.0; 7], Self::get_features_for_placement);
    //             let x = x as usize;
    //             let y = y as usize;
    //             for i in 0..ft.len() {
    //                 data[y * 7 * 7 + x * 7 + i] = ft[i];
    //             }
    //         }
    //     }

    //     if let Move::Place(placement) = mv {
    //         let square = placement.square;
    //         let ft = Self::get_features_for_placement(placement);
    //         let x = square.x() as usize;
    //         let y = square.y() as usize;
    //         let start = y * 7 * 7 + x * 7;

    //         for i in 0..ft.len() {
    //             data[start + i] = ft[i];
    //         }

    //         for i in 0..4 {
    //             data[start + i * 3 + 2] = 1.0;
    //         }
    //     }
    //     features.copy_from(&data);
    //     features
    // }

    // #[cfg(not(feature = "nightly"))]
    /// This function returns an array of features for a placement.
    /// The array is structured as follows:
    /// 0–588: placements on the board
    /// 588–595: heuristics
    fn get_features(board: &board::Board, mv: Move, device: &Cpu) -> Tensor<Rank1<595>, f32, Cpu> {
        let mut features = device.zeros();
        // let board_feature_count = 12 * 7 * 7;
        // let heuristics_feature_count = 7;

        let mut data = [0.0; 595];
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

            data[588..595].copy_from_slice(&[
                board.piece_connects_to_exit(placement) as i32 as f32,
                board.piece_count_connections(placement) as f32 / 4.0,
                board.piece_locks_out_other_piece(placement) as i32 as f32,
                board.piece_is_2nd_order_neighbor(placement) as i32 as f32,
                board.piece_is_3rd_order_neighbor(placement) as i32 as f32,
                Piece::is_optional(placement.piece) as i32 as f32,
                board.count_placed() as f32 / (BOARD_SIZE * BOARD_SIZE) as f32,
            ]);
        }

        features.copy_from(&data);
        features
    }

    /// This function returns a 12-length array of features for a placement.
    /// The array is structured as follows:
    /// flat( [north, east, south, west] * [road, rail, current] )
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
    // #[cfg(feature = "nightly")]
    // pub features: Vec<Tensor<Rank3<7, 7, 7>>>,
    // #[cfg(not(feature = "nightly"))]
    pub features: Vec<Tensor<Rank1<595>, f32, Cpu>>,
    pub labels: Vec<Tensor<Rank1<1>, f32, Cpu>>,
}

impl Dataset {
    pub fn load(device: &Cpu) -> Self {
        let mut boards = Vec::new();
        let mut moves = Vec::new();
        let mut scores = Vec::new();

        let mut file = std::fs::File::open("./src/mcts/heuristics/nn/training_data.csv").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let total = contents.lines().count();
        let mut rng = thread_rng();

        for (i, line) in contents.lines().enumerate() {
            // This is a way to sample the data
            // so that lines later in the file are more likely to be included
            let line_pct = i as f32 / total as f32;

            if rng.gen::<f32>() > line_pct.powi(2) {
                continue;
            }

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

        let labels: Vec<Tensor<Rank1<1>, f32, Cpu>> = scores
            .into_iter()
            .map(|score| device.tensor([score]))
            .collect();

        Self { features, labels }
    }

    // #[cfg(feature = "nightly")]
    // pub fn get_batch<const B: usize>(
    //     &self,
    //     device: &Device,
    //     indices: [usize; B],
    // ) -> (Tensor<Rank4<B, 7, 7, 7>>, Tensor<Rank2<B, 1>>) {
    //     let mut features_data = Vec::with_capacity(B * 7 * 7 * 7);
    //     let mut labels_data = Vec::with_capacity(B * 1);

    //     for &index in indices.iter() {
    //         features_data.extend(
    //             self.features
    //                 .get(index)
    //                 .unwrap()
    //                 .array()
    //                 .iter()
    //                 .flatten()
    //                 .flatten()
    //                 .copied(),
    //         );
    //         labels_data.extend(self.labels.get(index).unwrap().array().iter().copied());
    //     }

    //     let mut features = device.zeros();
    //     features.copy_from(&features_data);
    //     let mut labels = device.zeros();
    //     labels.copy_from(&labels_data);
    //     (features, labels)
    // }

    // #[cfg(not(feature = "nightly"))]
    pub fn get_batch<const B: usize>(
        &self,
        device: &Cpu,
        indices: [usize; B],
    ) -> (
        Tensor<Rank2<B, 595>, f32, Cpu>,
        Tensor<Rank2<B, 1>, f32, Cpu>,
    ) {
        let mut features_data = Vec::with_capacity(B * 595);
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
