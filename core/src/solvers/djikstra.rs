use std::cell::RefCell;
use std::io;
use std::path::Path;

use crate::cell::CellHandle;
use crate::Grid;

use super::{distances, Distances, Solver};

/// Simple Djikstra's algorithm solver
#[derive(Debug)]
pub struct Djikstra {
    grid: Grid,
    root: CellHandle,

    distances: RefCell<Option<Distances>>,
}

impl Djikstra {
    pub fn new(grid: Grid, root_row: usize, root_column: usize) -> Self {
        Self {
            grid,
            root: CellHandle::new(root_row, root_column),
            distances: RefCell::new(None),
        }
    }
}

impl Solver for Djikstra {
    fn cell_contents(&self, row: usize, col: usize) -> String {
        if let Some(distances) = &*self.distances.borrow() {
            format!(
                "{}",
                radix_fmt::radix_36(
                    distances
                        .get_distance(CellHandle::new(row, col))
                        .unwrap_or_default()
                )
            )
        } else {
            String::from(" ")
        }
    }

    fn solve(&self) {
        *self.distances.borrow_mut() = Some(distances(&self.grid, self.root));

        // TODO: finish this
    }

    fn render_ascii(&self) {
        self.grid.render_ascii_internal(Some(self));
    }

    fn save_png(&self, path: &Path, cell_size: usize) -> io::Result<()> {
        self.grid.save_png(path, cell_size)
    }
}
