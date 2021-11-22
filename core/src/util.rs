use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

use rand::Rng;

/// Reads a file, removing empty lines
pub fn read_file_lines_no_empty(path: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);

    // TODO: pass up the io error here rather than unwrapping it
    let lines: Vec<String> = reader
        .lines()
        .map(Result::unwrap)
        .map(|x| x.trim().to_string())
        .filter(|x| !x.is_empty())
        .collect();

    Ok(lines)
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const WHITE: Color = Color::new(255, 255, 255, 255);

    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

/// Renders a quad in the given data
pub fn quad(
    mut data: impl AsMut<[u8]>,
    width: usize,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    color: Color,
) {
    for y in y1..=y2 {
        for x in x1..=x2 {
            plot(data.as_mut(), width, x, y, color);
        }
    }
}

/// Renders a horizontal line in the given data
pub fn horizontal_line(
    mut data: impl AsMut<[u8]>,
    width: usize,
    x1: usize,
    x2: usize,
    y: usize,
    color: Color,
) {
    for x in x1..=x2 {
        plot(data.as_mut(), width, x, y, color);
    }
}

/// Renders a vertical line in the given data
pub fn vertical_line(
    mut data: impl AsMut<[u8]>,
    width: usize,
    x: usize,
    y1: usize,
    y2: usize,
    color: Color,
) {
    for y in y1..=y2 {
        plot(data.as_mut(), width, x, y, color);
    }
}

fn plot(mut data: impl AsMut<[u8]>, width: usize, x: usize, y: usize, color: Color) {
    let data = data.as_mut();

    let index = ((y * width) + x) * 4;
    data[index] = color.r;
    data[index + 1] = color.g;
    data[index + 2] = color.b;
    data[index + 3] = color.a;
}

pub fn coin() -> bool {
    let mut rng = rand::thread_rng();

    rng.gen_range(0..=1) == 0
}

pub fn sample<T>(items: &[T]) -> &T {
    let mut rng = rand::thread_rng();

    let index = rng.gen_range(0..items.as_ref().len());
    &items[index]
}
