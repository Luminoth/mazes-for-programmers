pub mod aldous_broder;
pub mod binarytree;
pub mod hunt_and_kill;
pub mod recursive_backtracker;
pub mod sidewinder;
pub mod wilsons;

use crate::Grid;

pub use aldous_broder::*;
pub use binarytree::*;
pub use hunt_and_kill::*;
pub use recursive_backtracker::*;
pub use sidewinder::*;
pub use wilsons::*;

/// All maze generators implement this trait
// TODO: use an enum instead of a trait
pub trait Generator {
    fn name(&self) -> &str;

    /// Generates a new grid-based maze
    fn generate(&self, rows: usize, cols: usize) -> Grid;
}

/// Generator that doesn't generate anything
#[derive(Debug, Default)]
pub struct NoneGenerator;

impl Generator for NoneGenerator {
    fn name(&self) -> &str {
        "None"
    }

    fn generate(&self, rows: usize, cols: usize) -> Grid {
        Grid::new(rows, cols)
    }
}
