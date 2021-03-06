use std::fs;
use std::io;
use std::path::Path;

use bit_vec::BitVec;
use rand::Rng;
use tracing::{debug, info};

use crate::util::read_file_lines;

/// Masks can be used to specify which cells in a grid are enabled or disabled
#[derive(Debug, Clone)]
pub struct Mask {
    pub(crate) rows: usize,
    pub(crate) cols: usize,

    bits: BitVec,
}

impl Mask {
    /// Creates a new mask
    pub fn new(rows: usize, cols: usize) -> Self {
        assert!(rows > 0 && cols > 0);

        let size = rows * cols;
        Self {
            rows,
            cols,

            bits: BitVec::from_elem(size, true),
        }
    }

    fn remove_comments(lines: Vec<String>) -> Vec<String> {
        lines
            .iter()
            .cloned()
            .filter(|line| !line.starts_with('#'))
            .collect()
    }

    /// Creates a new mask from a file
    pub fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        info!("Reading mask from file {:?} ...", path.as_ref());

        let lines: Vec<String> = Self::remove_comments(
            read_file_lines(path)?
                .iter()
                .cloned()
                .filter(|x| !x.is_empty())
                .collect(),
        );
        if lines.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Invalid mask - no rows",
            ));
        }

        if !lines.iter().all(|x| x.len() == lines[0].len()) {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Invalid mask - column length mismatch",
            ));
        }

        let mut mask = Mask::new(lines.len(), lines[0].len());

        for (rowi, row) in lines.iter().enumerate() {
            for (coli, ch) in row.chars().enumerate() {
                if ch == 'x' || ch == 'X' {
                    mask.set(rowi, coli, false);
                }
            }
        }

        Ok(mask)
    }

    /// Creates a new mask from an image
    pub fn from_image(path: impl AsRef<Path>) -> io::Result<Self> {
        info!("Reading mask from image {:?} ...", path.as_ref());

        let file = fs::File::open(path)?;
        let decoder = png::Decoder::new(file);
        let mut reader = decoder.read_info()?;
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf)?;

        if info.color_type != png::ColorType::Rgba {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Invalid mask - must be rgba",
            ));
        }

        if info.bit_depth != png::BitDepth::Eight {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Invalid mask - bit depth must be 8",
            ));
        }

        debug!("data size: {}", info.buffer_size());
        let data = &buf[..info.buffer_size()];

        let mut mask = Mask::new(info.height as usize, info.width as usize);

        for row in 0..mask.rows {
            for col in (0..mask.cols * 4).step_by(4) {
                // transparency disables cells
                let index = mask.index(row, col);
                if data[index + 3] != 255 {
                    mask.set(row, col, false);
                }
            }
        }

        Ok(mask)
    }

    /// Returns the size of the mask
    pub fn size(&self) -> usize {
        self.rows * self.cols
    }

    pub(crate) fn index(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    /// Gets the enabled value of the given cell
    pub fn get(&self, row: usize, col: usize) -> bool {
        let index = self.index(row, col);
        self.bits.get(index).unwrap()
    }

    /// Sets the enabled value of the given cell
    pub fn set(&mut self, row: usize, col: usize, v: bool) {
        let index = self.index(row, col);
        self.bits.set(index, v);
    }

    /// Returns the number of enabled cells in the mask
    pub fn count(&self) -> usize {
        self.bits.iter().filter(|x| *x).count()
    }

    /// Returns a random enabled cell
    pub fn get_random(&self) -> (usize, usize) {
        assert!(self.bits.any());

        let mut rng = rand::thread_rng();

        // TODO: this could be smarter and avoid looping
        loop {
            let row = rng.gen_range(0..self.rows);
            let col = rng.gen_range(0..self.cols);

            if self.get(row, col) {
                return (row, col);
            }
        }
    }

    /// Returns the first enabled cell
    pub fn get_first_enabled(&self) -> Option<(usize, usize)> {
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.get(row, col) {
                    return Some((row, col));
                }
            }
        }
        None
    }
}
