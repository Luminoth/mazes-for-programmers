pub mod djikstra;

use crate::util::Color;
use crate::{Grid, Renderable};

pub use djikstra::*;

/// All maze solvers implement this trait
// TODO: use an enum instead of a trait
pub trait Solver: Renderable {
    fn grid(&self) -> &Grid;

    /// Returns the solver-based contents of the given cell
    fn cell_contents(&self, _row: usize, _col: usize) -> String {
        let (_, empty) = self.grid().empty_cell_contents();
        empty
    }

    /// Returns the solver-based background color of the given cell
    fn cell_background(&self, _row: usize, _col: usize) -> Color {
        Color::WHITE
    }

    /// Solves the maze
    fn solve(&self, goal_row: usize, goal_col: usize);
}
