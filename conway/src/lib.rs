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
