use sdl2::pixels::Color;
use sdl2::render::Texture;
use std::rc::Rc;

use crate::managers::FontDetails;
use crate::renderer::Renderer;

pub struct TextTexture<'a> {
    pub width: u32,
    pub height: u32,
    pub tex: Rc<Texture<'a>>,
}

pub fn render_text<'a>(
    font_details: &FontDetails,
    text: &str,
    color: &Color,
    main_renderer: &mut Renderer<'a, 'a>,
) -> TextTexture<'a> {
    let font = main_renderer.font_manager.load(&font_details).unwrap();

    let surface = font.render(text).blended(color.clone()).unwrap();

    let width = surface.width();
    let height = surface.height();

    let tex = Rc::new(
        main_renderer
            .texture_creator
            .create_texture_from_surface(&surface)
            .unwrap(),
    );
    TextTexture { height, width, tex }
}
