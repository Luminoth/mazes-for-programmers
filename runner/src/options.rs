use std::path::PathBuf;

use argh::FromArgs;

use core::algorithms::{Algorithm, BinaryTree, Sidewinder};

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum AlgorithmOption {
    BinaryTree(BinaryTreeAlgorithm),
    Sidewinder(SidewinderAlgorithm),
}

impl AlgorithmOption {
    pub fn algorithm(&self) -> Box<dyn Algorithm> {
        match self {
            AlgorithmOption::BinaryTree(_) => Box::new(BinaryTree::default()),
            AlgorithmOption::Sidewinder(_) => Box::new(Sidewinder::default()),
        }
    }
}

#[derive(FromArgs, PartialEq, Debug)]
/// Binary tree algorithm
#[argh(subcommand, name = "binarytree")]
pub struct BinaryTreeAlgorithm {}

#[derive(FromArgs, PartialEq, Debug)]
/// Binary tree algorithm
#[argh(subcommand, name = "sidewinder")]
pub struct SidewinderAlgorithm {}

/// Maze algorithm runner
#[derive(FromArgs, Debug)]
pub struct Options {
    /// algorithm to run
    #[argh(subcommand)]
    pub algorithm: AlgorithmOption,

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
