use std::iter::Iterator;
use std::slice::Iter;

use crate::cell::*;

use rand::Rng;

#[derive(Debug)]
pub struct Grid {
    rows: usize,
    cols: usize,

    grid: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut grid = Self {
            rows,
            cols,
            grid: Vec::new(),
        };

        grid.prepare_grid();
        grid.configure_cells();

        grid
    }

    fn prepare_grid(&mut self) {
        self.grid = Vec::with_capacity(self.rows);
        for row in 0..self.rows {
            let mut cells = Vec::with_capacity(self.cols);
            for col in 0..self.cols {
                cells.push(Cell::new(row, col));
            }
            self.grid.push(cells);
        }
    }

    fn configure_cells(&mut self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let north = self.get(row - 1, col).map(|cell| cell.state);
                let south = self.get(row + 1, col).map(|cell| cell.state);
                let west = self.get(row, col - 1).map(|cell| cell.state);
                let east = self.get(row, col + 1).map(|cell| cell.state);

                let cell = self.get_mut(row, col);
                if let Some(cell) = cell {
                    cell.north = north;
                    cell.south = south;
                    cell.west = west;
                    cell.east = east;
                }
            }
        }
    }

    pub fn size(&self) -> usize {
        self.rows * self.cols
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&Cell> {
        self.grid.get(row)?.get(col)
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut Cell> {
        self.grid.get_mut(row)?.get_mut(col)
    }

    pub fn get_random(&self) -> Option<&Cell> {
        let mut rng = rand::thread_rng();

        let row = rng.gen_range(0..self.rows);
        let col = rng.gen_range(0..self.cols);
        self.get(row, col)
    }

    pub fn row_iter(&self) -> Iter<'_, Vec<Cell>> {
        self.grid.iter()
    }

    pub fn iter(&self) -> CellIterator<'_> {
        CellIterator::new(self)
    }
}

impl<'a> IntoIterator for &'a Grid {
    type Item = &'a Cell;
    type IntoIter = CellIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct CellIterator<'a> {
    grid: &'a Grid,

    row: usize,
    col: usize,
}

impl<'a> CellIterator<'a> {
    fn new(grid: &'a Grid) -> Self {
        Self {
            grid,
            row: 0,
            col: 0,
        }
    }
}

impl<'a> Iterator for CellIterator<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.grid.get(self.row, self.col);

        let mut next_row = self.row;
        let mut next_col = self.col + 1;
        if next_col >= self.grid.cols {
            next_row += 1;
            next_col = 0;
        }

        self.row = next_row;
        self.col = next_col;

        ret
    }
}
