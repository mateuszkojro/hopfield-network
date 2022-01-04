mod network;
mod visualize;

use crate::visualize::show;

use nalgebra::{dvector, DVector};
use rand::{rngs, FromEntropy, Rng};

use std::{fs, io::Read, path};

fn read_img(path: &str) -> DVector<i64> {
    let mut file = fs::File::open(path).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let mut v = vec![];

    contents.split_ascii_whitespace().for_each(|x| {
        v.push(x.parse::<i64>().unwrap());
    });

    DVector::from_vec(v)
}

fn main() {
    let letter_a = dvector![
        1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1, 1, -1, -1, -1, -1, -1, 1, -1, -1, -1, -1, -1, 1, -1,
        -1, -1, -1, -1, 1, -1, -1, -1, -1, -1, -1, -1, -1, 1, -1, -1, -1, -1, -1, 1, 1, 1, 1, 1
    ];

    let letter_b = dvector![
        1, -1, 1, 1, 1, -1, -1, -1, -1, -1, -1, 1, -1, -1, -1, -1, -1, 1, -1, -1, -1, -1, -1, 1,
        -1, -1, -1, -1, -1, 1, -1, -1, -1, -1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1, 1, 1, 1, 1, 1
    ];
    let letter_c = dvector![
        1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1, 1, -1, -1, -1, -1, -1, 1, -1, -1, -1, -1, -1, 1, -1,
        1, 1, 1, -1, 1, -1, -1, -1, -1, -1, -1, -1, -1, 1, -1, -1, -1, -1, -1, 1, 1, 1, -1, -1
    ];

    let letter_d = dvector![
        1, -1, 1, 1, 1, -1, 1, 1, 1, 1, -1, 1, -1, -1, -1, -1, -1, 1, -1, -1, -1, -1, -1, 1, -1,
        -1, -1, -1, -1, 1, -1, -1, -1, -1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1, 1, 1, 1, 1, 1
    ];

    let input_sz = &letter_a.len();

    let mut network = network::Network::with_random_weights(*input_sz);

    let input_imgs = vec![&letter_a, &letter_b, &letter_c, &letter_d];
    network.write(&input_imgs);

    let mut last_output = DVector::from_element(*input_sz, rngs::StdRng::from_entropy().gen());
    let mut output = network.read(&letter_a);

    let mut i = 0;
    while last_output != output {
        last_output = output.clone();
        output = network.read(&last_output);

        eprintln!("[info] Iteration: {}", i);
        i += 1;
    }

    let out_dir = path::Path::new("output");
    if !out_dir.exists() {
        fs::create_dir(out_dir).unwrap();
    }

    input_imgs.into_iter().enumerate().for_each(|(idx, img)| {
        show(format!("output/{}.svg", idx).as_str(), img);
    });
    show("output/out.svg", &output);
}
