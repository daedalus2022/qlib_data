#[cfg(test)]
mod alstm {
    use tch::nn::OptimizerConfig;
    use tch::{
        kind,
        nn::{self, Module},
        Device, Tensor,
    };

    fn my_module(p: nn::Path, dim: i64) -> impl nn::Module {
        let x1 = p.zeros("x1", &[dim]);
        let x2 = p.zeros("x2", &[dim]);
        nn::func(move |xs| xs * &x1 + xs.exp() * &x2)
    }

    #[test]
    fn alstm_build_model_cuda_works() {
        let vs = nn::VarStore::new(Device::Cuda(0));
        let my_module = my_module(vs.root(), 7);
        let mut opt = nn::Sgd::default().build(&vs, 1e-2).unwrap();
        for _idx in 1..5000 {
            // Dummy mini-batches made of zeros.
            let xs = Tensor::zeros(&[7], kind::FLOAT_CUDA);
            let ys = Tensor::zeros(&[7], kind::FLOAT_CUDA);
            let loss = (my_module.forward(&xs) - ys)
                .pow_tensor_scalar(2)
                .sum(kind::Kind::Float);
            opt.backward_step(&loss);
        }
    }

    #[test]
    fn alstm_build_model_cpu_works() {
        let vs = nn::VarStore::new(Device::Cpu);
        let my_module = my_module(vs.root(), 7);
        let mut opt = nn::Sgd::default().build(&vs, 1e-2).unwrap();
        for _idx in 1..5000 {
            // Dummy mini-batches made of zeros.
            let xs = Tensor::zeros(&[7], kind::FLOAT_CPU);
            let ys = Tensor::zeros(&[7], kind::FLOAT_CPU);
            let loss = (my_module.forward(&xs) - ys)
                .pow_tensor_scalar(2)
                .sum(kind::Kind::Float);
            opt.backward_step(&loss);
        }
    }
}
