use std::path::PathBuf;

use argh::FromArgs;
use derive_more::Display;

use mazecore::generators::*;
use mazecore::solvers::*;
use mazecore::Grid;

// TODO: all of this would be cleaner with macros

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

    #[display(fmt = "Hunt-and-Kill")]
    HuntAndKill(HuntAndKillGenerator),
}

impl GeneratorOption {
    pub fn generator(&self) -> Box<dyn Generator> {
        match self {
            GeneratorOption::BinaryTree(_) => Box::new(BinaryTree::default()),
            GeneratorOption::Sidewinder(_) => Box::new(Sidewinder::default()),
            GeneratorOption::AldousBroder(_) => Box::new(AldousBroder::default()),
            GeneratorOption::Wilsons(_) => Box::new(Wilsons::default()),
            GeneratorOption::HuntAndKill(_) => Box::new(HuntAndKill::default()),
        }
    }

    pub fn solver_type(&self) -> SolverOption {
        match self {
            GeneratorOption::BinaryTree(generator) => generator
                .solver
                .clone()
                .unwrap_or(SolverOption::None(NoneSolver {})),
            GeneratorOption::Sidewinder(generator) => generator
                .solver
                .clone()
                .unwrap_or(SolverOption::None(NoneSolver {})),
            GeneratorOption::AldousBroder(generator) => generator
                .solver
                .clone()
                .unwrap_or(SolverOption::None(NoneSolver {})),
            GeneratorOption::Wilsons(generator) => generator
                .solver
                .clone()
                .unwrap_or(SolverOption::None(NoneSolver {})),
            GeneratorOption::HuntAndKill(generator) => generator
                .solver
                .clone()
                .unwrap_or(SolverOption::None(NoneSolver {})),
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

#[derive(FromArgs, PartialEq, Debug)]
/// Binary tree generator
#[argh(subcommand, name = "huntandkill")]
pub struct HuntAndKillGenerator {
    /// solver to run
    #[argh(subcommand)]
    pub solver: Option<SolverOption>,
}

#[derive(FromArgs, PartialEq, Debug, Display, Clone)]
#[argh(subcommand)]
pub enum SolverOption {
    #[display(fmt = "None")]
    None(NoneSolver),

    #[display(fmt = "Djikstra")]
    Djikstra(DjikstraSolver),
}

impl SolverOption {
    pub fn solver(&self, grid: Grid, root_row: usize, root_col: usize) -> Box<dyn Solver> {
        match self {
            SolverOption::None(_) => Box::new(mazecore::solvers::NoneSolver::new(grid)),
            SolverOption::Djikstra(_) => Box::new(Djikstra::new(grid, root_row, root_col)),
        }
    }
}

#[derive(FromArgs, PartialEq, Debug, Clone)]
/// Solver that doesn't solve anything
#[argh(subcommand, name = "none")]
pub struct NoneSolver {}

#[derive(FromArgs, PartialEq, Debug, Clone)]
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

    /// filename to render to
    #[argh(option)]
    pub filename: Option<PathBuf>,
}
