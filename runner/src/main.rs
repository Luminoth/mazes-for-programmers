mod options;

use options::{AlgorithmOption, Options};

use core::algorithms::{Algorithm, BinaryTree};
use core::Grid;

fn main() {
    let options: Options = argh::from_env();

    println!("Running algorithm: {:?}", options.algorithm);
    let algorithm = match options.algorithm {
        AlgorithmOption::BinaryTree(_) => BinaryTree::default(),
    };

    let mut grid = Grid::new(10, 10);
    algorithm.run(&mut grid);
}
