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

pub fn line(
    mut data: impl AsMut<[u8]>,
    width: usize,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    color: Color,
) {
    let data = data.as_mut();

    for y in y1..y2 {
        for x in x1..x2 {
            let index = ((y * width) + x) * 4;
            data[index] = color.r;
            data[index + 1] = color.g;
            data[index + 2] = color.b;
            data[index + 3] = color.a;
        }
    }
}
