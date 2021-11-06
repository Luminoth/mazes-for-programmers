use argh::FromArgs;

use core::algorithms::{Algorithm, BinaryTree};

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum AlgorithmOption {
    BinaryTree(BinaryTreeAlgorithm),
}

impl AlgorithmOption {
    pub fn algorithm(&self) -> impl Algorithm {
        match self {
            AlgorithmOption::BinaryTree(_) => BinaryTree::default(),
        }
    }
}

#[derive(FromArgs, PartialEq, Debug)]
/// Binary tree algorithm
#[argh(subcommand, name = "binarytree")]
pub struct BinaryTreeAlgorithm {}

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
}
