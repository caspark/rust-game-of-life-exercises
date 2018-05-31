//! This a solution to Step 1 - you shouldn't look at this!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//! No really, even if you're stuck, you should ask someone for help first :)
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//! I mean it, no need to look at this!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//! Well, if you insist. Don't say I didn't warn you.
//!
//! # The solution
//!
//! Here's a sample solution to implementing the `GameOfLife` trait.
//!
//! It's based on some example code for SDL with some modifications to make it easier to understand;
//! it's not optimized for speed.
//!

use game_of_life::{GameOfLife, PLAYGROUND_HEIGHT, PLAYGROUND_WIDTH};

pub struct GameOfLifeSolution {
    playground: [bool; (PLAYGROUND_WIDTH * PLAYGROUND_HEIGHT) as usize],
}

impl GameOfLifeSolution {
    pub fn new() -> GameOfLifeSolution {
        let mut playground = [false; (PLAYGROUND_WIDTH * PLAYGROUND_HEIGHT) as usize];

        // let's make a nice default pattern!
        for i in 1..(PLAYGROUND_HEIGHT - 1) {
            playground[(1 + i * PLAYGROUND_WIDTH) as usize] = true;
            playground[((PLAYGROUND_WIDTH - 2) + i * PLAYGROUND_WIDTH) as usize] = true;
        }
        for j in 2..(PLAYGROUND_WIDTH - 2) {
            playground[(PLAYGROUND_WIDTH + j) as usize] = true;
            playground[((PLAYGROUND_HEIGHT - 2) * PLAYGROUND_WIDTH + j) as usize] = true;
        }

        GameOfLifeSolution { playground }
    }

    fn get_cell_mut(&mut self, x: i32, y: i32) -> Option<&mut bool> {
        if x >= 0 && y >= 0 && (x as u32) < PLAYGROUND_WIDTH && (y as u32) < PLAYGROUND_HEIGHT {
            Some(&mut self.playground[(x as u32 + (y as u32) * PLAYGROUND_WIDTH) as usize])
        } else {
            None
        }
    }
}

impl GameOfLife for GameOfLifeSolution {
    fn is_cell_alive(&self, x: i32, y: i32) -> Option<bool> {
        if x >= 0 && y >= 0 && (x as u32) < PLAYGROUND_WIDTH && (y as u32) < PLAYGROUND_HEIGHT {
            Some(self.playground[(x as u32 + (y as u32) * PLAYGROUND_WIDTH) as usize])
        } else {
            None
        }
    }

    fn toggle_cell(&mut self, x: i32, y: i32) {
        if let Some(square) = self.get_cell_mut(x as i32, y as i32) {
            println!("Toggling cell at {}, {}", x, y);
            *square = !(*square);
        } else {
            eprintln!("Avoiding toggling cell at {}, {} - is out of bounds!", x, y);
        }
    }

    fn tick(&mut self) {
        let mut new_playground = self.playground;
        for (u, square) in new_playground.iter_mut().enumerate() {
            let u = u as u32;
            let x = u % PLAYGROUND_WIDTH;
            let y = u / PLAYGROUND_WIDTH;
            let mut count: u32 = 0;
            for i in -1..2 {
                for j in -1..2 {
                    if !(i == 0 && j == 0) {
                        let peek_x: i32 = (x as i32) + i;
                        let peek_y: i32 = (y as i32) + j;
                        if let Some(true) = self.is_cell_alive(peek_x, peek_y) {
                            count += 1;
                        }
                    }
                }
            }
            if count > 3 || count < 2 {
                *square = false;
            } else if count == 3 {
                *square = true;
            } else if count == 2 {
                *square = *square;
            }
        }
        self.playground = new_playground;
    }
}
