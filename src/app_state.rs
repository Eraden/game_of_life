use sdl2::pixels::Color;
use sdl2::rect::{ Rect, Point };

use crate::map::Map;

use crate::button::Button;

use crate::renderer::{Render, Renderer};

use crate::cell_state::CellState;

use crate::app::CELL_SIZE;
use crate::app::GRID_SIZE;
use crate::app::LEFT_MARGIN;
use crate::app::TICKS_PER_SECOND;
use crate::app::TOP_MARGIN;
use crate::app::{UpdateResult, WindowCanvas};

#[derive(Debug, Clone, PartialEq)]
pub enum AppStatus {
    Running,
    Paused,
}

#[derive(Clone)]
pub struct AppState<'a> {
    map: Map,
    pause_button: Button<'a>,
    run_button: Button<'a>,
    status: AppStatus,
    logic_counter: i32,
}

impl<'a> AppState<'a> {
    pub fn new(renderer: &mut Renderer<'a, 'a>) -> Self {
        Self {
            map: Map::new(),
            pause_button: Button::new(
                renderer,
                "Pause",
                Rect::new(400, 510, 200, 70),
                Color::RGBA(255, 255, 50, 255),
                Color::RGBA(0, 0, 0, 255),
            ),
            run_button: Button::new(
                renderer,
                "Run",
                Rect::new(400, 510, 200, 70),
                Color::RGBA(0, 255, 0, 255),
                Color::RGBA(0, 0, 0, 255),
            ),
            status: AppStatus::Paused,
            logic_counter: AppState::default_logic_counter(),
        }
    }

    pub fn render(&self, canvas: &mut WindowCanvas, renderer: &mut Renderer<'a, 'a>) {
        renderer.clear(canvas);
        self.map.render(canvas, renderer);
        match self.status {
            AppStatus::Running => {
                self.pause_button.render(canvas, renderer);
            }
            AppStatus::Paused => {
                self.run_button.render(canvas, renderer);
            }
        }
        renderer.present(canvas);
    }

    pub fn update(&mut self) {
        match self.status {
            AppStatus::Running => {
                self.logic_counter -= 1;
                if self.logic_counter < 0 {
                    self.logic_counter = AppState::default_logic_counter();
                    self.next_generation();
                }
            }
            AppStatus::Paused => {}
        }
    }

    fn next_generation(&mut self) {
        for y in 0..20 {
            for x in 0..20 {
                let mut neighbours = self.map.get_neighbours(x, y);
                let mut s = 0;
                for neighbour in &mut neighbours {
                    if *neighbour == CellState::Alive {
                        s += 1;
                    }
                }
                match self.map.get_at(x, y) {
                    CellState::Alive => {
                        let s = s - 1;
                        if s != 3 && s != 2 {
                            self.map.set_dead(x, y);
                        }
                    }
                    CellState::Dead => {
                        if s == 3 {
                            self.map.set_alive(x, y);
                        }
                    }
                }
            }
        }
    }

    fn default_logic_counter() -> i32 {
        (TICKS_PER_SECOND / 2) as i32
    }

    pub fn handle_click(&mut self, x: i32, y: i32) -> UpdateResult {
        match (x, y) {
            _cell if self.is_cell(x, y) => {
                let nx: usize = (x - LEFT_MARGIN as i32) as usize / CELL_SIZE as usize;
                let ny: usize = (y - TOP_MARGIN as i32) as usize / CELL_SIZE as usize;
                self.map.set_alive(nx as i32, ny as i32);
                UpdateResult::NoOp
            }
            _run if self.is_run_button(x, y) => {
                self.status = AppStatus::Running;
                println!("Button clicked (run)");
                UpdateResult::NoOp

            }
            _pause if self.is_pause_button(x, y) => {
                self.status = AppStatus::Paused;
                println!("Button clicked (pause)");
                UpdateResult::NoOp

            }
            _ => UpdateResult::NoOp,
        }
    }

    fn is_cell(&self, x: i32, y: i32) -> bool {
        x >= LEFT_MARGIN as i32
            && x < (GRID_SIZE + LEFT_MARGIN) as i32
            && y >= TOP_MARGIN as i32
            && y < (GRID_SIZE + TOP_MARGIN) as i32
    }

    fn is_run_button(&self, x: i32, y: i32) -> bool {
        match self.status {
            AppStatus::Paused => self.run_button.contains(Point::new(x, y)),
            AppStatus::Running => false,
        }
    }

    fn is_pause_button(&self, x: i32, y: i32) -> bool {
        match self.status {
            AppStatus::Running => self.pause_button.contains(Point::new(x, y)),
            AppStatus::Paused => false,
        }
    }
}
