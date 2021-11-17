use crate::util::sample;
use crate::{Cell, CellHandle, Grid};

use super::Generator;

/// BinaryTree maze generator
/// Selects random N/E neighbors to link
/// Perfect - Yes
/// Uniform - No
/// Bias - North / East edge paths are always unbroken
/// Runtime - Fast
#[derive(Debug, Default)]
pub struct BinaryTree;

impl BinaryTree {
    /// Pick a random N/E neighbor to link
    pub(crate) fn choose_neighbor(cell: &Cell) -> Option<CellHandle> {
        // TODO: do this without allocating

        let mut neighbors = Vec::with_capacity(2);

        if let Some(north) = cell.north {
            neighbors.push(north);
        }

        if let Some(east) = cell.east {
            neighbors.push(east);
        }

        if neighbors.is_empty() {
            return None;
        }

        let neighbor = *sample(&neighbors);
        Some(neighbor)
    }
}

impl Generator for BinaryTree {
    fn name(&self) -> &str {
        "Binary Tree"
    }

    fn generate(&self, rows: usize, cols: usize) -> Grid {
        let mut grid = Grid::new(rows, cols);

        let links = grid
            .iter()
            .filter_map(|cell| {
                if let Some(neighbor) = Self::choose_neighbor(cell) {
                    return Some((cell.handle(), neighbor));
                }
                None
            })
            .collect::<Vec<(CellHandle, CellHandle)>>();
        grid.link_cells_multi(links);

        grid
    }
}
