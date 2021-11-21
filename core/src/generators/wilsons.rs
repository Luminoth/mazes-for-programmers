use crate::util::sample;
use crate::{CellHandle, Grid};

use super::Generator;

/// Aldous-Broder maze generator
/// Loop-erasing random walk of cells until all are visited
/// Perfect - Yes
/// Uniform - Yes
/// Bias - None
/// Runtime - Slow to start, fast to finish
#[derive(Debug, Default)]
pub struct Wilsons;

impl Generator for Wilsons {
    fn name(&self) -> &str {
        "Wilson's Algorithm"
    }

    fn run(&self, grid: &mut Grid) {
        let mut unvisited = grid.handles_iter().collect::<Vec<CellHandle>>();

        // visit the first cell
        let first = *sample(&unvisited);
        let unvisited_index = unvisited.iter().position(|&c| c == first).unwrap();
        unvisited.swap_remove(unvisited_index);

        // visit everything else starting with a random unvisited cell
        while !unvisited.is_empty() {
            let mut cell_handle = *sample(&unvisited);
            let mut path = vec![cell_handle];

            // random walk unvisited cells
            // building a path between them
            // erasing loops as we go
            while unvisited.contains(&cell_handle) {
                let cell = cell_handle.get_cell(grid).unwrap();
                cell_handle = cell.get_random_neighbor();

                let position = path.iter().position(|&c| c == cell_handle);
                if let Some(position) = position {
                    // we've hit a loop, so erase it
                    path.truncate(position + 1);
                } else {
                    path.push(cell_handle);
                }
            }

            // carve the path
            for index in 0..=path.len() - 2 {
                grid.link_cells(path[index], path[index + 1]);

                let unvisited_index = unvisited.iter().position(|&c| c == path[index]).unwrap();
                unvisited.swap_remove(unvisited_index);
            }
        }
    }
}
