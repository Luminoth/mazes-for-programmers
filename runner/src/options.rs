use std::path::PathBuf;

use argh::FromArgs;

use core::generators::{BinaryTree, Generator, Sidewinder};
use core::solvers::{Djikstra, Solver};
use core::Grid;

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum GeneratorOption {
    BinaryTree(BinaryTreeGenerator),
    Sidewinder(SidewinderGenerator),
}

impl GeneratorOption {
    pub fn generator(&self) -> Box<dyn Generator> {
        match self {
            GeneratorOption::BinaryTree(_) => Box::new(BinaryTree::default()),
            GeneratorOption::Sidewinder(_) => Box::new(Sidewinder::default()),
        }
    }

    pub fn solver(&self) -> &SolverOption {
        match self {
            GeneratorOption::BinaryTree(generator) => &generator.solver,
            GeneratorOption::Sidewinder(generator) => &generator.solver,
        }
    }
}

#[derive(FromArgs, PartialEq, Debug)]
/// Binary tree generator
#[argh(subcommand, name = "binarytree")]
pub struct BinaryTreeGenerator {
    /// solver to run
    #[argh(subcommand)]
    pub solver: SolverOption,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Sidewinder generator
#[argh(subcommand, name = "sidewinder")]
pub struct SidewinderGenerator {
    /// solver to run
    #[argh(subcommand)]
    pub solver: SolverOption,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum SolverOption {
    Djikstra(DjikstraSolver),
}

impl SolverOption {
    pub fn solver(&self, grid: Grid, root_row: usize, root_col: usize) -> Box<dyn Solver> {
        match self {
            SolverOption::Djikstra(_) => Box::new(Djikstra::new(grid, root_row, root_col)),
        }
    }
}

#[derive(FromArgs, PartialEq, Debug)]
/// Binary tree generator
#[argh(subcommand, name = "djikstra")]
pub struct DjikstraSolver {}

/// Maze runner
#[derive(FromArgs, Debug)]
pub struct Options {
    /// generator to run
    #[argh(subcommand)]
    pub generator: GeneratorOption,

    /// grid width
    #[argh(option, default = "10")]
    pub width: usize,

    /// grid height
    #[argh(option, default = "10")]
    pub height: usize,

    /// filename to write to
    #[argh(option)]
    pub filename: Option<PathBuf>,
}
