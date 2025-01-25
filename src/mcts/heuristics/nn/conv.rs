use burn::nn::conv::{Conv2d, Conv2dConfig};
// use burn::nn::pool::{MaxPool2d, MaxPool2dConfig};
use burn::nn::{BatchNorm, BatchNormConfig, Gelu};
use burn::prelude::{Backend, Module, Tensor};

#[derive(Module, Debug)]
pub struct ConvBlock<B: Backend> {
    conv: Conv2d<B>,
    // pool: AvgPool2d,
    norm: BatchNorm<B, 2>,
    activation: Gelu,
}

impl<B: Backend> ConvBlock<B> {
    pub fn init(
        in_channels: usize,
        out_channels: usize,
        kernel_size: [usize; 2],
        device: &B::Device,
    ) -> Self {
        let conv = Conv2dConfig::new([in_channels, out_channels], kernel_size).init(device);
        // let pool = MaxPool2dConfig::new(kernel_size).init();
        let norm = BatchNormConfig::new(in_channels).init(device);
        Self {
            conv,
            norm,
            activation: Gelu::new(),
        }
    }

    pub fn forward(&self, input: Tensor<B, 4>) -> Tensor<B, 4> {
        let x = self.conv.forward(input);
        let x = self.norm.forward(x);
        self.activation.forward(x)
    }
}
