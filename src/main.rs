#![deny(missing_docs)]

//! A simple sudoku game

extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::{Events, EventLoop, EventSettings};
use piston::input::RenderEvent;
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics, Filter, GlyphCache, TextureSettings};

pub use gameboard::Gameboard;
pub use gameboard_controller::GameboardController;
pub use gameboard_view::{GameboardView, GameboardViewSettings};

mod gameboard;
mod gameboard_controller;
mod gameboard_view;

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Sudoku", [512; 2]).opengl(opengl)
                                                          .exit_on_esc(true);
    let mut window: GlutinWindow = settings.build()
                                           .expect("Could not create window");

    // We declare the settings as lazy as we only want to render when window
    // receives input. Moreover, this will not update.
    let mut events = Events::new(EventSettings::new().lazy(true));

    // gl object will store shaders and buffers that the OpenGL backend needs
    // to talk with the GPU.
    let mut gl = GlGraphics::new(opengl);

    let mut gameboard = Gameboard::new();
    gameboard.init();
    let mut gameboard_controller = GameboardController::new(gameboard);
    let gameboard_view_settings = GameboardViewSettings::new();
    let gameboard_view = GameboardView::new(gameboard_view_settings);

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", (), texture_settings)
                                        .expect("Could not load font.");

    while let Some(e) = events.next(&mut window) {
        gameboard_controller.event(gameboard_view.settings.position,
                                   gameboard_view.settings.size,
                                   &e);
        if let Some(args) = e.render_args() {
            // The backend here implements the graphics::Graphics trait. This
            // is used to write data from Piston-Graphics into buffers.
            gl.draw(args.viewport(), |context, backend| {
                use graphics::clear;
                // Clear the screen, overwrite with white.
                clear([1.0; 4], backend);

                // Draw the gameboard.
                gameboard_view.draw(&gameboard_controller, glyphs, &context, backend);
            });
        }
    }
}
