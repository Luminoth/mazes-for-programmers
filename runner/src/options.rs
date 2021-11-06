use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum AlgorithmOption {
    BinaryTree(BinaryTreeAlgorithm),
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
}
