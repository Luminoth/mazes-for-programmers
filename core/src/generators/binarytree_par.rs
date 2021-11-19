use rayon::prelude::*;

use crate::{CellHandle, Grid};

use super::{BinaryTree, Generator};

/// BinaryTree maze generator (parallelized)
/// Selects random N/E neighbors to link
/// Perfect - Yes
/// Uniform - No
/// Bias - North / East edge paths are always unbroken
/// Runtime - Fast
#[derive(Debug, Default)]
pub struct BinaryTreeParallel;

impl Generator for BinaryTreeParallel {
    fn name(&self) -> &str {
        "Binary Tree (Parallel)"
    }

    fn run(&self, grid: &mut Grid) {
        let links = grid
            .iter()
            .par_bridge()
            .filter_map(|cell| {
                if let Some(neighbor) = BinaryTree::choose_neighbor(cell) {
                    return Some((cell.handle(), neighbor));
                }
                None
            })
            .collect::<Vec<(CellHandle, CellHandle)>>();
        grid.link_cells_multi(links);
    }
}
