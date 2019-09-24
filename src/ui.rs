//! This module contains logic for rendering a Game of Life simulation using SDL2.
//!
//! You're welcome to read it to learn more Rust syntax or style - there are no spoilers here.
//!
//! But you probably don't need to change it, unless you want to extend the UI with new behavior.

use crate::game_of_life::GameOfLife;
use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use std::time::{Duration, SystemTime};

/// Configuration settings for the UI.
///
/// You should instantiate this struct directly and pass it to `run_game` - no builder pattern required.
pub struct UiOptions {
    /// How many milliseconds should elapse before we update the game.
    /// E.g. set to 500 to update twice a second, or set to 0 to update each frame.
    pub millis_between_ticks: u64,

    /// How wide/high each cell in the game of life display should be, in pixels.
    ///
    /// Should be a power of 2; 8 or 16 are suitable for small to medium patterns.
    /// Options lower than 4 will result in rendering artifacts, so that's disallowed.
    pub square_size: i32,
}

impl UiOptions {
    fn ready_for_next_tick(&self, time_since_last_tick: Duration) -> bool {
        let elapsed_ms = time_since_last_tick.as_secs() * 1_000
            + time_since_last_tick.subsec_nanos() as u64 / 1_000_000;
        elapsed_ms >= self.millis_between_ticks
    }
}

pub fn run_game(mut game: Box<dyn GameOfLife>, options: &UiOptions) {
    assert!(
        options.square_size >= 4,
        "UI's configured square_size should be at least 4"
    );

    let mut sim = Simulation::new();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // the window is the representation of a window in your operating system,
    // however you can only manipulate properties of that window, like its size, whether it's
    // fullscreen, ... but you cannot change its content without using a Canvas or using the
    // `surface()` method.
    let window = video_subsystem
        .window(
            "RustLife",
            (options.square_size * game.width()) as u32,
            (options.square_size * game.height()) as u32,
        )
        .position_centered()
        .build()
        .unwrap();

    // the canvas allows us to both manipulate the property of the window and to change its content
    // via hardware or software rendering. See CanvasBuilder for more info.
    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .unwrap();

    println!("Using SDL_Renderer \"{}\"", canvas.info().name);
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    // clears the canvas with the color we set in `set_draw_color`.
    canvas.clear();
    // However the canvas has not been updated to the window yet, everything has been processed to
    // an internal buffer, but if we want our buffer to be displayed on the window, we need to call
    // `present`. We need to call this every time we want to render a new frame on the window.
    canvas.present();

    // this struct manages textures. For lifetime reasons, the canvas cannot directly create
    // textures, you have to create a `TextureCreator` instead.
    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    let (playing_texture, paused_texture) =
        generate_textures(&mut canvas, &texture_creator, options.square_size as u32);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut last_tick_time = SystemTime::now();
    'running: loop {
        // get the inputs here
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    repeat: false,
                    ..
                } => {
                    sim.toggle_state();
                }
                Event::MouseButtonDown {
                    x,
                    y,
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    let cell_x = x / options.square_size;
                    let cell_y = y / options.square_size;
                    println!(
                        "Attempting to toggle cell at {}, {} due to mouse click at {}, {}",
                        cell_x, cell_y, x, y
                    );
                    game.toggle_cell(cell_x as i32, cell_y as i32);
                }
                _ => {}
            }
        }

        // update the game loop here
        if sim.state == SimulationState::Playing {
            match last_tick_time.elapsed() {
                Ok(duration) if options.ready_for_next_tick(duration) => {
                    game.tick();
                    last_tick_time = SystemTime::now();
                }
                _ => {
                    // clock drift or not enough time has elapsed since last tick - do nothing yet
                }
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let square_texture = if sim.state == SimulationState::Playing {
            &playing_texture
        } else {
            &paused_texture
        };
        // there are more efficient ways to iterate over all cells, but the API used here is easiest
        // to implement for people with little to no Rust experience, so we'll stick with this.
        for x in 0..game.width() {
            for y in 0..game.height() {
                match game.is_cell_alive(x as i32, y as i32) {
                    Some(true) => canvas
                        .copy(
                            square_texture,
                            None,
                            Rect::new(
                                (x * options.square_size) as i32,
                                (y * options.square_size) as i32,
                                options.square_size as u32,
                                options.square_size as u32,
                            ),
                        )
                        .unwrap(),
                    Some(false) => (), // do nothing, empty canvas block is sufficient for a dead cell
                    None => panic!(
                        "logic error in checking cell liveness! x={}, y={} and got no result",
                        x, y
                    ),
                }
            }
        }

        canvas.present();
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum SimulationState {
    Paused,
    Playing,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Simulation {
    state: SimulationState,
}

impl Simulation {
    fn new() -> Simulation {
        Simulation {
            state: SimulationState::Paused,
        }
    }

    fn toggle_state(&mut self) {
        self.state = match self.state {
            SimulationState::Paused => SimulationState::Playing,
            SimulationState::Playing => SimulationState::Paused,
        }
    }
}

fn generate_textures<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    square_size: u32,
) -> (Texture<'a>, Texture<'a>) {
    enum TextureColor {
        Yellow,
        White,
    };
    let mut square_texture1: Texture = texture_creator
        .create_texture_target(None, square_size, square_size)
        .unwrap();
    let mut square_texture2: Texture = texture_creator
        .create_texture_target(None, square_size, square_size)
        .unwrap();
    // let's change the textures we just created
    {
        let textures = vec![
            (&mut square_texture1, TextureColor::Yellow),
            (&mut square_texture2, TextureColor::White),
        ];
        canvas
            .with_multiple_texture_canvas(textures.iter(), |texture_canvas, user_context| {
                texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
                texture_canvas.clear();
                match *user_context {
                    TextureColor::Yellow => {
                        for i in 0..square_size {
                            for j in 0..square_size {
                                if (i + j) % 4 == 0 {
                                    texture_canvas.set_draw_color(Color::RGB(255, 255, 0));
                                    texture_canvas
                                        .draw_point(Point::new(i as i32, j as i32))
                                        .unwrap();
                                }
                                if (i + j * 2) % 9 == 0 {
                                    texture_canvas.set_draw_color(Color::RGB(200, 200, 0));
                                    texture_canvas
                                        .draw_point(Point::new(i as i32, j as i32))
                                        .unwrap();
                                }
                            }
                        }
                    }
                    TextureColor::White => {
                        for i in 0..square_size {
                            for j in 0..square_size {
                                // drawing pixel by pixel isn't very effective, but we only do it once and store
                                // the texture afterwards so it's still alright!
                                if (i + j) % 7 == 0 {
                                    // this doesn't mean anything, there was some trial and error to find
                                    // something that wasn't too ugly
                                    texture_canvas.set_draw_color(Color::RGB(192, 192, 192));
                                    texture_canvas
                                        .draw_point(Point::new(i as i32, j as i32))
                                        .unwrap();
                                }
                                if (i + j * 2) % 5 == 0 {
                                    texture_canvas.set_draw_color(Color::RGB(64, 64, 64));
                                    texture_canvas
                                        .draw_point(Point::new(i as i32, j as i32))
                                        .unwrap();
                                }
                            }
                        }
                    }
                };
                for i in 0..square_size {
                    for j in 0..square_size {
                        // drawing pixel by pixel isn't very effective, but we only do it once and store
                        // the texture afterwards so it's still alright!
                        if (i + j) % 7 == 0 {
                            // this doesn't mean anything, there was some trial and serror to find
                            // something that wasn't too ugly
                            texture_canvas.set_draw_color(Color::RGB(192, 192, 192));
                            texture_canvas
                                .draw_point(Point::new(i as i32, j as i32))
                                .unwrap();
                        }
                        if (i + j * 2) % 5 == 0 {
                            texture_canvas.set_draw_color(Color::RGB(64, 64, 64));
                            texture_canvas
                                .draw_point(Point::new(i as i32, j as i32))
                                .unwrap();
                        }
                    }
                }
            })
            .unwrap();
    }
    (square_texture1, square_texture2)
}
