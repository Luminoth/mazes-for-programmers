pub mod aldous_broder;
pub mod binarytree;
pub mod sidewinder;
pub mod wilsons;

use crate::Grid;

pub use aldous_broder::*;
pub use binarytree::*;
pub use sidewinder::*;
pub use wilsons::*;

/// All maze generators implement this trait
// TODO: use an enum instead of a trait
pub trait Generator {
    /// Generates a new grid-based maze
    fn generate(&self, rows: usize, cols: usize) -> Grid;
}
