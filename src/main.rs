extern crate rand;
extern crate sdl2;

mod app;
mod cell_state;
mod map;

fn main() {
    let mut app = app::App::new();
    app.run();
}
