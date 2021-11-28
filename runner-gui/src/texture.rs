use eframe::{egui, epi};

use crate::image::Image;

// taken from the egui_demo_lib's http_app demo:
// https://github.com/emilk/egui/blob/master/egui_demo_lib/src/apps/http_app.rs

#[derive(Debug, Default)]
pub struct Texture {
    id: Option<egui::TextureId>,
    size: Option<egui::Vec2>,
}

impl Texture {
    pub fn id(&self) -> Option<egui::TextureId> {
        self.id
    }

    pub fn size(&self) -> egui::Vec2 {
        self.size.unwrap_or_default()
    }

    pub fn load(&mut self, frame: &mut epi::Frame<'_>, image: &Image) {
        self.unload(frame);

        self.id = Some(
            frame
                .tex_allocator()
                .alloc_srgba_premultiplied(image.size(), image.pixels()),
        );

        let size = image.size();
        self.size = Some(egui::Vec2::new(size.0 as f32, size.1 as f32));
    }

    pub fn unload(&mut self, frame: &mut epi::Frame<'_>) {
        if let Some(id) = self.id.take() {
            frame.tex_allocator().free(id);
        }

        self.size = None;
    }
}
