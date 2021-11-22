use crate::util::sample;
use crate::Grid;

use super::Generator;

/// Hunt-and-Kill maze generator
/// Selects random unvisited cells to visit until all of them are visited,
/// hunting for unvisited cells when a loop is detected
/// Perfect - Yes
/// Uniform - No
/// Bias - Yes
/// Runtime - Fast
/// Slower than Recursive Backtracker but more memory efficient
#[derive(Debug, Default)]
pub struct HuntAndKill;

impl Generator for HuntAndKill {
    fn name(&self) -> &str {
        "Hunt-and-Kill"
    }

    fn run(&self, grid: &mut Grid) {
        let mut cell_handle = Some(grid.get_random().handle());

        while cell_handle.is_some() {
            let unvisited_neighbors = {
                let cell = cell_handle.unwrap().get_cell(grid).unwrap();
                let mut neighbors = cell.neighbors();
                neighbors.retain(|neighbor_handle| {
                    let neighbor = neighbor_handle.get_cell(grid).unwrap();
                    !neighbor.has_links()
                });
                neighbors
            };

            if !unvisited_neighbors.is_empty() {
                let neighbor_handle = sample(&unvisited_neighbors);
                grid.link_cells(cell_handle.unwrap(), *neighbor_handle);
                cell_handle = Some(*neighbor_handle);
            } else {
                cell_handle = None;

                // hunt for an unvisited cell that borders a visited cell
                let mut neighbor_handle = None;
                for cell in grid.iter() {
                    let mut visited_neighbors = cell.neighbors();
                    visited_neighbors.retain(|neighbor_handle| {
                        let neighbor = neighbor_handle.get_cell(grid).unwrap();
                        neighbor.has_links()
                    });

                    if !cell.has_links() && !visited_neighbors.is_empty() {
                        cell_handle = Some(cell.handle());
                        neighbor_handle = Some(*sample(&visited_neighbors));
                    }
                }

                // if we found one, link it to a neighbor and continue the random walk
                if let Some(neighbor_handle) = neighbor_handle {
                    grid.link_cells(cell_handle.unwrap(), neighbor_handle);
                }
            }
        }
    }
}
