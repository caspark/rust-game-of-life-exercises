// This is the "interface" that you should implement; the UI uses this to drive the behavior of the
// game of life.
pub trait GameOfLife {
    /// Return `Some(true)` if the cell is alive, `Some(false)` if it is dead, or `None` if `x`
    /// and/or `y` are out of bounds.
    fn is_cell_alive(&self, x: usize, y: usize) -> Option<bool>;

    /// Swap the given cell from alive to dead or dead to alive.
    ///
    /// If `x` or `y` is out of bounds, this method should do nothing.
    ///
    /// The origin is assumed to be at the top left, i.e. when `(x, y) == (0, 0)` then the
    /// top-left-most cell should be toggled.
    fn toggle_cell(&mut self, x: usize, y: usize);

    /// Execute one timestep; i.e. cause cells to live, be born, or die based on the amount of
    /// neighbors they have.
    fn tick(&mut self);

    /// Return the current width in cells of the game.
    fn width(&self) -> usize;

    /// Return the current height in cells of the game.
    fn height(&self) -> usize;
}

/// A blatantly-wrong implementation of GameOfLife, to show the syntax for implementing traits.
///
/// In case it's not obvious, the blatantly wrong part of this implementation is that it always
/// creates a gameboard with just a single cell (and always just toggles that), rather than
/// creating a game board of the requested size.
///
/// You can start off your own implementation by copy-pasting this.
pub struct BrokenGame {
    cell_state: bool,
}

impl BrokenGame {
    // note `new` is just a regular function - there's no such thing as a "constructor"
    pub fn new(game_width: usize, game_height: usize) -> BrokenGame {
        assert!(game_width > 0, "game width must be greater than 0");
        assert!(game_height > 0, "game height must be greater than 0");
        BrokenGame { cell_state: true }
    }
}

impl GameOfLife for BrokenGame {
    fn is_cell_alive(&self, _x: usize, _y: usize) -> Option<bool> {
        // Broken: this doesn't respect the x & y params at all.
        Some(self.cell_state)
    }

    // NB: underscores stop compiler complaining about unused variables - if you use them, you
    // should rename them to remove the underscores.
    fn toggle_cell(&mut self, _x: usize, _y: usize) {
        // Broken: toggle the only cell we have, instead of the one refenced by _x and _y
        self.cell_state = !self.cell_state;
    }

    fn tick(&mut self) {
        // Broken: each game tick, we'll just toggle some arbitrary cell's state from what it
        // previously was, instead of implementing the rules of Conway's Game of Life.
        self.toggle_cell(42, 42);

        println!(
            "Broken game tick completed - cell_state is now {}",
            self.cell_state
        );
    }

    fn width(&self) -> usize {
        49 // Broken: this implementation always returns the same width
    }

    fn height(&self) -> usize {
        40 // Broken: this implementation always returns the same height
    }
}

#[cfg(test)] // this attr means the module below is only included when doing `cargo test`
mod broken_game_test {
    use super::{BrokenGame, GameOfLife};

    /// A basic test to show you how to write tests in Rust, in case you want to write your own.
    #[test]
    fn broken_game_is_definitely_broken() {
        let mut game = BrokenGame::new(10, 10);
        let cell_0_0_orig_val = game.is_cell_alive(0, 0);

        // change a totally different cell from 0,0
        game.toggle_cell(1, 1);

        let cell_0_0_new_val = game.is_cell_alive(0, 0);
        // now we expect cell 0,0's liveness to have changed because we know that BrokenGame
        // is a totally broken implementation. If the two values are equal, then something seriously
        // weird is going on.
        // Tip: `assert_ne!` means "assert not equal" - normally using `assert!` or `assert_eq!` is typical.
        assert_ne!(
            cell_0_0_orig_val, cell_0_0_new_val,
            "Uh oh, cell 0,0 failed to change from its \
             original value even though we tried to mutate another cell, so BrokenGame may not be \
             broken anymore!?"
        );
    }
}
