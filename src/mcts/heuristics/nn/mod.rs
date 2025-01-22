use burn::nn::{Linear, LinearConfig};
use burn::prelude::{Backend, Module, Tensor};

mod conv;
pub mod data;
mod linear;

use conv::ConvBlock;
use linear::LinearBlock;

#[derive(Module, Debug)]
pub struct CustomModel<B: Backend> {
    conv_block1: ConvBlock<B>,
    conv_block2: ConvBlock<B>,
    linear_block1: LinearBlock<B>,
    output_layer: Linear<B>,
}

impl<B: Backend> CustomModel<B> {
    pub fn init(device: &B::Device) -> Self {
        let input_b_size = 7;

        let conv_block1 = ConvBlock::init(7, 16, [3, 3], device);
        let conv_block2 = ConvBlock::init(16, 32, [3, 3], device);
        let linear_block1 = LinearBlock::init(32 * 5 * 5 + input_b_size, 64, device);
        let output_layer = LinearConfig::new(64, 1).init(device);

        Self {
            conv_block1,
            conv_block2,
            linear_block1,
            output_layer,
        }
    }

    pub fn forward(&self, input_a: Tensor<B, 4>, input_b: Tensor<B, 2>) -> Tensor<B, 2> {
        let [batch_size, _] = input_b.dims();
        let x = self.conv_block1.forward(input_a);
        let x = self.conv_block2.forward(x);
        let x = x.reshape([batch_size, 32 * 3]); // Flatten the tensor
        let x = Tensor::cat(vec![x, input_b], 1); // Concatenate along the feature dimension
        let x = self.linear_block1.forward(x);
        self.output_layer.forward(x)
    }
}
