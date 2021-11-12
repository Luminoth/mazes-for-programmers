use std::collections::hash_set::Iter;
use std::collections::HashSet;

use rand::Rng;

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
}

impl From<(usize, usize)> for CellHandle {
    fn from(handle: (usize, usize)) -> Self {
        Self {
            row: handle.0,
            col: handle.1,
        }
    }
}

#[derive(Debug)]
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
        let mut rng = rand::thread_rng();

        let neighbors = self.neighbors();

        let index = rng.gen_range(0..neighbors.len());
        neighbors[index]
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
