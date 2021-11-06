use crate::grid::Grid;

use rand::Rng;

pub struct BinaryTree;

impl BinaryTree {
    pub fn run(grid: &mut Grid) {
        for cell in grid {
            let mut rng = rand::thread_rng();

            let mut neighbors = Vec::new();

            if let Some(north) = cell.north {
                neighbors.push(north);
            }

            if let Some(east) = cell.east {
                neighbors.push(east);
            }

            let index = rng.gen_range(0..neighbors.len());
            let neighbor = neighbors[index];

            cell.link(neighbor);
        }
    }
}
