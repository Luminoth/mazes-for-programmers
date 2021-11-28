use eframe::egui;

// taken from the egui_demo_lib's http_app demo

#[derive(Debug, Clone)]
pub struct Image {
    size: (usize, usize),
    pixels: Vec<egui::Color32>,
}

impl Image {
    pub fn from_pixels(size: (usize, usize), pixels: impl AsRef<[u8]>) -> Self {
        assert!(size.0 * size.1 * 4 == pixels.as_ref().len());

        // convert to egui format
        let pixels = pixels
            .as_ref()
            .chunks(4)
            .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
            .collect();

        Self { size, pixels }
    }

    pub fn size(&self) -> (usize, usize) {
        self.size
    }

    pub fn pixels(&self) -> &Vec<egui::Color32> {
        &self.pixels
    }
}
