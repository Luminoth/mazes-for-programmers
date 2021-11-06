use crate::grid::Grid;

use super::Algorithm;

#[derive(Debug, Default)]
pub struct Sidewinder;

impl Algorithm for Sidewinder {
    fn run(&self, grid: &mut Grid) {}
}
