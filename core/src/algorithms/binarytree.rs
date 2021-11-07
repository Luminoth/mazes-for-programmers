use crate::grid::Grid;

use super::Algorithm;

use rand::Rng;

/// BinaryTree maze generator
/// Selects random N/E neighbors to link
/// Generates a perfect maze
/// North / East edge paths are always unbroken
#[derive(Debug, Default)]
pub struct BinaryTree;

impl Algorithm for BinaryTree {
    fn run(&self, grid: &mut Grid) {
        let mut rng = rand::thread_rng();

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
            let index = rng.gen_range(0..neighbors.len());
            let neighbor = neighbors[index];

            links.push((cell.handle(), neighbor));
        }

        for link in links {
            grid.link_cells(link.0, link.1);
        }
    }
}
