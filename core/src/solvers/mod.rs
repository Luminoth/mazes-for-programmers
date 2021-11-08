pub mod djikstra;

use std::collections::hash_map::Keys;
use std::collections::HashMap;

use crate::cell::*;
use crate::grid::*;

pub trait Solver {
    fn solve(&self, grid: &mut Grid);
}

pub struct Distances {
    root: CellHandle,
    cells: HashMap<CellHandle, usize>,
}

impl Distances {
    pub(crate) fn new(&mut self, root: CellHandle) -> Self {
        let mut cells = HashMap::new();
        cells.insert(root, 0);

        Self { root, cells }
    }

    pub(crate) fn get_distance(&self, cell: CellHandle) -> Option<usize> {
        self.cells.get(&cell).copied()
    }

    pub(crate) fn set_distance(&mut self, cell: CellHandle, distance: usize) {
        self.cells.insert(cell, distance);
    }

    pub(crate) fn cells(&self) -> Keys<'_, CellHandle, usize> {
        self.cells.keys()
    }
}
