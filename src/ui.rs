//! This module contains logic for rendering a Game of Life simulation using SDL2.
//!
//! You probably don't need to worry about it, unless you want to extend the UI with new behavior.

use game_of_life::{GameOfLife, PLAYGROUND_HEIGHT, PLAYGROUND_WIDTH};
use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

const SQUARE_SIZE: u32 = 16;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SimulationState {
    Paused,
    Playing,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Simulation {
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

pub fn run_game(mut game: Box<GameOfLife>) {
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
            SQUARE_SIZE * PLAYGROUND_WIDTH,
            SQUARE_SIZE * PLAYGROUND_HEIGHT,
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

    let (playing_texture, paused_texture) = generate_textures(&mut canvas, &texture_creator);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut frame: u32 = 0;
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
                    let cell_x = (x as u32) / SQUARE_SIZE;
                    let cell_y = (y as u32) / SQUARE_SIZE;
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
        if frame >= 100 {
            game.tick();
            frame = 0;
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
        for x in 0..(PLAYGROUND_WIDTH) {
            for y in 0..(PLAYGROUND_HEIGHT) {
                match game.is_cell_alive(x as i32, y as i32) {
                    Some(true) => canvas
                        .copy(
                            square_texture,
                            None,
                            Rect::new(
                                (x * SQUARE_SIZE) as i32,
                                (y * SQUARE_SIZE) as i32,
                                SQUARE_SIZE,
                                SQUARE_SIZE,
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
        if sim.state == SimulationState::Playing {
            frame += 1;
        };
    }
}

fn generate_textures<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> (Texture<'a>, Texture<'a>) {
    enum TextureColor {
        Yellow,
        White,
    };
    let mut square_texture1: Texture = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .unwrap();
    let mut square_texture2: Texture = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
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
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
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
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
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
                for i in 0..SQUARE_SIZE {
                    for j in 0..SQUARE_SIZE {
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
