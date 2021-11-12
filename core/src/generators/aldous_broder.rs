use crate::Grid;

use super::Generator;

/// Aldous-Broder maze generator
/// Selects random cells to visit until all of them are visited
/// Perfect - Yes
/// Uniform - Yes
/// Bias - None
/// Runtime - Long
#[derive(Debug, Default)]
pub struct AldousBroder;

impl Generator for AldousBroder {
    fn generate(&self, rows: usize, cols: usize) -> Grid {
        let mut grid = Grid::new(rows, cols);

        let mut cell_handle = grid.get_random().handle();
        let mut unvisited = grid.size() - 1;

        while unvisited > 0 {
            let neighbor_handle = {
                let cell = grid.get(cell_handle.row, cell_handle.col).unwrap();
                cell.get_random_neighbor()
            };

            let has_links = {
                let neighbor = grid.get(neighbor_handle.row, neighbor_handle.col).unwrap();
                neighbor.has_links()
            };

            if !has_links {
                grid.link_cells(cell_handle, neighbor_handle);
                unvisited -= 1;
            }

            cell_handle = neighbor_handle;
        }

        grid
    }
}
