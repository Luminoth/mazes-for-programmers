use crate::util::sample;
use crate::Grid;

use super::Generator;

/// BinaryTree maze generator
/// Selects random N/E neighbors to link
/// Perfect - Yes
/// Uniform - No
/// Bias - North / East edge paths are always unbroken
/// Runtime - Fast
#[derive(Debug, Default)]
pub struct BinaryTree;

impl Generator for BinaryTree {
    fn name(&self) -> &str {
        "Binary Tree"
    }

    fn generate(&self, rows: usize, cols: usize) -> Grid {
        let mut grid = Grid::new(rows, cols);

        let mut links = Vec::default();
        for cell in grid.iter() {
            let mut neighbors = Vec::new();

            if let Some(north) = cell.north {
                neighbors.push(north);
            }

            if let Some(east) = cell.east {
                neighbors.push(east);
            }

            if neighbors.is_empty() {
                continue;
            }

            // pick a random N/E neighbor to link
            let neighbor = *sample(&neighbors);
            links.push((cell.handle(), neighbor));
        }

        for link in links {
            grid.link_cells(link.0, link.1);
        }

        grid
    }
}
