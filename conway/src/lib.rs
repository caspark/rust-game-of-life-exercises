// declare what modules this library provides
//TODO you'll need to add a module for your implementation here
mod game_of_life;
mod game_of_life_broken;
mod game_of_life_solution;

// export bits of relevant modules to avoid consuming crates depending on internal implementation
// details
//TODO you'll need to export your implementation here
pub use game_of_life::GameOfLife;
pub use game_of_life_broken::BrokenGame;
pub use game_of_life_solution::GameOfLifeSolution;

#[allow(unused_variables)] //TODO remove this lint once you're implementing this function
pub fn load_and_apply_pattern(game: &mut dyn game_of_life::GameOfLife, pattern_filename: &str) {
    //TODO fill this out for the step 2 exercise :)
    unimplemented!("Pattern loading from file is not implemented yet!");
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
