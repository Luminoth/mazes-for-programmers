use std::collections::hash_set::Iter;
use std::collections::HashSet;

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

    pub fn is_linked(&self, cell: CellHandle) -> bool {
        self.links.contains(&cell)
    }

    pub fn link(&mut self, cell: CellHandle) {
        self.links.insert(cell);
    }

    pub fn unlink(&mut self, cell: CellHandle) {
        self.links.remove(&cell);
    }

    pub fn links(&self) -> Iter<'_, CellHandle> {
        self.links.iter()
    }
}
