use std::fs;
use std::io;
use std::iter::Iterator;
use std::path::Path;

use rand::Rng;
//use tracing::debug;

use crate::solvers::Solver;
use crate::util::{horizontal_line, quad, vertical_line, Color};
use crate::{Cell, CellHandle, Renderable};

/// Grid-based maze data structure
#[derive(Debug, Clone)]
pub struct Grid {
    rows: usize,
    cols: usize,

    grid: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(rows: usize, cols: usize) -> Self {
        assert!(rows > 0 && cols > 0);

        let mut grid = Self {
            rows,
            cols,
            grid: Vec::new(),
        };

        grid.init_grid();
        grid.init_cells();

        grid
    }

    fn init_grid(&mut self) {
        self.grid = Vec::with_capacity(self.rows);
        for row in 0..self.rows {
            let mut cells = Vec::with_capacity(self.cols);
            for col in 0..self.cols {
                cells.push(Cell::new(row, col));
            }
            self.grid.push(cells);
        }
    }

    fn init_cells(&mut self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let north = if row > 0 {
                    self.get(row - 1, col).map(|cell| cell.handle())
                } else {
                    None
                };

                let south = self.get(row + 1, col).map(|cell| cell.handle());

                let west = if col > 0 {
                    self.get(row, col - 1).map(|cell| cell.handle())
                } else {
                    None
                };

                let east = self.get(row, col + 1).map(|cell| cell.handle());

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

    /// The number of rows in the grid
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// The number of cols in the grid
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// The number of cells in the grid
    pub fn size(&self) -> usize {
        self.rows * self.cols
    }

    /// Set of dead end cells (cells with only one link) in the grid
    pub fn get_dead_ends(&self) -> Vec<&Cell> {
        self.iter()
            .filter(|&cell| cell.links().len() == 1)
            .collect()
    }

    /// Gets a reference to the given cell if it exists
    pub fn get(&self, row: usize, col: usize) -> Option<&Cell> {
        self.grid.get(row)?.get(col)
    }

    /// Gets a mutable reference to the given cell if it exists
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut Cell> {
        self.grid.get_mut(row)?.get_mut(col)
    }

    /// Gets a reference to a random cell
    pub fn get_random(&self) -> &Cell {
        let mut rng = rand::thread_rng();

        let row = rng.gen_range(0..self.rows);
        let col = rng.gen_range(0..self.cols);
        self.get(row, col).unwrap()
    }

    /// Gets a mutable reference to a random cell
    pub fn get_random_mut(&mut self) -> &mut Cell {
        let mut rng = rand::thread_rng();

        let row = rng.gen_range(0..self.rows);
        let col = rng.gen_range(0..self.cols);
        self.get_mut(row, col).unwrap()
    }

    /// Returns an iterator over the grid rows
    pub fn rows_iter(&self) -> std::slice::Iter<'_, Vec<Cell>> {
        self.grid.iter()
    }

    /// Returns a mutable iterator over the grid rows
    pub fn rows_iter_mut(&mut self) -> std::slice::IterMut<'_, Vec<Cell>> {
        self.grid.iter_mut()
    }

    /// Returns an iterator over the grid cells
    pub fn iter(&self) -> Iter<'_> {
        Iter::new(self)
    }

    /// Returns a mutable iterator over the grid cells
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut::new(self)
    }

    /// Links two cells bidirectionally
    /// This creates a path between the cells
    pub(crate) fn link_cells(&mut self, a: CellHandle, b: CellHandle) {
        if let Some(a) = self.get_mut(a.row, a.col) {
            a.link(b);
        }

        if let Some(b) = self.get_mut(b.row, b.col) {
            b.link(a);
        }
    }

    pub(crate) fn link_cells_multi(&mut self, links: impl AsRef<[(CellHandle, CellHandle)]>) {
        for link in links.as_ref() {
            self.link_cells(link.0, link.1);
        }
    }

    /// Unlinks two cells bidirectionally
    /// This removes the path between the cells
    #[allow(unused)]
    pub(crate) fn unlink_cells(&mut self, a: CellHandle, b: CellHandle) {
        if let Some(a) = self.get_mut(a.row, a.col) {
            a.unlink(b);
        }

        if let Some(b) = self.get_mut(b.row, b.col) {
            b.unlink(a);
        }
    }

    /// Compute the longest path through the maze
    pub fn longest_path(&self) -> ((usize, usize), (usize, usize)) {
        let start = CellHandle::new(0, 0);

        let distances = crate::distances(self, start);
        let (new_start, _) = distances.max_distance();

        let distances = crate::distances(self, new_start);
        let (goal, _) = distances.max_distance();

        (new_start.unpack(), goal.unpack())
    }

    pub fn empty_cell_contents(&self) -> (usize, String) {
        let digits = (self.size() as f64).log(36.0).ceil() as usize;
        (digits, str::repeat(" ", digits))
    }

    pub(crate) fn render_ascii_internal(&self, solver: Option<&impl Solver>) -> String {
        let (digits, empty) = self.empty_cell_contents();
        let mut output = format!(
            "+{}\n",
            format!("-{}-+", str::repeat("-", digits)).repeat(self.cols)
        );

        for row in self.rows_iter() {
            let mut top = String::from("|");
            let mut bottom = String::from("+");

            for cell in row {
                let body = format!(
                    " {} ",
                    solver
                        .map(|solver| solver.cell_contents(cell.row, cell.col))
                        .unwrap_or_else(|| empty.clone())
                );

                top.push_str(&body);
                let east_boundary = if let Some(east) = cell.east {
                    if cell.is_linked(east) {
                        " "
                    } else {
                        "|"
                    }
                } else {
                    "|"
                };
                top.push_str(east_boundary);

                let south_boundary = if let Some(south) = cell.south {
                    if cell.is_linked(south) {
                        format!(" {} ", str::repeat(" ", digits))
                    } else {
                        format!("-{}-", str::repeat("-", digits))
                    }
                } else {
                    format!("-{}-", str::repeat("-", digits))
                };
                bottom.push_str(&south_boundary);
                bottom.push('+');
            }

            output.push_str(&top);
            output.push('\n');

            output.push_str(&bottom);
            output.push('\n');
        }

        output
    }

    fn generate_image(
        &self,
        cell_size: usize,
        solver: Option<&impl Solver>,
        color: bool,
    ) -> (usize, usize, Vec<u8>) {
        let wall = Color::new(0, 0, 0, 255);

        // width / height in pixels
        // (plus 2 for the edge walls)
        let width = (cell_size * self.cols) + 2;
        let height = (cell_size * self.rows) + 2;

        // size in bytes (4 bytes per-pixel)
        let size = width * height * 4;

        // init image to the background color for each cell
        let mut data = vec![0; size];
        for cell in self {
            let cell_handle = cell.handle();

            let background = if color {
                solver
                    .map(|solver| solver.cell_background(cell_handle.row, cell_handle.col))
                    .unwrap_or_else(|| Color::WHITE)
            } else {
                Color::WHITE
            };

            let x1 = cell.col * cell_size;
            let y1 = cell.row * cell_size;
            let x2 = (cell.col + 1) * cell_size;
            let y2 = (cell.row + 1) * cell_size;

            quad(&mut data, width, x1, y1, x2, y2, background);
        }

        // draw the cell walls
        for cell in self {
            let x1 = 1 + (cell.col * cell_size);
            let y1 = 1 + (cell.row * cell_size);
            let x2 = (cell.col + 1) * cell_size;
            let y2 = (cell.row + 1) * cell_size;

            if cell.north.is_none() {
                horizontal_line(&mut data, width, x1, x2, y1, wall);
            }

            if cell.west.is_none() {
                vertical_line(&mut data, width, x1, y1, y2, wall);
            }

            if let Some(east) = cell.east {
                if !cell.is_linked(east) {
                    vertical_line(&mut data, width, x2, y1, y2, wall);
                }
            } else {
                vertical_line(&mut data, width, x2, y1, y2, wall);
            }

            if let Some(south) = cell.south {
                if !cell.is_linked(south) {
                    horizontal_line(&mut data, width, x1, x2, y2, wall);
                }
            } else {
                horizontal_line(&mut data, width, x1, x2, y2, wall);
            }
        }

        (width, height, data)
    }

    fn save_png(
        &self,
        path: impl AsRef<Path>,
        cell_size: usize,
        solver: Option<&impl Solver>,
        color: bool,
    ) -> io::Result<()> {
        let file = fs::File::create(path)?;
        let w = io::BufWriter::new(file);

        let (width, height, data) = self.generate_image(cell_size, solver, color);

        let mut encoder = png::Encoder::new(w, width as u32, height as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        //debug!("data size: {}", data.len());
        writer.write_image_data(&data)?;

        Ok(())
    }

    pub(crate) fn save_png_internal(
        &self,
        path: impl AsRef<Path>,
        cell_size: usize,
        solver: Option<&impl Solver>,
    ) -> io::Result<()> {
        let path = path.as_ref();

        // save in greyscale
        self.save_png(path, cell_size, solver, false)?;

        // get the file name but with _color appended
        let mut color_filename = path.file_stem().unwrap().to_os_string();
        color_filename.push("-colored");

        // build the color file path
        let mut color_path = path.to_owned();
        color_path.set_file_name(&color_filename);
        if let Some(extension) = path.extension() {
            color_path.set_extension(extension);
        }

        // save in color
        self.save_png(color_path, cell_size, solver, true)?;

        Ok(())
    }
}

impl Renderable for Grid {
    fn render_ascii(&self) -> String {
        self.render_ascii_internal(None::<&crate::solvers::Djikstra>)
    }

    fn save_png(&self, path: &Path, cell_size: usize) -> io::Result<()> {
        self.save_png_internal(path, cell_size, None::<&crate::solvers::Djikstra>)
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
