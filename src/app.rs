#[allow(unused_imports)]
use std::time::Duration;

use sdl2::event::Event;
use sdl2::event::Event::{MouseButtonDown, Quit};
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::video::{Window, WindowContext};
use sdl2::EventPump;
use sdl2::Sdl;

use crate::app_state::AppState;
use crate::managers::FontManager;
use crate::renderer::Renderer;
use sdl2::ttf::Sdl2TtfContext;

pub type WindowCanvas = Canvas<Window>;
pub type WindowTextureCreator = TextureCreator<WindowContext>;

pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 600;
pub const LEFT_MARGIN: u32 = 200;
pub const TOP_MARGIN: u32 = 100;
pub const CELL_SIZE: u32 = 20;
pub const GRID_SIZE: u32 = 400;
pub const TICKS_PER_SECOND: u32 = 60;
pub const SECOND: u32 = 1_000_000_000u32;

#[derive(Clone, Debug)]
pub enum UpdateResult {
    NoOp,
    Stop,
}

pub struct App {
    sdl_context: Sdl,
    canvas: WindowCanvas,
}

impl App {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();

        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Game of Life", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let canvas = window.into_canvas().accelerated().build().unwrap();

        Self {
            sdl_context,
            canvas,
        }
    }

    pub fn run(&mut self) {
        let sleep_time: Duration = Duration::new(0, SECOND as u32 / TICKS_PER_SECOND as u32);
        let font_context: Sdl2TtfContext = sdl2::ttf::init().unwrap();
        let font_manager: FontManager = FontManager::new(&font_context);
        let mut event_pump: EventPump = self.sdl_context.event_pump().unwrap();

        let texture_creator: WindowTextureCreator = self.canvas.texture_creator();
        let mut renderer: Renderer = Renderer::new(&font_context, &texture_creator);

        let mut app_state: AppState = AppState::new(&mut renderer);

        'running: loop {
            match self.handle_events(&mut event_pump, &mut app_state) {
                UpdateResult::Stop => break 'running,
                _ => {}
            }

            app_state.update();
            app_state.render(&mut self.canvas, &mut renderer);
            ::std::thread::sleep(sleep_time);
        }
    }

    fn handle_events(
        &mut self,
        event_pump: &mut EventPump,
        app_state: &mut AppState,
    ) -> UpdateResult {
        for event in event_pump.poll_iter() {
            match event {
                Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return UpdateResult::Stop,
                MouseButtonDown { x, y, .. } => {
                    return app_state.handle_click(x, y);
                }
                _ => UpdateResult::NoOp,
            };
        }
        UpdateResult::NoOp
    }
}
