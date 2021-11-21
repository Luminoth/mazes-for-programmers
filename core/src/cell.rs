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

    pub fn is_orphaned(&self) -> bool {
        !self.has_neighbors()
    }

    // TODO: this could be better if we didn't allocate a vec each time
    pub fn neighbors(&self) -> Vec<CellHandle> {
        assert!(!self.is_orphaned());

        let mut neighbors = Vec::with_capacity(4);

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

    // sets this cell as orphaned
    // orphaned() must be called to
    // tell this cell's neighbors it was orphaned
    pub fn orphan(&mut self) {
        self.north = None;
        self.south = None;
        self.east = None;
        self.west = None;
    }

    // removes this cell from its neighbors,
    // effectively orphaning it
    // orphan() must be called to tell this cell it was orphaned
    pub fn orphaned(&self, grid: &mut Grid) {
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
        assert!(!self.is_orphaned());

        self.links.contains(&cell)
    }

    // NOTE: this is not bidirectional
    // use Grid::link_cells() for that
    pub fn link(&mut self, cell: CellHandle) {
        assert!(!self.is_orphaned());

        self.links.insert(cell);
    }

    // NOTE: this is not bidirectional
    // use Grid::unlink_cells() for that
    pub fn unlink(&mut self, cell: CellHandle) {
        assert!(!self.is_orphaned());

        self.links.remove(&cell);
    }

    pub fn has_links(&self) -> bool {
        assert!(!self.is_orphaned());

        !self.links.is_empty()
    }

    pub fn links(&self) -> Iter<'_, CellHandle> {
        assert!(!self.is_orphaned());

        self.links.iter()
    }
}
