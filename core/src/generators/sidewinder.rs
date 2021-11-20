use crate::util::{coin, sample};
use crate::{Cell, CellHandle, Grid};

use super::Generator;

/// Sidewinder maze generator
/// Groups adjacent cells as a run before path carving
/// Perfect - Yes
/// Uniform - No
/// Bias - North edge path is always unbroken
/// Runtime - Fast
#[derive(Debug, Default)]
pub struct Sidewinder;

impl Sidewinder {
    pub(crate) fn link(
        grid: &Grid,
        cell: &Option<Cell>,
        run: &mut Vec<CellHandle>,
    ) -> Option<(CellHandle, CellHandle)> {
        if let Some(cell) = cell {
            let cell_handle = cell.handle();
            run.push(cell_handle);

            let at_eastern_boundary = cell.east.is_none();
            let at_northern_boundary = cell.north.is_none();

            // close out a run either at the eastern border
            // or randomly within a row, except at the northern border
            let should_close_out = at_eastern_boundary || (!at_northern_boundary && coin());

            if should_close_out {
                let member_handle = *sample(run);
                let member = member_handle.get_cell(grid);
                if let Some(member) = member {
                    if let Some(north) = member.north {
                        return Some((member_handle, north));
                    }
                }
                run.clear();
            } else {
                return Some((cell_handle, cell.east.unwrap()));
            }
        }

        None
    }
}

impl Generator for Sidewinder {
    fn name(&self) -> &str {
        "Sidewinder"
    }

    fn run(&self, grid: &mut Grid) {
        if grid.has_orphans() {
            return;
        }

        let links = grid
            .rows_iter()
            .map(|row| {
                let mut run = Vec::new();
                row.iter()
                    .filter_map(|cell| Self::link(grid, cell, &mut run))
                    .collect::<Vec<(CellHandle, CellHandle)>>()
            })
            .flatten()
            .collect::<Vec<(CellHandle, CellHandle)>>();
        grid.link_cells_multi(links);
    }
}
