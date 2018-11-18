use super::cell_state::CellState;
use crate::app::WindowCanvas;
use crate::app::CELL_SIZE;
use crate::app::LEFT_MARGIN;
use crate::app::TOP_MARGIN;
use crate::renderer::Render;
use crate::renderer::Renderer;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

#[derive(Clone)]
pub struct Map {
    cells: [CellState; 400],
}

impl Map {
    pub fn new() -> Self {
        Self {
            cells: [CellState::Dead; 400],
        }
    }

    pub fn get_at(&self, x: i32, y: i32) -> CellState {
        self.cells[((y * 20) + x) as usize]
    }

    pub fn set_alive(&mut self, x: i32, y: i32) {
        self.set_at(x, y, CellState::Alive);
    }

    pub fn set_dead(&mut self, x: i32, y: i32) {
        self.set_at(x, y, CellState::Dead);
    }

    fn set_at(&mut self, x: i32, y: i32, state: CellState) {
        self.cells[((y * 20) + x) as usize] = state;
    }

    pub fn get_neighbours(&self, x: i32, y: i32) -> [CellState; 9] {
        let mut d9 = [CellState::Dead; 9];
        let x = if x < 0 {
            0
        } else if x > 19 {
            19
        } else {
            x
        };
        let y = if y < 0 {
            0
        } else if y > 19 {
            19
        } else {
            y
        };
        match (x, y) {
            (0, 0) => {
                // [ 0 1 2 ]
                // [ 3 4 5 ]
                d9[4] = self.get_at(x, y);
                d9[5] = self.get_at(x + 1, y);
                // [ 6 7 8 ]
                d9[7] = self.get_at(x, y + 1);
                d9[8] = self.get_at(x + 1, y + 1);
            }
            (19, 19) => {
                // [ 0 1 2 ]
                d9[0] = self.get_at(x - 1, y - 1);
                d9[1] = self.get_at(x, y - 1);
                // [ 3 4 5 ]
                d9[3] = self.get_at(x - 1, y);
                d9[4] = self.get_at(x, y);
                // [ 6 7 8 ]
            }
            (_, 0) => {
                // [ 0 1 2 ]
                // [ 3 4 5 ]
                d9[3] = self.get_at(x - 1, y);
                d9[4] = self.get_at(x, y);
                d9[5] = self.get_at(x + 1, y);
                // [ 6 7 8 ]
                d9[6] = self.get_at(x - 1, y + 1);
                d9[7] = self.get_at(x, y + 1);
                d9[8] = self.get_at(x + 1, y + 1);
            }
            (_, 19) => {
                // [ 0 1 2 ]
                d9[0] = self.get_at(x - 1, y - 1);
                d9[1] = self.get_at(x, y - 1);
                d9[2] = self.get_at(x + 1, y - 1);
                // [ 3 4 5 ]
                d9[3] = self.get_at(x - 1, y);
                d9[4] = self.get_at(x, y);
                d9[5] = self.get_at(x + 1, y);
                // [ 6 7 8 ]
            }
            (0, _) => {
                // [ 0 1 2 ]
                d9[1] = self.get_at(x, y - 1);
                d9[2] = self.get_at(x + 1, y - 1);
                // [ 3 4 5 ]
                d9[4] = self.get_at(x, y);
                d9[5] = self.get_at(x + 1, y);
                // [ 6 7 8 ]
                d9[7] = self.get_at(x, y + 1);
                d9[8] = self.get_at(x + 1, y + 1);
            }
            (19, _) => {
                // [ 0 1 2 ]
                d9[0] = self.get_at(x - 1, y - 1);
                d9[1] = self.get_at(x, y - 1);
                // [ 3 4 5 ]
                d9[3] = self.get_at(x - 1, y);
                d9[4] = self.get_at(x, y);
                // [ 6 7 8 ]
                d9[6] = self.get_at(x - 1, y + 1);
                d9[7] = self.get_at(x, y + 1);
            }
            _ => {
                // [ 0 1 2 ]
                d9[0] = self.get_at(x - 1, y - 1);
                d9[1] = self.get_at(x, y - 1);
                d9[2] = self.get_at(x + 1, y - 1);
                // [ 3 4 5 ]
                d9[3] = self.get_at(x - 1, y);
                d9[4] = self.get_at(x, y);
                d9[5] = self.get_at(x + 1, y);
                // [ 6 7 8 ]
                d9[6] = self.get_at(x - 1, y + 1);
                d9[7] = self.get_at(x, y + 1);
                d9[8] = self.get_at(x + 1, y + 1);
            }
        }
        d9
    }

    fn get_draw_rect(x: i32, y: i32) -> Rect {
        Rect::new(
            LEFT_MARGIN as i32 + (x * CELL_SIZE as i32),
            TOP_MARGIN as i32 + (y * CELL_SIZE as i32),
            CELL_SIZE as u32,
            CELL_SIZE as u32,
        )
    }
}

impl<'a> Render for Map {
    fn render(&self, canvas: &mut WindowCanvas, renderer: &mut Renderer) {
        for y in 0..20 {
            for x in 0..20 {
                let rect = Self::get_draw_rect(x, y);
                self.get_at(x, y).draw(&rect, canvas);

                let frame_color = Color::RGBA(200, 200, 200, 255);
                renderer.render_frame(canvas, &rect, &frame_color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cell_state::*;
    use crate::map::*;
    use sdl2::rect::Rect;

    #[test]
    fn it_resolve_valid_neighbours() {
        let map = Map::new();
        assert_eq!(
            map.get_neighbours(0, 0).to_vec(),
            [CellState::Dead; 9].to_vec()
        );
        assert_eq!(
            map.get_neighbours(0, 1).to_vec(),
            [CellState::Dead; 9].to_vec()
        );
        assert_eq!(
            map.get_neighbours(1, 0).to_vec(),
            [CellState::Dead; 9].to_vec()
        );
        assert_eq!(
            map.get_neighbours(0, 19).to_vec(),
            [CellState::Dead; 9].to_vec()
        );
        assert_eq!(
            map.get_neighbours(19, 0).to_vec(),
            [CellState::Dead; 9].to_vec()
        );
        assert_eq!(
            map.get_neighbours(19, 19).to_vec(),
            [CellState::Dead; 9].to_vec()
        );
        assert_eq!(
            map.get_neighbours(1, 1).to_vec(),
            [CellState::Dead; 9].to_vec()
        );
        assert_eq!(
            map.get_neighbours(10, 10).to_vec(),
            [CellState::Dead; 9].to_vec()
        );
        assert_eq!(
            map.get_neighbours(18, 18).to_vec(),
            [CellState::Dead; 9].to_vec()
        );
        assert_eq!(
            map.get_neighbours(-1, -1).to_vec(),
            [CellState::Dead; 9].to_vec()
        );
        assert_eq!(
            map.get_neighbours(20, 20).to_vec(),
            [CellState::Dead; 9].to_vec()
        );
    }

    #[test]
    fn it_give_valid_draw_rect() {
        assert_eq!(Map::get_draw_rect(0, 0), Rect::new(200, 100, 20, 20));
        assert_eq!(Map::get_draw_rect(1, 0), Rect::new(220, 100, 20, 20));
        assert_eq!(Map::get_draw_rect(0, 1), Rect::new(200, 120, 20, 20));
        assert_eq!(Map::get_draw_rect(5, 5), Rect::new(300, 200, 20, 20));
    }
}
