pub mod binarytree;

use crate::grid::Grid;

pub use binarytree::*;

pub trait Algorithm {
    fn run(&self, grid: &mut Grid);
}
