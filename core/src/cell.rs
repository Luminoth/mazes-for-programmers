use std::collections::hash_set::Iter;
use std::collections::HashSet;

use crate::util::sample;
use crate::Grid;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct CellHandle {
    pub row: usize,
    pub col: usize,
}

impl CellHandle {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    // not sure why but into() doesn't work even tho From is impl'd
    pub fn unpack(&self) -> (usize, usize) {
        (self.row, self.col)
    }

    pub fn get_cell<'a>(&self, grid: &'a Grid) -> Option<&'a Cell> {
        grid.get(self.row, self.col)
    }
}

impl From<(usize, usize)> for CellHandle {
    fn from(handle: (usize, usize)) -> Self {
        Self {
            row: handle.0,
            col: handle.1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub row: usize,
    pub col: usize,

    // track whether we have a neighbor or not
    // (this helps identify edge cells)
    pub north: Option<CellHandle>,
    pub south: Option<CellHandle>,
    pub east: Option<CellHandle>,
    pub west: Option<CellHandle>,

    // linked cells have no wall between them
    links: HashSet<CellHandle>,
}

impl Cell {
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            row,
            col,
            north: None,
            south: None,
            east: None,
            west: None,
            links: HashSet::default(),
        }
    }

    pub fn handle(&self) -> CellHandle {
        CellHandle::new(self.row, self.col)
    }

    pub fn has_neighbors(&self) -> bool {
        self.north.is_some() || self.south.is_some() || self.east.is_some() || self.west.is_some()
    }

    // TODO: this could be better if we didn't build a vec each time
    pub fn neighbors(&self) -> Vec<CellHandle> {
        let mut neighbors = Vec::new();

        if let Some(neighbor) = self.north {
            neighbors.push(neighbor);
        }

        if let Some(neighbor) = self.south {
            neighbors.push(neighbor);
        }

        if let Some(neighbor) = self.east {
            neighbors.push(neighbor);
        }

        if let Some(neighbor) = self.west {
            neighbors.push(neighbor);
        }

        neighbors
    }

    pub fn get_random_neighbor(&self) -> CellHandle {
        let neighbors = self.neighbors();
        *sample(&neighbors)
    }

    pub fn orphan(&self, grid: &mut Grid) {
        if let Some(north) = self.north {
            if let Some(cell) = grid.get_mut(north.row, north.col) {
                cell.south = None;
            }
        }

        if let Some(south) = self.south {
            if let Some(cell) = grid.get_mut(south.row, south.col) {
                cell.north = None;
            }
        }

        if let Some(east) = self.east {
            if let Some(cell) = grid.get_mut(east.row, east.col) {
                cell.west = None;
            }
        }

        if let Some(west) = self.west {
            if let Some(cell) = grid.get_mut(west.row, west.col) {
                cell.east = None;
            }
        }
    }

    pub fn is_linked(&self, cell: CellHandle) -> bool {
        self.links.contains(&cell)
    }

    // NOTE: this is not bidirectional
    // use Grid::link_cells() for that
    pub fn link(&mut self, cell: CellHandle) {
        self.links.insert(cell);
    }

    // NOTE: this is not bidirectional
    // use Grid::unlink_cells() for that
    pub fn unlink(&mut self, cell: CellHandle) {
        self.links.remove(&cell);
    }

    pub fn has_links(&self) -> bool {
        !self.links.is_empty()
    }

    pub fn links(&self) -> Iter<'_, CellHandle> {
        self.links.iter()
    }
}
