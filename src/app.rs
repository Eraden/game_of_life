#[allow(unused_imports)]
use std::time::Duration;

use sdl2::event::Event;
use sdl2::event::Event::{MouseButtonDown, Quit};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::{Sdl};

use super::cell_state::CellState;
use super::map::Map;

pub type WindowCanvas = Canvas<Window>;

#[derive(Clone, Debug)]
enum UpdateResult {
    NoOp,
    Stop,
}

pub struct App {
    map: Map,
    sdl_context: Sdl,
    canvas: WindowCanvas,
}

#[derive(Clone, Debug)]
struct Config {
    window_width: u32,
    window_height: u32,
}

impl Config {
    pub fn new() -> Self {
        Self {
            window_width: 800,
            window_height: 600,
        }
    }
}

impl App {
    pub fn new() -> Self {
        let config = Config::new();
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Game of Life", config.window_width, config.window_height)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let canvas = window.into_canvas().accelerated().build().unwrap();

        Self {
            map: Map::new(),
            sdl_context,
            canvas,
        }
    }

    pub fn run(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let sleep_time = Duration::new(0, 1_000_000_000u32 / 60);
        let mut logic_counter = App::default_logic_counter();

        'running: loop {
            match self.handle_events(&mut event_pump) {
                UpdateResult::Stop => break 'running,
                _ => {}
            }

            self.clear();
            logic_counter -= 1;
            if logic_counter < 0 {
                logic_counter = App::default_logic_counter();
                self.update();
            }
            self.render();
            self.present();
            ::std::thread::sleep(sleep_time);
        }
    }

    fn default_logic_counter() -> i32 {
        2 * 60
    }

    fn handle_events(&mut self, event_pump: &mut EventPump) -> UpdateResult {
        for event in event_pump.poll_iter() {
            match event {
                Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return UpdateResult::Stop,
                MouseButtonDown { x, y, .. } => {
                    if x >= 200 && x < 600 && y >= 100 && y < 500 {
                        let nx: usize = (x - 200) as usize / 20;
                        let ny: usize = (y - 100) as usize / 20;
                        self.map.set_alive(nx as i32, ny as i32);
                    }
                    UpdateResult::NoOp
                }
                _ => UpdateResult::NoOp,
            };
        }
        UpdateResult::NoOp
    }

    fn present(&mut self) {
        self.canvas.present();
    }

    fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();
    }

    fn render(&mut self) {
        for y in 0..20 {
            for x in 0..20 {
                let rect = App::get_draw_rect(x, y);

                self.map.get_at(x, y).draw(&rect, &mut self.canvas);
                self.canvas.set_draw_color(Color::RGBA(200, 200, 200, 255));

                self.canvas.draw_rect(rect).unwrap();
            }
        }
    }

    fn update(&mut self) {
        for y in 0..20 {
            for x in 0..20 {
                let neighbours = self.map.get_neighbours(x, y).to_vec();
                let res: Vec<&CellState> = neighbours
                    .iter()
                    .filter(|t| **t == CellState::Alive)
                    .collect();
                let s = res.len();
                let _z = self.map.get_at(x, y);
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

    fn get_draw_rect(x: i32, y: i32) -> Rect {
        Rect::new(200 + (x * 20), 100 + (y * 20), 20, 20)
    }
}

#[cfg(test)]
mod tests {
    use crate::app::*;
    use sdl2::rect::Rect;

    #[test]
    fn it_give_valid_draw_rect() {
        assert_eq!(App::get_draw_rect(0, 0), Rect::new(200, 100, 20, 20));
        assert_eq!(App::get_draw_rect(1, 0), Rect::new(220, 100, 20, 20));
        assert_eq!(App::get_draw_rect(0, 1), Rect::new(200, 120, 20, 20));
        assert_eq!(App::get_draw_rect(5, 5), Rect::new(300, 200, 20, 20));
    }
}
