use std::path::PathBuf;

use argh::FromArgs;
use derive_more::Display;

use mazecore::generators::*;
use mazecore::solvers::*;
use mazecore::Grid;

#[derive(FromArgs, PartialEq, Debug, Display)]
#[argh(subcommand)]
pub enum GeneratorOption {
    #[display(fmt = "Binary Tree")]
    BinaryTree(BinaryTreeGenerator),

    #[display(fmt = "Sidewinder")]
    Sidewinder(SidewinderGenerator),

    #[display(fmt = "Aldous-Broder")]
    AldousBroder(AldousBroderGenerator),

    #[display(fmt = "Wilson's Algorithm")]
    Wilsons(WilsonsGenerator),
}

impl GeneratorOption {
    pub fn generator(&self) -> Box<dyn Generator> {
        match self {
            GeneratorOption::BinaryTree(_) => Box::new(BinaryTree::default()),
            GeneratorOption::Sidewinder(_) => Box::new(Sidewinder::default()),
            GeneratorOption::AldousBroder(_) => Box::new(AldousBroder::default()),
            GeneratorOption::Wilsons(_) => Box::new(Wilsons::default()),
        }
    }

    pub fn solver(&self) -> &Option<SolverOption> {
        match self {
            GeneratorOption::BinaryTree(generator) => &generator.solver,
            GeneratorOption::Sidewinder(generator) => &generator.solver,
            GeneratorOption::AldousBroder(generator) => &generator.solver,
            GeneratorOption::Wilsons(generator) => &generator.solver,
        }
    }
}

#[derive(FromArgs, PartialEq, Debug)]
/// Binary tree generator
#[argh(subcommand, name = "binarytree")]
pub struct BinaryTreeGenerator {
    /// solver to run
    #[argh(subcommand)]
    pub solver: Option<SolverOption>,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Sidewinder generator
#[argh(subcommand, name = "sidewinder")]
pub struct SidewinderGenerator {
    /// solver to run
    #[argh(subcommand)]
    pub solver: Option<SolverOption>,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Binary tree generator
#[argh(subcommand, name = "aldousbroder")]
pub struct AldousBroderGenerator {
    /// solver to run
    #[argh(subcommand)]
    pub solver: Option<SolverOption>,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Binary tree generator
#[argh(subcommand, name = "wilsons")]
pub struct WilsonsGenerator {
    /// solver to run
    #[argh(subcommand)]
    pub solver: Option<SolverOption>,
}

#[derive(FromArgs, PartialEq, Debug, Display)]
#[argh(subcommand)]
pub enum SolverOption {
    #[display(fmt = "Djikstra")]
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
/// Simple Djikstra's algorithm solver
#[argh(subcommand, name = "djikstra")]
pub struct DjikstraSolver {}

/// Maze runner
#[derive(FromArgs, Debug)]
pub struct Options {
    /// generator to run
    #[argh(subcommand)]
    pub generator: GeneratorOption,

    /// grid width
    #[argh(option, default = "20")]
    pub width: usize,

    /// grid height
    #[argh(option, default = "20")]
    pub height: usize,

    /// filename to write to
    #[argh(option)]
    pub filename: Option<PathBuf>,
}
