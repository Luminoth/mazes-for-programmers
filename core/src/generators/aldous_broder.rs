use crate::Grid;

use super::Generator;

/// Aldous-Broder maze generator
/// Selects random cells, including cells already visited, to visit until all of them are visited
/// Perfect - Yes
/// Uniform - Yes
/// Bias - None
/// Runtime - Fast to start, slow to finish
#[derive(Debug, Default)]
pub struct AldousBroder;

impl Generator for AldousBroder {
    fn name(&self) -> &str {
        "Aldous-Broder"
    }

    fn generate(&self, rows: usize, cols: usize) -> Grid {
        let mut grid = Grid::new(rows, cols);

        let mut cell_handle = grid.get_random().handle();
        let mut unvisited = grid.size() - 1;

        while unvisited > 0 {
            let neighbor_handle = cell_handle.get_cell(&grid).unwrap().get_random_neighbor();
            let neighbor = neighbor_handle.get_cell(&grid).unwrap();
            if !neighbor.has_links() {
                grid.link_cells(cell_handle, neighbor_handle);
                unvisited -= 1;
            }

            cell_handle = neighbor_handle;
        }

        grid
    }
}
