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
    #[display(fmt = "Analysis")]
    Analysis(AnalysisCommand),

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

    #[display(fmt = "Recursive Backtracker")]
    RecursiveBacktracker(RecursiveBacktrackerGenerator),
}

impl GeneratorOption {
    pub fn is_analysis(&self) -> bool {
        matches!(self, GeneratorOption::Analysis(_))
    }

    pub fn mask(&self) -> Option<PathBuf> {
        match self {
            GeneratorOption::Analysis(_) => None,
            GeneratorOption::BinaryTree(_) => None,
            GeneratorOption::Sidewinder(_) => None,
            GeneratorOption::AldousBroder(generator) => generator.mask.clone(),
            GeneratorOption::Wilsons(generator) => generator.mask.clone(),
            GeneratorOption::HuntAndKill(generator) => generator.mask.clone(),
            GeneratorOption::RecursiveBacktracker(generator) => generator.mask.clone(),
        }
    }

    pub fn generator(&self) -> Box<dyn Generator> {
        match self {
            GeneratorOption::Analysis(_) => Box::new(NoneGenerator::default()),
            GeneratorOption::BinaryTree(generator) => {
                if generator.parallel {
                    Box::new(BinaryTreeParallel::default())
                } else {
                    Box::new(BinaryTree::default())
                }
            }
            GeneratorOption::Sidewinder(generator) => {
                if generator.parallel {
                    Box::new(SidewinderParallel::default())
                } else {
                    Box::new(Sidewinder::default())
                }
            }
            GeneratorOption::AldousBroder(_) => Box::new(AldousBroder::default()),
            GeneratorOption::Wilsons(_) => Box::new(Wilsons::default()),
            GeneratorOption::HuntAndKill(_) => Box::new(HuntAndKill::default()),
            GeneratorOption::RecursiveBacktracker(_) => Box::new(RecursiveBacktracker::default()),
        }
    }

    pub fn solver_type(&self) -> SolverOption {
        match self {
            GeneratorOption::Analysis(_) => SolverOption::None(NoneSolver {}),
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
            GeneratorOption::RecursiveBacktracker(generator) => generator
                .solver
                .clone()
                .unwrap_or(SolverOption::None(NoneSolver {})),
        }
    }
}

#[derive(FromArgs, PartialEq, Debug)]
/// Run generator analysis
#[argh(subcommand, name = "analysis")]
pub struct AnalysisCommand {}

#[derive(FromArgs, PartialEq, Debug)]
/// Binary tree generator
#[argh(subcommand, name = "binarytree")]
pub struct BinaryTreeGenerator {
    /// solver to run
    #[argh(subcommand)]
    pub solver: Option<SolverOption>,

    /// run the parallelized generator
    #[argh(switch)]
    pub parallel: bool,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Sidewinder generator
#[argh(subcommand, name = "sidewinder")]
pub struct SidewinderGenerator {
    /// solver to run
    #[argh(subcommand)]
    pub solver: Option<SolverOption>,

    /// run the parallelized generator
    #[argh(switch)]
    pub parallel: bool,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Aldous-Broder generator
#[argh(subcommand, name = "aldousbroder")]
pub struct AldousBroderGenerator {
    /// solver to run
    #[argh(subcommand)]
    pub solver: Option<SolverOption>,

    /// mask the grid with the given file
    #[argh(option)]
    pub mask: Option<PathBuf>,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Wilson's algorithm generator
#[argh(subcommand, name = "wilsons")]
pub struct WilsonsGenerator {
    /// solver to run
    #[argh(subcommand)]
    pub solver: Option<SolverOption>,

    /// mask the grid with the given file
    #[argh(option)]
    pub mask: Option<PathBuf>,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Hunt-and-Kill generator
#[argh(subcommand, name = "huntandkill")]
pub struct HuntAndKillGenerator {
    /// solver to run
    #[argh(subcommand)]
    pub solver: Option<SolverOption>,

    /// mask the grid with the given file
    #[argh(option)]
    pub mask: Option<PathBuf>,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Hunt-and-Kill generator
#[argh(subcommand, name = "recursivebacktracker")]
pub struct RecursiveBacktrackerGenerator {
    /// solver to run
    #[argh(subcommand)]
    pub solver: Option<SolverOption>,

    /// mask the grid with the given file
    #[argh(option)]
    pub mask: Option<PathBuf>,
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

    /// don't render the grid
    #[argh(switch)]
    pub norender: bool,

    /// filename to render to
    #[argh(option)]
    pub filename: Option<PathBuf>,
}
