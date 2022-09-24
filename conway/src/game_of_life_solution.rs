//! This a solution to Part 1 - you shouldn't look at this!
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
//!
//! (In case you're wondering, `//!` is how you do module level documentation in Rust)

use crate::game_of_life::GameOfLife;

#[derive(Clone, Debug)]
pub struct GameOfLifeSolution {
    width: usize,
    playground: Vec<bool>,
}

impl GameOfLifeSolution {
    pub fn new(width: usize, height: usize) -> GameOfLifeSolution {
        println!("width is {} and height is {}", width, height);
        let mut playground = Vec::new();
        playground.extend(::std::iter::repeat(false).take(width * height));

        GameOfLifeSolution { width, playground }
    }

    fn get_cell_mut(&mut self, x: usize, y: usize) -> Option<&mut bool> {
        if x < self.width() && y < self.height() {
            Some(&mut self.playground[x + y * self.width])
        } else {
            None
        }
    }
}

impl GameOfLife for GameOfLifeSolution {
    fn is_cell_alive(&self, x: usize, y: usize) -> Option<bool> {
        if x < self.width() && y < self.height() {
            Some(self.playground[(x + y * self.width) as usize])
        } else {
            None
        }
    }

    fn toggle_cell(&mut self, x: usize, y: usize) {
        if let Some(square) = self.get_cell_mut(x, y) {
            *square = !(*square);
        } else {
            eprintln!("Avoiding toggling cell at {}, {} - is out of bounds!", x, y);
        }
    }

    fn tick(&mut self) {
        let mut new_playground = self.playground.clone();
        for (u, square) in new_playground.iter_mut().enumerate() {
            let x = u % self.width();
            let y = u / self.width();
            let mut count: u32 = 0;
            // We're working with unsigned numbers, so instead of the top left offset
            // being -1,-1, it's 0,0. Also, to avoid underflowing, leave out the top
            // (/left) row (/column) of cells if we're dealing with the topmost row
            // (/leftmost column) on the board.
            let min_x_offset = if x > 0 { 0 } else { 1 };
            let min_y_offset = if y > 0 { 0 } else { 1 };
            for i in min_x_offset..=2 {
                for j in min_y_offset..=2 {
                    if !(i == 1 && j == 1) {
                        // subtract 1 to correct for 0,0 offset being top left
                        // neighbor
                        let peek_x = x + i - 1;
                        let peek_y = y + j - 1;
                        if let Some(true) = self.is_cell_alive(peek_x, peek_y) {
                            count += 1;
                        }
                    }
                }
            }

            // prevent clippy from complaining about not using a range check on the next if condition
            #[allow(clippy::manual_range_contains)]
            if count < 2 || count > 3 {
                // Any live cell with fewer than two live neighbors dies, as if by under population.
                // &
                // Any live cell with more than three live neighbors dies, as if by overpopulation.
                *square = false;
            } else if *square && (count == 2 || count == 3) {
                // Any live cell with two or three live neighbors lives on to the next generation.
                // (*square is already true so nothing to do here)
            } else if !*square && count == 3 {
                // Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
                *square = true;
            }
        }
        self.playground = new_playground;
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.playground.len() / self.width
    }
}
