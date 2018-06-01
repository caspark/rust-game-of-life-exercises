// declare dependencies (somewhat duplicated by Cargo's Crate.toml because the Rust compiler can be used without Cargo)
extern crate sdl2;

// declare the modules that the Rust compiler should look for
mod bench;
mod game_of_life;
mod game_of_life_solution;
mod ui;

// import types and modules we want to use; note that `extern crate` and `mod` statements pre-import those crates/modules for this file too.
use game_of_life::GameOfLife;
use std::env;

// define a few constants for our homegrown arg parsing
const MODE_RENDER: &str = "render";
const MODE_BENCH: &str = "bench";

const SIM_SOLUTION: &str = "solution";
const SIM_BROKEN: &str = "broken";
const SIM_MINE: &str = "mine";

const MODES: [&str; 2] = [MODE_BENCH, MODE_RENDER];
const SIMS: [&str; 3] = [SIM_SOLUTION, SIM_BROKEN, SIM_MINE];

/// When running a benchmark, how many Game of Life iterations should we run though?
const BENCHMARK_STEPS: u32 = 1_000_000;

pub fn main() {
    // Rust has a few nice arg parsing libraries (e.g. clap and docopt-rs) but this is simpler and
    // also shows off some Rust syntax around string formatting, so we'll roll our own for now.
    let args: Vec<String> = env::args().collect();
    let (prog_name, actual_args) = args.split_first().expect("will always have at least 1 arg");

    let available_modes = MODES.join(", ");
    let available_sims = SIMS.join(", ");
    let usage = format!(
        "Usage: {prog_name} MODE SIM [PATTERN_FILENAME]
    \tMODE - One of the available modes: {modes}
    \tSIM - One of the available simulation implementations: {sims}
    \tPATTERN_FILENAME- Filename to load as a starting pattern",
        prog_name = prog_name,
        modes = available_modes,
        sims = available_sims
    );

    if ![2, 3].contains(&actual_args.len()) {
        panic!(
            "Invalid number of args - expected 2-3, got {}\n{}",
            actual_args.len(),
            usage
        );
    }

    let mode = &actual_args[0];
    let sim = &actual_args[1];
    let maybe_pattern_filename = actual_args.get(2);

    let mut game: Box<GameOfLife> = match sim.as_str() {
        SIM_SOLUTION => Box::new(game_of_life_solution::GameOfLifeSolution::new()),
        SIM_BROKEN => Box::new(game_of_life::BrokenGame::new()),
        SIM_MINE => {
            //FIXME reference your implementation here for the step 1 exercise :)
            unimplemented!("The {} simulation is not yet implemented!", SIM_MINE);
        }
        other => panic!(
            "Bad sim {}, expected one of {}\n{}",
            other, available_sims, usage
        ),
    };

    if let Some(pattern_filename) = maybe_pattern_filename {
        load_and_apply_pattern(game.as_mut(), pattern_filename);
    } else {
        apply_default_pattern(game.as_mut());
    }

    match mode.as_str() {
        MODE_RENDER => {
            ui::run_game(game, &ui::UiOptions {
                // you can change these parameters if you like - UiOptions docs explain each param
                millis_between_ticks: 500,
                square_size: 16,
            })
        }
        MODE_BENCH => bench::run_bench(game, BENCHMARK_STEPS),
        other => panic!(
            "Bad mode {}, expected one of {}\n{}",
            other, available_sims, usage
        ),
    };
}

#[allow(unused_variables)]
fn load_and_apply_pattern(game: &mut GameOfLife, pattern_filename: &str) {
    //FIXME fill this out for the step 2 exercise :)
    unimplemented!("Pattern loading from file is not implemented yet!");
}

/// Loads a nice default pattern into the given game
fn apply_default_pattern(game: &mut GameOfLife) {
    use game_of_life::{PLAYGROUND_HEIGHT, PLAYGROUND_WIDTH};

    for x in 1..PLAYGROUND_WIDTH - 1 {
        game.toggle_cell(x as i32, 1);
        game.toggle_cell(x as i32, PLAYGROUND_HEIGHT as i32 - 2);
    }

    for y in 1..PLAYGROUND_HEIGHT - 1 {
        game.toggle_cell(1, y as i32);
        game.toggle_cell(PLAYGROUND_WIDTH as i32 - 2, y as i32);
    }
}
