pub mod binarytree;
pub mod sidewinder;

use crate::grid::Grid;

pub use binarytree::*;
pub use sidewinder::*;

pub trait Generator {
    fn generate(&self, grid: &mut Grid);
}
