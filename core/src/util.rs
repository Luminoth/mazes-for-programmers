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
    image_width: usize,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    color: Color,
) {
    for y in y1..=y2 {
        for x in x1..=x2 {
            plot(data.as_mut(), image_width, x, y, color);
        }
    }
}

/// Renders a circle in the given data
// https://stackoverflow.com/questions/38334081/howto-draw-circles-arcs-and-vector-graphics-in-sdl
// https://en.wikipedia.org/wiki/Midpoint_circle_algorithm
// https://web.archive.org/web/20120422045142/https://banu.com/blog/7/drawing-circles/
pub fn circle(
    mut data: impl AsMut<[u8]>,
    image_size: (usize, usize),
    center: (usize, usize),
    radius: usize,
    color: Color,
) {
    let radius = radius as isize;
    let center = (center.0 as isize, center.1 as isize);

    let d = radius * 2;

    let mut x = radius - 1;
    let mut y = 0;
    let mut tx = 1;
    let mut ty = 1;
    let mut error = tx - d;

    while x >= y {
        let x1 = center.0 + x;
        let x2 = center.0 - x;
        let y1 = center.1 + y;
        let y2 = center.1 - y;

        // skip pixels that are out of bounds
        if x1 < 0
            || x1 >= image_size.0 as isize
            || x2 < 0
            || x2 >= image_size.0 as isize
            || y1 < 0
            || y1 >= image_size.1 as isize
            || y2 < 0
            || y2 >= image_size.1 as isize
        {
            continue;
        }

        let x1 = x1 as usize;
        let x2 = x2 as usize;
        let y1 = y1 as usize;
        let y2 = y2 as usize;

        // each of the following renders an octant of the circle
        plot(data.as_mut(), image_size.0, x1, y1, color);
        plot(data.as_mut(), image_size.0, x1, y2, color);
        plot(data.as_mut(), image_size.0, x2, y1, color);
        plot(data.as_mut(), image_size.0, x2, y2, color);
        plot(data.as_mut(), image_size.0, y1, x1, color);
        plot(data.as_mut(), image_size.0, y1, x2, color);
        plot(data.as_mut(), image_size.0, y2, x1, color);
        plot(data.as_mut(), image_size.0, y2, x2, color);

        if error <= 0 {
            y += 1;
            error += ty;
            ty += 2;
        }

        if error > 0 {
            x -= 1;
            tx += 2;
            error += tx - d;
        }
    }
}

/// Renders a line in the given data
// https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
pub fn line(
    mut data: impl AsMut<[u8]>,
    image_size: (usize, usize),
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    color: Color,
) {
    let mut x1 = x1 as isize;
    let x2 = x2 as isize;
    let mut y1 = y1 as isize;
    let y2 = y2 as isize;

    let dx = (x2 - x1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let dy = -((y2 - y1).abs());
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        plot(data.as_mut(), image_size.0, x1 as usize, y1 as usize, color);

        if x1 == x2 && y1 == y2 {
            break;
        }

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x1 += sx;
        }

        if e2 <= dx {
            err += dx;
            y1 += sy;
        }
    }
}

fn plot(mut data: impl AsMut<[u8]>, image_width: usize, x: usize, y: usize, color: Color) {
    let data = data.as_mut();

    let index = ((y * image_width) + x) * 4;
    data[index] = color.r;
    data[index + 1] = color.g;
    data[index + 2] = color.b;
    data[index + 3] = color.a;
}

/// Random coin flip
pub fn coin() -> bool {
    let mut rng = rand::thread_rng();

    rng.gen_range(0..=1) == 0
}

/// Returns a random item from the given set
pub fn sample<T>(items: &[T]) -> &T {
    let mut rng = rand::thread_rng();

    let index = rng.gen_range(0..items.as_ref().len());
    &items[index]
}
