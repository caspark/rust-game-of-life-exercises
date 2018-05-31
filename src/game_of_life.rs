pub const SQUARE_SIZE: u32 = 16;
pub const PLAYGROUND_WIDTH: u32 = 49;
pub const PLAYGROUND_HEIGHT: u32 = 40;


pub struct GameOfLife {
    playground: [bool; (PLAYGROUND_WIDTH*PLAYGROUND_HEIGHT) as usize],
}

impl GameOfLife {
    pub fn new() -> GameOfLife {
        let mut playground = [false; (PLAYGROUND_WIDTH * PLAYGROUND_HEIGHT) as usize];

        // let's make a nice default pattern !
        for i in 1..(PLAYGROUND_HEIGHT-1) {
            playground[(1 + i* PLAYGROUND_WIDTH) as usize] = true;
            playground[((PLAYGROUND_WIDTH-2) + i* PLAYGROUND_WIDTH) as usize] = true;
        }
        for j in 2..(PLAYGROUND_WIDTH-2) {
            playground[(PLAYGROUND_WIDTH + j) as usize] = true;
            playground[((PLAYGROUND_HEIGHT-2)*PLAYGROUND_WIDTH + j) as usize] = true;
        }

        GameOfLife {
            playground,
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<bool> {
        if x >= 0 && y >= 0 &&
            (x as u32) < PLAYGROUND_WIDTH && (y as u32) < PLAYGROUND_HEIGHT {
            Some(self.playground[(x as u32 + (y as u32)* PLAYGROUND_WIDTH) as usize])
        } else {
            None
        }
    }

    fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut bool> {
        if x >= 0 && y >= 0 &&
            (x as u32) < PLAYGROUND_WIDTH && (y as u32) < PLAYGROUND_HEIGHT {
            Some(&mut self.playground[(x as u32 + (y as u32)* PLAYGROUND_WIDTH) as usize])
        } else {
            None
        }
    }

    pub fn toggle_cell(&mut self, x: i32, y: i32) {
        if let Some(square) = self.get_mut(x as i32, y as i32) {
            *square = !(*square);
        };
    }

    pub fn update(&mut self) {
        let mut new_playground = self.playground;
        for (u, square) in new_playground.iter_mut().enumerate() {
            let u = u as u32;
            let x = u % PLAYGROUND_WIDTH;
            let y = u / PLAYGROUND_WIDTH;
            let mut count : u32 = 0;
            for i in -1..2 {
                for j in -1..2 {
                    if !(i == 0 && j == 0) {
                        let peek_x : i32 = (x as i32) + i;
                        let peek_y : i32 = (y as i32) + j;
                        if let Some(true) = self.get(peek_x, peek_y) {
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



impl<'a> IntoIterator for &'a GameOfLife {
    type Item = &'a bool;
    type IntoIter = ::std::slice::Iter<'a, bool>;
    fn into_iter(self) -> ::std::slice::Iter<'a, bool> {
        self.playground.iter()
    }
}