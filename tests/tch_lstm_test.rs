use ndarray_rand::rand_distr::num_traits;
use num_traits::{One, Zero};
use std::ops::{Add, Sub};
use tch::Tensor;

pub struct LSTM {
    pub input_gate: Tensor,
    pub forget_gate: Tensor,
    pub output_gate: Tensor,
    pub hidden: Tensor,
}

impl LSTM {
    pub fn new(input_size: usize, hidden_size: usize) -> Self {
        let input_gate = Tensor::new(); //(input_size, hidden_size).unwrap();
        let forget_gate = Tensor::new(); //Tensor::new(hidden_size, hidden_size).unwrap();
        let output_gate = Tensor::new(); //Tensor::new(hidden_size, hidden_size).unwrap();
        let hidden = Tensor::new(); //Tensor::new(hidden_size, 1).unwrap();

        Self {
            input_gate,
            forget_gate,
            output_gate,
            hidden,
        }
    }

    pub fn step(&mut self, input: &Tensor, hidden: &Tensor) {
        // 计算输入门
        let mut input_gate = self.input_gate;
        input_gate.add_out(&hidden, 1.0);
        input_gate.sigmoid();

        // 计算遗忘门
        let mut forget_gate = self.forget_gate;
        forget_gate.add_mat(&hidden, 1.0);
        forget_gate.sigmoid();

        // 更新隐藏状态
        let mut hidden = self.hidden;
        hidden.add_mat(&forget_gate, 1.0);
        hidden.add_mat(&input_gate, 1.0);
        hidden.tanh();

        // 计算输出门
        let mut output_gate = self.output_gate;
        output_gate.add_mat(&hidden, 1.0);
        output_gate.sigmoid();
    }
}

pub struct LSTMModel {
    pub input: Tensor,
    pub lstm: Vec<LSTM>,
    pub output: Tensor,
    pub loss: Tensor,
}

impl LSTMModel {
    pub fn new(input_size: usize, hidden_size: usize, output_size: usize) -> Self {
        let input = Tensor::new(input_size, 1).unwrap();
        let lstm = (0..hidden_size)
            .map(|_| LSTM::new(input_size, hidden_size).unwrap())
            .collect();
        let output = Tensor::new(output_size, 1).unwrap();
        let loss = Tensor::new(1, 1).unwrap();

        Self {
            input,
            lstm,
            output,
            loss,
        }
    }

    pub fn forward(&mut self, input: &Tensor) {
        for (i, lstm) in self.lstm.iter_mut().enumerate() {
            lstm.step(input, &self.lstm[i].hidden);
        }
        self.output.assign(self.lstm[self.lstm.len() - 1].hidden);
    }

    pub fn compute_loss(&self, target: &Tensor) {
        let mut loss = self.loss.clone();
        loss.square_diff(&target);
        loss.mean();
    }

    pub fn train(&mut self, input: &Tensor, target: &Tensor) {
        self.forward(input);
        self.compute_loss(target);

        // 这里可以添加优化器实例，如 ADAM，然后更新权重
        // 例如：
        // self.lstm[0].input_gate.gradient().backward(&self.input, 1.0);
    }
}
use rand::Rng;

fn main() {
    let input_size = 2;
    let hidden_size = 10;
    let output_size = 1;

    // 创建随机数据
    let mut rng = rand::thread_rng();
    let input_data = vec![
        Tensor::new(input_size, 1).unwrap().assign(rng.gen::<f64>().to_tensor());
    ];
    let target_data = vec![
        Tensor::new(output_size, 1).unwrap().assign(rng.gen::<f64>().to_tensor());
    ];

    // 创建 LSTM 模型
    let mut model = LSTMModel::new(input_size, hidden_size, output_size);

    // 训练模型
    for (input, target) in input_data.iter().zip(target_data.iter()) {
        model.train(input, target);
    }

    // 测试模型
    // 在这里，你可以使用测试数据集对模型进行预测并计算损失
}
