// #![allow(dead_code)]
extern crate nalgebra as na;

use na::{DMatrix, DVector};
use rand::Rng;

type ValT = i64;

pub struct Network {
    input_size: usize,
    pub weights: DMatrix<ValT>,
}

/// Convenience construtors for networks.
impl Network {
    fn new(input_size: usize, weights: DMatrix<ValT>) -> Network {
        Network {
            input_size,
            weights,
        }
    }

    pub fn with_random_weights(input_size: usize) -> Network {
        let mut rng = rand::thread_rng();
        let weights = DMatrix::from_fn(input_size, input_size, |i, j| {
            if i == j {
                0
            } else {
                rng.gen_range(-1, 1)
            }
        });
        Network::new(input_size, weights)
    }
}

/// Implementation of the kronecker delta function.
fn kronecker(i: usize, j: usize) -> ValT {
    if i == j {
        1
    } else {
        0
    }
}

/// Shuffle elements in a vector.
// FIXME: Remove the deprecated shuffle function.
fn shuffle(input: &mut Vec<usize>) {
    let mut rng = rand::thread_rng();
    rng.shuffle(input);
}

impl Network {
    /// Remember the inputs
    pub fn write(&mut self, wzorce: &Vec<&DVector<ValT>>) {
        for i in 0..self.input_size {
            for j in 0..self.input_size {
                let mut sum = 0;
                for s in wzorce.iter() {
                    sum += (1 - kronecker(i, j)) * s[i] * s[j];
                }
                self.weights[(i, j)] = sum;
            }
        }
    }

    /// Calculate the output of the network.
    pub fn read(&self, input: &DVector<ValT>) -> DVector<ValT> {
        let mut output = DVector::from_element(self.input_size, 0);
        let mut neuron_list = (0..self.input_size).collect::<Vec<usize>>();
        shuffle(&mut neuron_list);

        for neuron_idx in neuron_list {
            
            let mut sum = 0;
            for input_idx in 0..self.input_size {
                sum += self.weights[(input_idx, neuron_idx)] * input[input_idx];
            }

            output[neuron_idx] = match sum {
                x if x > 0 => 1,
                x if x < 0 => -1,
                _ => input[neuron_idx],
            };
            // break;
        }
        output
    }
}
