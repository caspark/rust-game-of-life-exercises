use std::path::Path;

// declare what modules this library provides
mod game_of_life;
mod game_of_life_broken;
mod game_of_life_mine;
mod game_of_life_solution;

// export bits of relevant modules to avoid consuming crates depending on internal implementation
// details
pub use game_of_life::GameOfLife;
pub use game_of_life_broken::GameOfLiveBroken;
pub use game_of_life_mine::GameOfLifeMine;
pub use game_of_life_solution::GameOfLifeSolution;

#[allow(unused_variables)] //TODO remove this lint once you're implementing this function for part 2
pub fn load_and_apply_pattern(game: &mut dyn game_of_life::GameOfLife, pattern_filename: &Path) {
    //TODO fill this out for the part 2 exercise :)
    unimplemented!(
        "Need to implement loading pattern from file {:?}",
        &pattern_filename
    );
}

/// Loads a nice default pattern into the given game
pub fn apply_default_pattern(game: &mut dyn game_of_life::GameOfLife) {
    for x in 1..game.width() - 1 {
        game.toggle_cell(x, 1);
        game.toggle_cell(x, game.height() - 2);
    }

    for y in 1..game.height() - 1 {
        game.toggle_cell(1, y);
        game.toggle_cell(game.width() - 2, y);
    }
}
