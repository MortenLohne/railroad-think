use burn::nn::{Gelu, Linear, LinearConfig};
use burn::prelude::{Backend, Module, Tensor};

#[derive(Module, Debug)]
pub struct LinearBlock<B: Backend> {
    linear: Linear<B>,
    activation: Gelu,
}

impl<B: Backend> LinearBlock<B> {
    pub fn init(in_features: usize, out_features: usize, device: &B::Device) -> Self {
        Self {
            linear: LinearConfig::new(in_features, out_features).init(device),
            activation: Gelu::new(),
        }
    }

    pub fn forward(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        let x = self.linear.forward(input);
        self.activation.forward(x)
    }
}
