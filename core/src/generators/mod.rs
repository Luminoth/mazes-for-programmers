pub mod binarytree;
pub mod sidewinder;

use crate::Grid;

pub use binarytree::*;
pub use sidewinder::*;

/// All maze generators implement this trait
pub trait Generator {
    /// Generates a new grid-based maze
    fn generate(&self, rows: usize, cols: usize) -> Grid;
}
