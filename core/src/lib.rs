mod cell;
mod distances;
pub mod generators;
pub mod grid;
pub mod mask;
pub mod solvers;
mod util;

use std::io;
use std::path::Path;

use cell::*;
use distances::*;
pub use grid::*;
pub use mask::*;

/// Implement this trait to allow rendering a maze
pub trait Renderable {
    /// Renders the renderable to the CLI
    fn render_ascii(&self) -> String;

    /// Saves the renderable as a PNG at the given path
    fn save_png(&self, path: &Path, cell_size: usize) -> io::Result<()>;
}
