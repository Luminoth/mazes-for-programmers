//use std::collections::hash_map::Keys;
use std::collections::HashMap;

use crate::CellHandle;
use crate::Grid;

#[derive(Debug)]
pub struct Distances {
    root: CellHandle,
    cells: HashMap<CellHandle, usize>,
}

impl Distances {
    pub fn new(root: CellHandle) -> Self {
        let mut cells = HashMap::new();
        cells.insert(root, 0);

        Self { root, cells }
    }

    pub fn contains(&self, cell: &CellHandle) -> bool {
        self.cells.contains_key(cell)
    }

    pub fn get_distance(&self, cell: &CellHandle) -> Option<usize> {
        self.cells.get(cell).copied()
    }

    pub fn set_distance(&mut self, cell: CellHandle, distance: usize) {
        self.cells.insert(cell, distance);
    }

    pub fn max_distance(&self) -> (CellHandle, usize) {
        let mut max_cell = self.root;
        let mut max_distance = 0;

        for (cell, distance) in &self.cells {
            if *distance > max_distance {
                max_cell = *cell;
                max_distance = *distance;
            }
        }

        (max_cell, max_distance)
    }

    /*pub fn cells(&self) -> Keys<'_, CellHandle, usize> {
        self.cells.keys()
    }*/
}

/// Computes the distance from the root cell to every other cell
pub fn distances(grid: &Grid, root: CellHandle) -> Distances {
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
