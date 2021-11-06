pub mod binarytree;
pub mod sidewinder;

use crate::grid::Grid;

pub use binarytree::*;
pub use sidewinder::*;

pub trait Algorithm {
    fn run(&self, grid: &mut Grid);
}
