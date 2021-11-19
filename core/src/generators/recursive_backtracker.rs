use crate::Grid;

use super::Generator;
use crate::util::sample;

/// Recursive backtracker maze generator
/// Selects random unvisited cells to visit until all of them are visited,
/// backtracking when loops are detected
/// Perfect - Yes
/// Uniform - No
/// Bias - Yes
/// Runtime - Fast
/// Faster than Hunt-and-Kill but less memory efficient
#[derive(Debug, Default)]
pub struct RecursiveBacktracker;

impl Generator for RecursiveBacktracker {
    fn name(&self) -> &str {
        "Recursive Backtracker"
    }

    fn run(&self, grid: &mut Grid) {
        let start = grid.get_random().handle();

        let mut stack = vec![start];
        while !stack.is_empty() {
            let current = *stack.last().unwrap();
            let neighbors = {
                let mut neighbors = current.get_cell(grid).unwrap().neighbors();
                neighbors
                    .retain(|neighbor_handle| !neighbor_handle.get_cell(grid).unwrap().has_links());
                neighbors
            };

            // if there are no unvisited neighbors
            // we need to backtrack
            if neighbors.is_empty() {
                stack.pop();
            } else {
                let neighbor = *sample(&neighbors);
                grid.link_cells(current, neighbor);
                stack.push(neighbor);
            }
        }
    }
}
