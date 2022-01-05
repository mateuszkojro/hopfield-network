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
        1, 1, 1, 1, 1, 1, -1, -1, -1, 1, 1, -1, -1, -1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1, 1, 1, -1,
        -1, -1, 1, 1, -1, -1, -1, 1
    ];

    let letter_z = dvector![
        1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1, 1, -1, -1, -1, -1, -1, 1, -1, -1, -1, -1, -1, 1, -1,
        -1, -1, -1, -1, 1, 1, 1, 1, 1, 1
    ];

    let letter_o = dvector![
        1, 1, 1, 1, 1, 1, -1, -1, -1, 1, 1, -1, -1, -1, 1, 1, -1, -1, -1, 1, 1, -1, -1, -1, 1, 1,
        -1, -1, -1, 1, 1, 1, 1, 1, 1
    ];

    let letter_actual_c = dvector![
        1, 1, 1, 1, 1, 1, -1, -1, -1, -1, 1, -1, -1, -1, -1, 1, -1, -1, -1, -1, 1, -1, -1, -1, -1,
        1, -1, -1, -1, -1, 1, 1, 1, 1, 1
    ];

    let letter_u = dvector![
        1, -1, -1, -1, 1, 1, -1, -1, -1, 1, 1, -1, -1, -1, 1, 1, -1, -1, -1, 1, 1, -1, -1, -1, 1,
        1, -1, -1, -1, 1, 1, 1, 1, 1, 1
    ];

    let magled_letter_o = dvector![
        1, -1, 1, -1, 1, 1, 1, -1, -1, 1, 1, -1, -1, -1, 1, 1, -1, -1, -1, 1, 1, -1, -1, -1, 1,
        1, -1, 1, -1, 1, 1, 1, 1, 1, 1
    ];


    let input_sz = &letter_a.len();

    let mut network = network::Network::with_random_weights(*input_sz);

    let input_imgs = vec![
        &letter_a,
        &letter_a,
        &letter_z,
        &letter_o,
        &letter_actual_c,
        &letter_u,
    ];
    network.write(&input_imgs);

    let mut last_output = DVector::from_element(*input_sz, rngs::StdRng::from_entropy().gen());
    let mut output = network.read(&magled_letter_o);

    let mut i = 0;
    while last_output != output {
        last_output = output.clone();
        output = network.read(&last_output);

        i += 1;
    }

    let out_dir = path::Path::new("output");
    if !out_dir.exists() {
        fs::create_dir(out_dir).unwrap();
    }

    input_imgs.into_iter().enumerate().for_each(|(idx, img)| {
        show(format!("output/{}.svg", idx).as_str(), img, 5, 7);
    });
    show("output/mangled_o.svg", &magled_letter_o, 5, 7);
    show("output/out.svg", &output, 5, 7);
}
