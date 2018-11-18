use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::surface::Surface;
use sdl2::ttf::Font;

use std::rc::Rc;

use crate::app::WindowCanvas;
use crate::managers::FontDetails;
use crate::renderer::{Render, Renderer};
use crate::ui::render_text;
use crate::ui::TextTexture;
use sdl2::rect::Point;

#[derive(Clone)]
pub struct Button<'a> {
    text_texture: Rc<Texture<'a>>,
    text_src: Rect,
    text_dest: Rect,
    pub background_rect: Rect,
    background_color: Color,
}

impl<'a> Button<'a> {
    pub fn new(
        renderer: &mut Renderer<'a, 'a>,
        text: &str,
        background_rect: Rect,
        background_color: Color,
        text_color: Color,
    ) -> Self {
        let r = background_rect.clone();
        let font_details = FontDetails::new("./assets/fonts/Beyond Wonderland.ttf", 30);
        let tex: TextTexture<'a> = render_text(&font_details, text, &text_color, renderer);
        let text_src = Rect::new(r.x(), r.y(), tex.width, tex.height);
        let text_dest = Rect::new(r.x(), r.y(), tex.width, tex.height);

        Self {
            text_texture: tex.tex.clone(),
            text_src,
            text_dest,
            background_rect,
            background_color,
        }
    }

    pub fn contains(&self, point: Point) -> bool {
        self.background_rect.contains(point)
    }
}

impl<'a> Render for Button<'a> {
    fn render(&self, canvas: &mut WindowCanvas, renderer: &mut Renderer) {
        renderer.render_rect(canvas, &self.background_rect, &self.background_color);
        renderer.render_texture(canvas, &self.text_texture, &self.text_src, &self.text_dest);
    }
}
