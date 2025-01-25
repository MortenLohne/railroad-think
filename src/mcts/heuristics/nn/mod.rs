use burn::nn::loss::{MseLoss, Reduction};
use burn::prelude::{Backend, Module, Tensor};

mod conv;
pub mod data;
mod linear;
pub mod training;

use burn::tensor::backend::AutodiffBackend;
use burn::train::{RegressionOutput, TrainOutput, TrainStep, ValidStep};
use conv::ConvBlock;
use data::DataBatch;
use linear::LinearBlock;

#[derive(Module, Debug)]
pub struct CustomModel<B: Backend> {
    conv_block1: ConvBlock<B>,
    conv_block2: ConvBlock<B>,
    linear_block1: LinearBlock<B>,
    output_block: LinearBlock<B>,
}

impl<B: Backend> CustomModel<B> {
    pub fn init(device: &B::Device) -> Self {
        let input_b_size = 7;

        let conv_block1 = ConvBlock::init(7, 7, [3, 3], device);
        let conv_block2 = ConvBlock::init(7, 7, [3, 3], device);
        let linear_block1 = LinearBlock::init(7 * 6 * 4 + input_b_size, 64, device);
        let output_block = LinearBlock::init(64, 1, device);

        Self {
            conv_block1,
            conv_block2,
            linear_block1,
            output_block,
        }
    }

    pub fn forward(&self, input_a: Tensor<B, 4>, input_b: Tensor<B, 2>) -> Tensor<B, 2> {
        let [batch_size, _] = input_b.dims();
        let x = self.conv_block1.forward(input_a);
        let x = self.conv_block2.forward(x);
        let [_, dim_x, dim_y, dim_z] = x.dims();
        let x = x.reshape([batch_size, dim_x * dim_y * dim_z]); // Flatten the tensor
        let x = Tensor::cat(vec![x, input_b], 1); // Concatenate along the feature dimension
        let x = self.linear_block1.forward(x);
        let x = self.output_block.forward(x);
        x
    }

    pub fn forward_step(&self, item: DataBatch<B>) -> RegressionOutput<B> {
        let targets: Tensor<B, 2> = item.targets.unsqueeze_dim(1);
        let output = self.forward(item.boards, item.heuristics);
        let loss = MseLoss::new().forward(output.clone(), targets.clone(), Reduction::Mean);

        RegressionOutput {
            loss,
            output,
            targets,
        }
    }
}

impl<B: AutodiffBackend> TrainStep<DataBatch<B>, RegressionOutput<B>> for CustomModel<B> {
    fn step(&self, item: DataBatch<B>) -> TrainOutput<RegressionOutput<B>> {
        let item = self.forward_step(item);

        TrainOutput::new(self, item.loss.backward(), item)
    }
}

impl<B: Backend> ValidStep<DataBatch<B>, RegressionOutput<B>> for CustomModel<B> {
    fn step(&self, item: DataBatch<B>) -> RegressionOutput<B> {
        self.forward_step(item)
    }
}
