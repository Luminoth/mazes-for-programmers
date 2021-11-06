use std::iter::Iterator;

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

    pub fn get_random_mut(&mut self) -> Option<&mut Cell> {
        let mut rng = rand::thread_rng();

        let row = rng.gen_range(0..self.rows);
        let col = rng.gen_range(0..self.cols);
        self.get_mut(row, col)
    }

    pub fn row_iter(&self) -> std::slice::Iter<'_, Vec<Cell>> {
        self.grid.iter()
    }

    pub fn row_iter_mut(&mut self) -> std::slice::IterMut<'_, Vec<Cell>> {
        self.grid.iter_mut()
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter::new(self)
    }

    pub fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut::new(self)
    }
}

pub struct Iter<'a> {
    grid: &'a Grid,

    row: usize,
    col: usize,
}

impl<'a> Iter<'a> {
    fn new(grid: &'a Grid) -> Self {
        Self {
            grid,
            row: 0,
            col: 0,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
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

impl<'a> IntoIterator for &'a Grid {
    type Item = &'a Cell;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct IterMut<'a> {
    grid: &'a mut Grid,

    row: usize,
    col: usize,
}

impl<'a> IterMut<'a> {
    fn new(grid: &'a mut Grid) -> Self {
        Self {
            grid,
            row: 0,
            col: 0,
        }
    }
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut Cell;

    fn next(&mut self) -> Option<Self::Item> {
        //let ret = self.grid.get_mut(self.row, self.col);
        // TODO: can we rework anything to remove this unsafe?
        let ret = unsafe {
            if self.row >= self.grid.rows || self.col >= self.grid.cols {
                return None;
            }

            let cols = self.grid.grid.get_mut(self.row).unwrap();
            let ptr = cols.as_mut_ptr();

            Some(&mut *ptr.add(self.col))
        };

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

impl<'a> IntoIterator for &'a mut Grid {
    type Item = &'a mut Cell;
    type IntoIter = IterMut<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
