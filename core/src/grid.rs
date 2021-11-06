use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct CellState {
    row: usize,
    col: usize,
}

#[derive(Debug)]
pub struct Cell {
    state: CellState,

    north: Option<CellState>,
    south: Option<CellState>,
    east: Option<CellState>,
    west: Option<CellState>,

    links: HashSet<CellState>,
}

impl Cell {
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            state: CellState { row, col },
            north: None,
            south: None,
            east: None,
            west: None,
            links: HashSet::default(),
        }
    }

    pub fn is_linked(&self, cell: &Cell) -> bool {
        self.links.contains(&cell.state)
    }

    pub fn link(&mut self, cell: &Cell) {
        self.links.insert(cell.state);
    }

    pub fn link_bidirectional(&mut self, cell: &mut Cell) {
        self.links.insert(cell.state);
        cell.links.insert(self.state);
    }

    pub fn unlink(&mut self, cell: &Cell) {
        self.links.remove(&cell.state);
    }

    pub fn unlink_bidirectional(&mut self, cell: &mut Cell) {
        self.links.remove(&cell.state);
        cell.links.remove(&self.state);
    }
}

#[derive(Debug, Default)]
pub struct Grid {}
