use linfa::prelude::Predict;
use linfa::prelude::SingleTargetRegression;
use linfa::traits::Fit;
use linfa_elasticnet::{ElasticNet, Result};

#[test]
fn test_() -> Result<()> {
    let (train, valid) = linfa_datasets::diabetes().split_with_ratio(0.9);

    // train pure LASSO model with 0.1 penalty
    let model = ElasticNet::params()
        .penalty(0.1)
        .l1_ratio(1.0)
        .fit(&train)?;

    println!("train:{:?}", train);

    println!("z score: {:?}", model.z_score());

    // validate
    let y_est = model.predict(&valid);
    println!("predicted variance: {}", y_est.r2(&valid)?);

    Ok(())
}

// #[derive(Debug)]
// struct LSTMParams {
//     weights_ih: Array2<f32>,
//     weights_hh: Array2<f32>,
//     bias_ih: Array2<f32>,
//     bias_hh: Array2<f32>,
// }

// impl LSTMParams {
//     fn new(input_size: usize, hidden_size: usize) -> Self {
//         let weights_ih = Array2::from_shape_fn((4 * hidden_size, input_size), |_| {
//             let mut rng = thread_rng();
//             rng.gen_range(0.0..1.0)
//         });
//         let weights_hh = Array2::from_shape_fn((4 * hidden_size, hidden_size), |_| {
//             let mut rng = thread_rng();
//             rng.gen_range(0.0..1.0)
//         });
//         let bias_ih = Array2::from_elem((4 * hidden_size, 1), 0.0);
//         let bias_hh = Array2::from_elem((4 * hidden_size, 1), 0.0);
//         LSTMParams {
//             weights_ih,
//             weights_hh,
//             bias_ih,
//             bias_hh,
//         }
//     }
// }

// #[derive(Debug)]
// struct LSTM {
//     params: LSTMParams,
//     hidden_size: usize,
// }

// impl LSTM {
//     fn new(input_size: usize, hidden_size: usize) -> Self {
//         LSTM {
//             params: LSTMParams::new(input_size, hidden_size),
//             hidden_size,
//         }
//     }

//     fn step(
//         &self,
//         x: &Array2<f32>,
//         h: &Array2<f32>,
//         c: &Array2<f32>,
//     ) -> (Array2<f32>, Array2<f32>) {
//         let xh = stack![Axis(0), x, h];
//         let gates = self.params.weights_ih.dot(&xh)
//             + self.params.weights_hh.dot(h)
//             + &self.params.bias_ih
//             + &self.params.bias_hh;
//         let i = sigmoid(gates.slice(s![..self.hidden_size, ..]));
//         let f = sigmoid(gates.slice(s![self.hidden_size..2 * self.hidden_size, ..]));
//         let g = tanh(gates.slice(s![2 * self.hidden_size..3 * self.hidden_size, ..]));
//         let o = sigmoid(gates.slice(s![3 * self.hidden_size.., ..]));
//         let c_new = f * c + i * g;
//         let h_new = o * tanh(&c_new);
//         (h_new, c_new)
//     }

//     fn forward(&self, input: &Array2<f32>) -> Array2<f32> {
//         let seq_len = input.shape()[0];
//         let batch_size = input.shape()[1];
//         let mut h = Array2::zeros((self.hidden_size, batch_size));
//         let mut c = Array2::zeros((self.hidden_size, batch_size));
//         let mut h_out = Array2::zeros((seq_len, self.hidden_size, batch_size));
//         for i in 0..seq_len {
//             let x = input.slice(s![i, .., ..]).into_owned();
//             let (h_new, c_new) = self.step(&x, &h, &c);
//             h = h_new;
//             c = c_new;
//             h_out.slice_mut(s![i, .., ..]).assign(&h_new.t().to_owned());
//         }
//         h_out
//     }
// }

// fn sigmoid(input: Array2<f32>) -> Array2<f32> {
//     (1.0 / (1.0 + (-input).mapv(|x| x.exp())))
// }

// fn tanh(input: Array2<f32>) -> Array2<f32> {
//     input.mapv(|x| x.tanh())
// }

// use linfa::prelude::*;
// use linfa_nn::*;

// fn main() {
//     let (train, valid) = prepare_data();
//     let input_size = train.nfeatures();
//     let hidden_size = 128;
//     let lstm = LSTM::new(input_size, hidden_size);
//     let (X_train, y_train) = extract(input_size, train);
//     let h_train = lstm.forward(&X_train);
//     let (X_valid, y_valid) = extract(input_size, valid);
//     let h_valid = lstm.forward(&X_valid);
//     train_and_evaluate(h_train, y_train, h_valid, y_valid);
// }

// fn prepare_data() -> (Dataset<f32>, Dataset<f32>) {
//     let ds = iris();

//     // load MNIST data
//     let (train, valid, _test) = (ds.nsamples(), ds.nfeatures(), ds.ntargets());

//     let train = train
//         .map_targets(|y| [y, 1.0 - y])
//         .unsafe_relayout(Axis(0))
//         .to_owned();
//     let valid = valid
//         .map_targets(|y| [y, 1.0 - y])
//         .unsafe_relayout(Axis(0))
//         .to_owned();

//     (train, valid)
// }

// fn extract(input_size: usize, dataset: Dataset<f32>) -> (Array2<f32>, Array2<f32>) {
//     let inputs = dataset
//         .records()
//         .to_owned()
//         .restyle((dataset.nsamples(), 28, 28))
//         .unwrap();
//     let outputs = dataset.targets().to_owned();
//     let seq_len = inputs.shape()[0];
//     let batch_size = inputs.shape()[1];
//     let inputs = inputs
//         .into_shape((seq_len, batch_size, input_size))
//         .unwrap()
//         .permuted_axes([0, 2, 1])
//         .to_owned();
//     let outputs = outputs.reshape((dataset.nsamples(), 2)).to_owned();
//     (inputs, outputs)
// }

// fn train_and_evaluate(
//     X_train: Array2<f32>,
//     y_train: Array2<f32>,
//     X_valid: Array2<f32>,
//     y_valid: Array2<f32>,
// ) {
//     let n_classes = y_train.shape()[1];
//     let mut classifier = LogisticRegression::params(n_classes)
//         .max_iterations(30)
//         .fit(&X_train, &y_train)
//         .unwrap();
//     let y_hat_train = classifier
//         .predict(&X_train)
//         .unwrap()
//         .expect("Failed to make predictions on train set");
//     let y_hat_valid = classifier
//         .predict(&X_valid)
//         .unwrap()
//         .expect("Failed to make predictions on validation set");

//     let train_accuracy = evaluate_accuracy(&y_train, &y_hat_train);
//     let valid_accuracy = evaluate_accuracy(&y_valid, &y_hat_valid);
//     println!(
//         "Train accuracy: {:.2}%, Validation accuracy: {:.2}%",
//         train_accuracy * 100.0_f32,
//         valid_accuracy * 100.0_f32
//     );
// }

// fn evaluate_accuracy(y_true: &Array2<f32>, y_pred: &Array2<f32>) -> f32 {
//     let n_samples = y_true.shape()[0];
//     let mut n_correct = 0;
//     for i in 0..n_samples {
//         let true_label = y_true.slice(s![i, ..]);
//         let pred_label = y_pred.slice(s![i, ..]);
//         if (true_label - pred_label).iter().all(|x| x.abs() < 1e-6) {
//             n_correct += 1;
//         }
//     }
//     n_correct as f32 / n_samples as f32
// }
