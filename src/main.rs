extern crate rand;
extern crate sdl2;

mod app;
mod app_state;
mod button;
mod cell_state;
mod managers;
mod map;
mod renderer;
mod ui;

fn main() {
    let mut app = app::App::new();
    app.run();
}
