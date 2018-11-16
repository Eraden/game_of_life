use super::app::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum CellState {
    Alive,
    Dead,
}

impl CellState {
    pub fn draw(&self, rect: &Rect, canvas: &mut WindowCanvas) {
        let color = match self {
            CellState::Dead => Color::RGBA(220, 220, 220, 255),
            CellState::Alive => Color::RGBA(20, 20, 20, 255),
        };
        canvas.set_draw_color(color);
        canvas.fill_rect(*rect).unwrap();
    }
}
