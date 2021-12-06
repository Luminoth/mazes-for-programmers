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
pub enum Cell {
    Orthogonal(OrthogonalCell),
    Polar(PolarCell),
}

impl Cell {
    pub fn new_ortho(row: usize, col: usize) -> Self {
        Self::Orthogonal(OrthogonalCell::new(row, col))
    }

    pub fn new_polar(row: usize, col: usize) -> Self {
        Self::Polar(PolarCell::new(row, col))
    }

    pub fn handle(&self) -> CellHandle {
        match self {
            Self::Orthogonal(cell) => CellHandle::new(cell.row, cell.col),
            Self::Polar(cell) => CellHandle::new(cell.row, cell.col),
        }
    }

    pub fn row(&self) -> usize {
        match self {
            Self::Orthogonal(cell) => cell.row,
            Self::Polar(cell) => cell.row,
        }
    }

    pub fn col(&self) -> usize {
        match self {
            Self::Orthogonal(cell) => cell.col,
            Self::Polar(cell) => cell.col,
        }
    }

    pub fn has_neighbors(&self) -> bool {
        match self {
            Self::Orthogonal(cell) => cell.has_neighbors(),
            Self::Polar(cell) => cell.has_neighbors(),
        }
    }

    pub fn is_orphaned(&self) -> bool {
        !self.has_neighbors()
    }

    pub fn neighbors(&self) -> Vec<CellHandle> {
        assert!(!self.is_orphaned());

        match self {
            Self::Orthogonal(cell) => cell.neighbors(),
            Self::Polar(cell) => cell.neighbors(),
        }
    }

    pub fn get_random_neighbor(&self) -> CellHandle {
        let neighbors = self.neighbors();
        *sample(&neighbors)
    }

    // sets this cell as orphaned
    // orphaned() must be called to
    // tell this cell's neighbors it was orphaned
    pub fn orphan(&mut self) {
        match self {
            Self::Orthogonal(cell) => cell.orphan(),
            Self::Polar(cell) => cell.orphan(),
        }
    }

    // removes this cell from its neighbors,
    // effectively orphaning it
    // orphan() must be called to tell this cell it was orphaned
    pub fn orphaned(&self, grid: &mut Grid) {
        match self {
            Self::Orthogonal(cell) => cell.orphaned(grid),
            Self::Polar(cell) => cell.orphaned(grid),
        }
    }

    pub fn is_linked(&self, other: CellHandle) -> bool {
        assert!(!self.is_orphaned());

        match self {
            Self::Orthogonal(cell) => cell.links.contains(&other),
            Self::Polar(cell) => cell.links.contains(&other),
        }
    }

    // NOTE: this is not bidirectional
    // use Grid::link_cells() for that
    pub fn link(&mut self, other: CellHandle) {
        assert!(!self.is_orphaned());

        match self {
            Self::Orthogonal(cell) => cell.links.insert(other),
            Self::Polar(cell) => cell.links.insert(other),
        };
    }

    // NOTE: this is not bidirectional
    // use Grid::unlink_cells() for that
    pub fn unlink(&mut self, other: CellHandle) {
        assert!(!self.is_orphaned());

        match self {
            Self::Orthogonal(cell) => cell.links.remove(&other),
            Self::Polar(cell) => cell.links.remove(&other),
        };
    }

    pub fn has_links(&self) -> bool {
        assert!(!self.is_orphaned());

        match self {
            Self::Orthogonal(cell) => !cell.links.is_empty(),
            Self::Polar(cell) => !cell.links.is_empty(),
        }
    }

    pub fn links(&self) -> Iter<'_, CellHandle> {
        assert!(!self.is_orphaned());

        match self {
            Self::Orthogonal(cell) => cell.links.iter(),
            Self::Polar(cell) => cell.links.iter(),
        }
    }
}

#[derive(Debug, Clone)]
struct OrthogonalCell {
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

impl OrthogonalCell {
    fn new(row: usize, col: usize) -> Self {
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

    fn has_neighbors(&self) -> bool {
        self.north.is_some() || self.south.is_some() || self.east.is_some() || self.west.is_some()
    }

    // TODO: this could be better if we didn't allocate a vec each time
    fn neighbors(&self) -> Vec<CellHandle> {
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

    fn orphan(&mut self) {
        self.north = None;
        self.south = None;
        self.east = None;
        self.west = None;
    }

    fn orphaned(&self, grid: &mut Grid) {
        if let Some(north) = self.north {
            if let Some(cell) = grid.get_mut(north.row, north.col) {
                match cell {
                    Cell::Orthogonal(cell) => cell.south = None,
                    _ => panic!("Invalid cell type"),
                }
            }
        }

        if let Some(south) = self.south {
            if let Some(cell) = grid.get_mut(south.row, south.col) {
                match cell {
                    Cell::Orthogonal(cell) => cell.north = None,
                    _ => panic!("Invalid cell type"),
                }
            }
        }

        if let Some(east) = self.east {
            if let Some(cell) = grid.get_mut(east.row, east.col) {
                match cell {
                    Cell::Orthogonal(cell) => cell.west = None,
                    _ => panic!("Invalid cell type"),
                }
            }
        }

        if let Some(west) = self.west {
            if let Some(cell) = grid.get_mut(west.row, west.col) {
                match cell {
                    Cell::Orthogonal(cell) => cell.east = None,
                    _ => panic!("Invalid cell type"),
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct PolarCell {
    pub row: usize,
    pub col: usize,

    // track whether we have a neighbor or not
    // (this helps identify edge cells)
    pub cw: Option<CellHandle>,
    pub ccw: Option<CellHandle>,
    pub inward: Option<CellHandle>,
    pub outward: Vec<CellHandle>,

    // linked cells have no wall between them
    links: HashSet<CellHandle>,
}

impl PolarCell {
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            row,
            col,
            cw: None,
            ccw: None,
            inward: None,
            outward: Vec::new(),
            links: HashSet::default(),
        }
    }

    fn has_neighbors(&self) -> bool {
        self.cw.is_some() || self.ccw.is_some() || self.inward.is_some() || !self.outward.is_empty()
    }

    // TODO: this could be better if we didn't allocate a vec each time
    fn neighbors(&self) -> Vec<CellHandle> {
        let mut neighbors = Vec::with_capacity(4);

        if let Some(neighbor) = self.cw {
            neighbors.push(neighbor);
        }

        if let Some(neighbor) = self.ccw {
            neighbors.push(neighbor);
        }

        if let Some(neighbor) = self.inward {
            neighbors.push(neighbor);
        }

        neighbors.extend(&self.outward);

        neighbors
    }

    fn orphan(&mut self) {
        self.cw = None;
        self.ccw = None;
        self.inward = None;
        self.outward.clear();
    }

    fn orphaned(&self, grid: &mut Grid) {
        if let Some(cw) = self.cw {
            if let Some(cell) = grid.get_mut(cw.row, cw.col) {
                match cell {
                    Cell::Polar(cell) => cell.ccw = None,
                    _ => panic!("Invalid cell type"),
                }
            }
        }

        if let Some(ccw) = self.ccw {
            if let Some(cell) = grid.get_mut(ccw.row, ccw.col) {
                match cell {
                    Cell::Polar(cell) => cell.cw = None,
                    _ => panic!("Invalid cell type"),
                }
            }
        }

        if let Some(inward) = self.inward {
            if let Some(cell) = grid.get_mut(inward.row, inward.col) {
                match cell {
                    Cell::Polar(cell) => cell.outward.clear(),
                    _ => panic!("Invalid cell type"),
                }
            }
        }

        for outward in self.outward {
            if let Some(cell) = grid.get_mut(outward.row, outward.col) {
                match cell {
                    Cell::Polar(cell) => cell.inward = None,
                    _ => panic!("Invalid cell type"),
                }
            }
        }
    }
}
