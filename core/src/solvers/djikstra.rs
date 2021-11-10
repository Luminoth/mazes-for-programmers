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
    path: RefCell<Option<Distances>>,
}

impl Djikstra {
    pub fn new(grid: Grid, root_row: usize, root_column: usize) -> Self {
        Self {
            grid,
            root: CellHandle::new(root_row, root_column),
            distances: RefCell::new(None),
            path: RefCell::new(None),
        }
    }

    fn cell_contents_from_distances(distances: &Distances, cell: CellHandle) -> String {
        let distance = distances.get_distance(&cell);
        if let Some(distance) = distance {
            format!("{}", radix_fmt::radix_36(distance))
        } else {
            String::from(" ")
        }
    }

    fn path_to(&self, goal: CellHandle) -> Distances {
        let distances = self.distances.borrow();
        {
            let distances = distances.as_ref().unwrap();

            let mut current = goal;
            let mut current_distance = distances.get_distance(&current).unwrap_or_default();

            let mut breadcrumbs = Distances::new(self.root);
            breadcrumbs.set_distance(current, current_distance);

            while current != self.root {
                let cell = self.grid.get(current.row, current.col).unwrap();

                for neighbor in cell.links() {
                    let neighbor_distance = distances.get_distance(neighbor).unwrap_or_default();
                    if neighbor_distance < current_distance {
                        breadcrumbs.set_distance(*neighbor, neighbor_distance);

                        current = *neighbor;
                        current_distance = distances.get_distance(&current).unwrap_or_default();
                    }
                }
            }

            breadcrumbs
        }
    }
}

impl Solver for Djikstra {
    fn grid(&self) -> &Grid {
        &self.grid
    }

    fn cell_contents(&self, row: usize, col: usize) -> String {
        let cell = CellHandle::new(row, col);

        if let Some(path) = &*self.path.borrow() {
            Djikstra::cell_contents_from_distances(path, cell)
        } else if let Some(distances) = &*self.distances.borrow() {
            Djikstra::cell_contents_from_distances(distances, cell)
        } else {
            String::from(" ")
        }
    }

    fn solve(&self, goal_row: usize, goal_col: usize) {
        *self.distances.borrow_mut() = Some(distances(&self.grid, self.root));
        *self.path.borrow_mut() = Some(self.path_to(CellHandle::new(goal_row, goal_col)));
    }

    fn render_ascii(&self) {
        self.grid.render_ascii_internal(Some(self));
    }

    fn save_png(&self, path: &Path, cell_size: usize) -> io::Result<()> {
        self.grid.save_png(path, cell_size)
    }
}
