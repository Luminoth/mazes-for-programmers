use std::path::PathBuf;

use argh::FromArgs;

use core::generators::{BinaryTree, Generator, Sidewinder};

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
}

#[derive(FromArgs, PartialEq, Debug)]
/// Binary tree generator
#[argh(subcommand, name = "binarytree")]
pub struct BinaryTreeGenerator {}

#[derive(FromArgs, PartialEq, Debug)]
/// Sidewinder generator
#[argh(subcommand, name = "sidewinder")]
pub struct SidewinderGenerator {}

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
