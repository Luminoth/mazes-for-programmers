pub mod aldous_broder;
pub mod binarytree;
pub mod sidewinder;

use crate::Grid;

pub use aldous_broder::*;
pub use binarytree::*;
pub use sidewinder::*;

/// All maze generators implement this trait
// TODO: use an enum instead of a trait
pub trait Generator {
    /// Generates a new grid-based maze
    fn generate(&self, rows: usize, cols: usize) -> Grid;
}
