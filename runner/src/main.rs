mod options;

use options::Options;

use core::algorithms::Algorithm;
use core::Grid;

fn main() {
    let options: Options = argh::from_env();

    println!("Generating {}x{} grid ...", options.width, options.height);
    let mut grid = Grid::new(options.width, options.height);

    println!("Running algorithm: {:?} ...", options.algorithm);
    let algorithm = options.algorithm.algorithm();
    algorithm.run(&mut grid);
}
