pub mod djikstra;

use std::io;
use std::path::Path;

use crate::util::Color;
use crate::{Grid, Renderable};

pub use djikstra::*;

/// All maze solvers implement this trait
// TODO: use an enum instead of a trait
pub trait Solver: Renderable {
    fn name(&self) -> &str;

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

/// Solver that doesn't solve anything
#[derive(Debug)]
pub struct NoneSolver {
    grid: Grid,
}

impl NoneSolver {
    pub fn new(grid: Grid) -> Self {
        Self { grid }
    }
}

impl Solver for NoneSolver {
    fn name(&self) -> &str {
        "None"
    }

    fn grid(&self) -> &Grid {
        &self.grid
    }

    fn solve(&self, _goal_row: usize, _goal_col: usize) {}
}

impl Renderable for NoneSolver {
    fn render_ascii(&self) -> String {
        self.grid.render_ascii()
    }

    fn save_png(&self, path: &Path, cell_size: usize) -> io::Result<()> {
        self.grid.save_png(path, cell_size)
    }
}
