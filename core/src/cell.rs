use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct CellHandle {
    pub(crate) row: usize,
    pub(crate) col: usize,
}

#[derive(Debug)]
pub struct Cell {
    pub row: usize,
    pub col: usize,

    pub(crate) north: Option<CellHandle>,
    pub(crate) south: Option<CellHandle>,
    pub(crate) east: Option<CellHandle>,
    pub(crate) west: Option<CellHandle>,

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
        CellHandle {
            row: self.row,
            col: self.col,
        }
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
}
