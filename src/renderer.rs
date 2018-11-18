use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::WindowContext;

use crate::managers::{FontManager, TextureManager};

use crate::app::WindowCanvas;
use sdl2::render::Texture;
use std::rc::Rc;

pub trait Render {
    fn render(&self, canvas: &mut WindowCanvas, renderer: &mut Renderer);
}

pub struct Renderer<'a, 'b> {
    pub font_manager: FontManager<'a>,
    pub texture_manager: TextureManager<'b, WindowContext>,
    pub texture_creator: &'b TextureCreator<WindowContext>,
}

impl<'a, 'b> Renderer<'a, 'b> {
    pub fn new(
        font_context: &'a Sdl2TtfContext,
        texture_creator: &'b TextureCreator<WindowContext>,
    ) -> Self {
        Self {
            font_manager: FontManager::new(&font_context),
            texture_manager: TextureManager::new(&texture_creator),
            texture_creator,
        }
    }

    pub fn present(&mut self, canvas: &mut WindowCanvas) {
        canvas.present();
    }

    pub fn clear(&mut self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
    }

    pub fn render_texture(
        &mut self,
        canvas: &mut WindowCanvas,
        tex: &Rc<Texture>,
        src: &Rect,
        dest: &Rect,
    ) {
        canvas
            .copy_ex(
                tex,
                Some(src.clone()),
                Some(dest.clone()),
                0.0,
                None,
                false,
                false,
            )
            .unwrap();
    }

    pub fn render_frame(&mut self, canvas: &mut WindowCanvas, rect: &Rect, color: &Color) {
        canvas.set_draw_color(color.clone());
        canvas.draw_rect(rect.clone()).unwrap();
    }

    pub fn render_rect(&mut self, canvas: &mut WindowCanvas, rect: &Rect, color: &Color) {
        canvas.set_draw_color(color.clone());
        canvas.fill_rect(rect.clone()).unwrap();
    }
}
