use std::cell::RefCell;
use std::io;
use std::path::Path;

use crate::util::Color;
use crate::{CellHandle, Distances, Grid, Renderable};

use super::Solver;

/// Simple Djikstra's algorithm solver
#[derive(Debug)]
pub struct Djikstra {
    grid: Grid,
    root: CellHandle,

    // used for cell background coloring
    distances_from_center: RefCell<Option<Distances>>,
    max_distance_from_center: RefCell<usize>,

    // solved path through the maze
    path: RefCell<Option<Distances>>,
}

impl Djikstra {
    /// Creates a new Djikstra's algorithm solver
    pub fn new(grid: Grid, root_row: usize, root_column: usize) -> Self {
        Self {
            grid,
            root: CellHandle::new(root_row, root_column),
            distances_from_center: RefCell::new(None),
            max_distance_from_center: RefCell::new(0),
            path: RefCell::new(None),
        }
    }

    fn cell_contents_from_distances(&self, distances: &Distances, cell: CellHandle) -> String {
        let (digits, empty) = self.grid.empty_cell_contents();

        let distance = distances.get_distance(&cell);
        if let Some(distance) = distance {
            format!(
                "{:>width$}",
                radix_fmt::radix_36(distance).to_string(),
                width = digits
            )
        } else {
            empty
        }
    }

    fn path_to(&self, goal: CellHandle, distances: &Distances) -> Distances {
        let mut current = goal;
        let mut current_distance = distances.get_distance(&current).unwrap_or_default();

        let mut breadcrumbs = Distances::new(self.root);
        breadcrumbs.set_distance(current, current_distance);

        while current != self.root {
            let cell = self.grid.get(current.row, current.col).unwrap();

            for neighbor in cell.links() {
                let neighbor_distance = distances.get_distance(neighbor).unwrap_or_default();
                if neighbor_distance < current_distance {
                    breadcrumbs.set_distance(*neighbor, neighbor_distance);

                    current = *neighbor;
                    current_distance = distances.get_distance(&current).unwrap_or_default();
                }
            }
        }

        breadcrumbs
    }
}

impl Solver for Djikstra {
    fn name(&self) -> &str {
        "Djikstra"
    }

    fn grid(&self) -> &Grid {
        &self.grid
    }

    fn cell_contents(&self, row: usize, col: usize) -> String {
        let cell = CellHandle::new(row, col);

        if let Some(path) = self.path.borrow().as_ref() {
            self.cell_contents_from_distances(path, cell)
        } else {
            let (_, empty) = self.grid.empty_cell_contents();
            empty
        }
    }

    fn cell_background(&self, row: usize, col: usize) -> Color {
        let cell = CellHandle::new(row, col);

        let distance = self
            .distances_from_center
            .borrow()
            .as_ref()
            .unwrap()
            .get_distance(&cell)
            .unwrap_or_default();
        let max_distance = *self.max_distance_from_center.borrow();

        let intensity = (max_distance - distance) as f32 / max_distance as f32;
        let dark = (255.0 * intensity).round() as u8;
        let bright = 128 + (127.0 * intensity).round() as u8;

        Color::new(dark, bright, dark, 255)
    }

    fn solve(&self, goal_row: usize, goal_col: usize) {
        // compute the shortest path
        let distances = crate::distances(&self.grid, self.root);
        *self.path.borrow_mut() =
            Some(self.path_to(CellHandle::new(goal_row, goal_col), &distances));

        // compute distances from the center
        // for cell background coloring
        let distances = crate::distances(
            &self.grid,
            CellHandle::new(self.grid.rows() / 2, self.grid.columns() / 2),
        );

        let (_, max_distance) = distances.max_distance();
        *self.max_distance_from_center.borrow_mut() = max_distance;

        *self.distances_from_center.borrow_mut() = Some(distances);
    }
}

impl Renderable for Djikstra {
    fn render_ascii(&self) -> String {
        self.grid.render_ascii_internal(Some(self))
    }

    fn save_png(&self, path: &Path, cell_size: usize) -> io::Result<()> {
        self.grid.save_png_internal(path, cell_size, Some(self))
    }
}
