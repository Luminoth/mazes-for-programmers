use crate::util::{coin, sample};
use crate::Grid;

use super::Generator;

/// Sidewinder maze generator
/// Groups adjacent cells as a run before path carving
/// Perfect - Yes
/// Uniform - No
/// Bias - North edge path is always unbroken
/// Runtime - Fast
#[derive(Debug, Default)]
pub struct Sidewinder;

impl Generator for Sidewinder {
    fn generate(&self, rows: usize, cols: usize) -> Grid {
        let mut grid = Grid::new(rows, cols);

        let mut links = Vec::default();
        for row in grid.rows_iter() {
            let mut run = Vec::new();

            for cell in row {
                let cell_handle = cell.handle();

                run.push(cell_handle);

                let at_eastern_boundary = cell.east.is_none();
                let at_northern_boundary = cell.north.is_none();

                // close out a run either at the eastern border
                // or randomly within a row, except at the northern border
                let should_close_out = at_eastern_boundary || (!at_northern_boundary && coin());

                if should_close_out {
                    let member_handle = *sample(&run);
                    let member = grid.get(member_handle.row, member_handle.col).unwrap();
                    if let Some(north) = member.north {
                        links.push((member_handle, north));
                    }
                    run.clear();
                } else {
                    links.push((cell_handle, cell.east.unwrap()));
                }
            }
        }

        for link in links {
            grid.link_cells(link.0, link.1);
        }

        grid
    }
}
