// declare what modules this library provides
mod game_of_life;
mod game_of_life_broken;
mod game_of_life_solution;

// re-export bits of relevant modules to avoid consuming crates depending on
// internal implementation details
pub use game_of_life::GameOfLife;
pub use game_of_life_broken::BrokenGame;
pub use game_of_life_solution::GameOfLifeSolution;
