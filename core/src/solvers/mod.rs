pub mod djikstra;

//use std::collections::hash_map::Keys;
use std::collections::HashMap;
use std::io;
use std::path::Path;

use crate::cell::*;
use crate::grid::*;

pub use djikstra::*;

/// Computes the distance from the root cell to every other cell
pub(crate) fn distances(grid: &Grid, root: CellHandle) -> Distances {
    let mut distances = Distances::new(root);
    let mut frontier = vec![root];

    while !frontier.is_empty() {
        let mut new_frontier = Vec::new();

        for cell_handle in frontier {
            let cell = grid.get(cell_handle.row, cell_handle.col).unwrap();

            // visit all of the cells this cell is linked (has a path) to
            for linked in cell.links() {
                // don't revisit cells
                if distances.contains(linked) {
                    continue;
                }

                distances.set_distance(*linked, distances.get_distance(&cell_handle).unwrap() + 1);
                new_frontier.push(*linked);
            }
        }

        frontier = new_frontier;
    }

    distances
}

/// All maze solvers implement this trait
pub trait Solver {
    fn grid(&self) -> &Grid;

    /// Returns the solver-based contents of the given cell
    fn cell_contents(&self, _row: usize, _col: usize) -> String {
        String::from(" ")
    }

    /// Solves the maze
    fn solve(&self, goal_row: usize, goal_col: usize);

    /// Renders the solved maze to the CLI
    fn render_ascii(&self);

    /// Saves the solved maze as a PNG at the given path
    fn save_png(&self, path: &Path, cell_size: usize) -> io::Result<()>;
}

#[derive(Debug, Default)]
pub struct Distances {
    //root: CellHandle,
    cells: HashMap<CellHandle, usize>,
}

impl Distances {
    pub(crate) fn new(root: CellHandle) -> Self {
        let mut cells = HashMap::new();
        cells.insert(root, 0);

        Self { /*root,*/ cells, }
    }

    pub(crate) fn contains(&self, cell: &CellHandle) -> bool {
        self.cells.contains_key(cell)
    }

    pub(crate) fn get_distance(&self, cell: &CellHandle) -> Option<usize> {
        self.cells.get(cell).copied()
    }

    pub(crate) fn set_distance(&mut self, cell: CellHandle, distance: usize) {
        self.cells.insert(cell, distance);
    }

    /*pub(crate) fn cells(&self) -> Keys<'_, CellHandle, usize> {
        self.cells.keys()
    }*/
}
