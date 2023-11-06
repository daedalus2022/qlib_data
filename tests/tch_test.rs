#[cfg(test)]
mod alstm {
    use tch::nn::OptimizerConfig;
    use tch::{kind, nn::{self, Module}, Device, Tensor, Kind};

    fn my_module(p: nn::Path, dim: i64) -> impl nn::Module {
        let x1 = p.zeros("x1", &[dim]);
        let x2 = p.zeros("x2", &[dim]);
        nn::func(move |xs| xs * &x1 + xs.exp() * &x2)
    }

    fn normalize_data(input: &Tensor) -> Tensor {
        let mean = input.mean(Kind::Float);
        let std = input.std(false);

        let normalized_input = input - mean;
        normalized_input / std
    }

    #[test]
    fn normalize_data_works(){
        // 加载数据，这里以一个简单的示例为例，实际应用中，您需要根据实际情况加载数据
        let data = vec![
            Tensor::from_slice(&vec![1.0, 2.0, 3.0, 4.0, 5.0]),
            Tensor::from_slice(&vec![6.0, 7.0, 8.0, 9.0, 10.0]),
        ];

        // 数据标准化
        let normalized_data = data.iter().map(|tensor| normalize_data(tensor)).collect::<Vec<_>>();

        // 打印标准化后的数据
        for (i, tensor) in normalized_data.iter().enumerate() {
            println!("Data normalized {}: {}", i, tensor);
        }
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
