use crate::grid::Grid;

use super::Generator;

use rand::Rng;

/// Sidewinder maze generator
/// Groups adjacent cells as a run before path carving
/// Generates a perfect maze?
/// North edge path is always unbroken
#[derive(Debug, Default)]
pub struct Sidewinder;

impl Generator for Sidewinder {
    fn generate(&self, grid: &mut Grid) {
        let mut rng = rand::thread_rng();

        let mut links = Vec::default();
        for row in grid.row_iter() {
            let mut run = Vec::new();

            for cell in row {
                let cell_handle = cell.handle();

                run.push(cell_handle);

                let at_eastern_boundary = cell.east.is_none();
                let at_northern_boundary = cell.north.is_none();

                // close out a run either at the eastern border
                // or randomly within a row, except at the northern border
                let should_close_out =
                    at_eastern_boundary || (!at_northern_boundary && rng.gen_range(0..=1) == 0);

                if should_close_out {
                    let index = rng.gen_range(0..run.len());
                    let member_handle = run[index];
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
    }
}
