use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct CellHandle {
    row: usize,
    col: usize,
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

    pub fn is_linked(&self, cell: &Cell) -> bool {
        self.links.contains(&cell.handle())
    }

    pub fn link(&mut self, cell: &Cell) {
        self.links.insert(cell.handle());
    }

    pub fn link_bidirectional(&mut self, cell: &mut Cell) {
        self.links.insert(cell.handle());
        cell.links.insert(self.handle());
    }

    pub fn unlink(&mut self, cell: &Cell) {
        self.links.remove(&cell.handle());
    }

    pub fn unlink_bidirectional(&mut self, cell: &mut Cell) {
        self.links.remove(&cell.handle());
        cell.links.remove(&self.handle());
    }
}
