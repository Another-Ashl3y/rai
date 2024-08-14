use neurons::Network;
mod neurons;

use rand::prelude::*;

fn main() {
    let n = Network::new(
        vec![
            (200 as usize, 1 as usize),
            (200 as usize, 200 as usize),
            (200 as usize, 200 as usize),
            (200 as usize, 200 as usize),
            (5 as usize, 200 as usize),
        ]
    );

    let mut rng = rand::thread_rng();

    loop {
        let test_data = vec![rng.gen()];
        let out = n.process(test_data);
        println!("{:?}", out);
    }
}