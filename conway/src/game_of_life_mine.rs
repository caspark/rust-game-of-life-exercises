use crate::game_of_life::GameOfLife; // TODO you should read the docs for this - it's the interface you will implement

/// Your implementation of GameOfLife, which you should implement.
///
/// It's scattered with various TODO markers that you should implement or deal with, such as this one:
///
/// TODO take a look at conway/src/game_of_life_broken.rs
/// (it shows a possible implementation of the GameOfLife trait - albeit a broken one - and may help you with syntax problems)
#[derive(Clone, Debug)]
pub struct GameOfLifeMine {
    //TODO decide what state GameOfLifeMine has (hint: google for "rust vec" if you want an array)
}

impl GameOfLifeMine {
    // TODO call this function over in src/main.rs in the parent crate
    pub fn new(game_width: usize, game_height: usize) -> GameOfLifeMine {
        assert!(game_width > 0, "game width must be greater than 0");
        assert!(game_height > 0, "game height must be greater than 0");
        GameOfLifeMine {
            // TODO you'll probably want to set initial state for the GameOfLifeMine struct here
        }
    }
}

impl GameOfLife for GameOfLifeMine {
    fn is_cell_alive(&self, _x: usize, _y: usize) -> Option<bool> {
        unimplemented!("FIXME is_cell_alive needs to be implemented still")
    }

    // NB: underscores stop compiler complaining about unused variables - if you use them, you
    // should rename them to remove the underscores.
    fn toggle_cell(&mut self, _x: usize, _y: usize) {
        unimplemented!("FIXME toggle_cell needs to be implemented still")
    }

    fn tick(&mut self) {
        unimplemented!("FIXME tick needs to be implemented still")
    }

    fn width(&self) -> usize {
        unimplemented!("FIXME width needs to be implemented still")
    }

    fn height(&self) -> usize {
        unimplemented!("FIXME height needs to be implemented still")
    }
}

#[cfg(test)] // this attr means the module below is only included when doing `cargo test`
#[allow(unused_imports)] // stop rust compiler complaining about imports below
mod broken_game_test {
    use super::{GameOfLife, *};

    /// Dead simple test for you to add to if you want.
    #[test]
    fn some_var_is_always_true() {
        let some_var = true;
        assert_eq!(
            some_var, true,
            "some_var must always be true, otherwise the universe is not in order"
        );
    }
}
