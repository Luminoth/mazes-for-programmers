use rayon::prelude::*;

use crate::{CellHandle, Grid};

use super::{Generator, Sidewinder};

/// Sidewinder maze generator (parallelized)
/// Groups adjacent cells as a run before path carving
/// Perfect - Yes
/// Uniform - No
/// Bias - North edge path is always unbroken
/// Runtime - Fast
#[derive(Debug, Default)]
pub struct SidewinderParallel;

impl Generator for SidewinderParallel {
    fn name(&self) -> &str {
        "Sidewinder (Parallel)"
    }

    fn generate(&self, rows: usize, cols: usize) -> Grid {
        let mut grid = Grid::new(rows, cols);

        let links = grid
            .rows_iter()
            .par_bridge()
            .map(|row| {
                let mut run = Vec::new();
                row.iter()
                    .filter_map(|cell| Sidewinder::link(&grid, cell, &mut run))
                    .collect::<Vec<(CellHandle, CellHandle)>>()
            })
            .flatten()
            .collect::<Vec<(CellHandle, CellHandle)>>();

        for link in links {
            grid.link_cells(link.0, link.1);
        }

        grid
    }
}
