#[derive(Debug, Default, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

/// Bresenham's line algorithm
pub fn line(
    data: impl AsMut<[u8]>,
    width: usize,
    x1: isize,
    y1: isize,
    x2: isize,
    y2: isize,
    color: Color,
) {
    #[allow(clippy::collapsible_else_if)]
    if (y2 - y1).abs() < (x2 - x1).abs() {
        if x1 > x2 {
            line_low(data, width, x2, y2, x1, y1, color);
        } else {
            line_low(data, width, x1, y1, x2, y2, color);
        }
    } else {
        if x1 > x2 {
            line_high(data, width, x2, y2, x1, y1, color);
        } else {
            line_high(data, width, x1, y1, x2, y2, color);
        }
    }
}

fn line_low(
    mut data: impl AsMut<[u8]>,
    width: usize,
    x1: isize,
    y1: isize,
    x2: isize,
    y2: isize,
    color: Color,
) {
    let dx = x2 - x1;
    let mut dy = y2 - y1;
    let mut yi = 1;
    if dy < 0 {
        yi = -1;
        dy = -dy;
    }

    let mut d = (2 * dy) - dx;
    let mut y = y1;

    for x in x1..x2 {
        plot(data.as_mut(), width, x, y, color);

        if d > 0 {
            y += yi;
            d += 2 * (dy - dx);
        } else {
            d += 2 * dy;
        }
    }
}

fn line_high(
    mut data: impl AsMut<[u8]>,
    width: usize,
    x1: isize,
    y1: isize,
    x2: isize,
    y2: isize,
    color: Color,
) {
    let mut dx = x2 - x1;
    let dy = y2 - y1;
    let mut xi = 1;
    if dx < 0 {
        xi = -1;
        dx = -dx;
    }

    let mut d = (2 * dx) - dy;
    let mut x = x1;

    for y in y1..y2 {
        plot(data.as_mut(), width, x, y, color);

        if d > 0 {
            x += xi;
            d += 2 * (dx - dy);
        } else {
            d += 2 * dx;
        }
    }
}

fn plot(mut data: impl AsMut<[u8]>, width: usize, x: isize, y: isize, color: Color) {
    let data = data.as_mut();

    let index = (((y * width as isize) + x) * 4) as usize;
    data[index] = color.r;
    data[index + 1] = color.g;
    data[index + 2] = color.b;
    data[index + 3] = color.a;
}
