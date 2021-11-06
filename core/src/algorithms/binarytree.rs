use crate::grid::Grid;

use super::Algorithm;

use rand::Rng;

#[derive(Debug, Default)]
pub struct BinaryTree;

impl Algorithm for BinaryTree {
    fn run(&self, grid: &mut Grid) {
        for cell in grid {
            let mut rng = rand::thread_rng();

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

            let index = rng.gen_range(0..neighbors.len());
            let neighbor = neighbors[index];

            cell.link(neighbor);
        }
    }
}
