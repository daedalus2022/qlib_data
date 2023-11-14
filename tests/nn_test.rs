use nn::{HaltCondition, NN};
#[test]
fn nn_example() {
    // create examples of the XOR function
    // the network is trained on tuples of vectors where the first vector
    // is the inputs and the second vector is the expected outputs
    let examples = [
        (vec![0f64, 0f64], vec![0f64]),
        (vec![0f64, 1f64], vec![1f64]),
        (vec![1f64, 0f64], vec![1f64]),
        (vec![1f64, 1f64], vec![0f64]),
    ];

    // create a new neural network by passing a pointer to an array
    // that specifies the number of layers and the number of nodes in each layer
    // in this case we have an input layer with 2 nodes, one hidden layer
    // with 3 nodes and the output layer has 1 node
    let mut net = NN::new(&[2, 3, 1]);

    // train the network on the examples of the XOR function
    // all methods seen here are optional except go() which must be called to begin training
    // see the documentation for the Trainer struct for more info on what each method does
    net.train(&examples)
        .halt_condition(HaltCondition::Epochs(10000))
        .log_interval(Some(100))
        .momentum(0.1)
        .rate(0.3)
        .go();

    // evaluate the network to see if it learned the XOR function
    for &(ref inputs, ref outputs) in examples.iter() {
        let results = net.run(inputs);
        let (result, key) = (results[0].round(), outputs[0]);
        assert!(result == key);
    }
}
