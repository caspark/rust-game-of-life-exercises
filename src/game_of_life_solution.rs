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

use crate::game_of_life::GameOfLife;

pub struct GameOfLifeSolution {
    width: i32,
    playground: Vec<bool>,
}

impl GameOfLifeSolution {
    pub fn new(width: i32, height: i32) -> GameOfLifeSolution {
        println!("width is {} and height is {}", width, height);
        let mut playground = Vec::new();
        playground.extend(::std::iter::repeat(false).take((width * height) as usize));

        GameOfLifeSolution { width, playground }
    }

    fn get_cell_mut(&mut self, x: i32, y: i32) -> Option<&mut bool> {
        if x >= 0 && y >= 0 && x < self.width() && y < self.height() {
            Some(&mut self.playground[(x + y * self.width) as usize])
        } else {
            None
        }
    }
}

impl GameOfLife for GameOfLifeSolution {
    fn is_cell_alive(&self, x: i32, y: i32) -> Option<bool> {
        if x >= 0 && y >= 0 && x < self.width() && y < self.height() {
            Some(self.playground[(x + y * self.width) as usize])
        } else {
            None
        }
    }

    fn toggle_cell(&mut self, x: i32, y: i32) {
        if let Some(square) = self.get_cell_mut(x as i32, y as i32) {
            *square = !(*square);
        } else {
            eprintln!("Avoiding toggling cell at {}, {} - is out of bounds!", x, y);
        }
    }

    fn tick(&mut self) {
        let mut new_playground = self.playground.clone();
        for (u, square) in new_playground.iter_mut().enumerate() {
            let u = u as i32;
            let x = u % self.width();
            let y = u / self.width();
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

    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.playground.len() as i32 / self.width
    }
}
