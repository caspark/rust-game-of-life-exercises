extern crate sdl2;

mod bench;
mod game_of_life;
mod game_of_life_solution;
mod ui;

use game_of_life::GameOfLife;
use std::env;

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

    let game: Box<GameOfLife> = match sim.as_str() {
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
        load_and_apply_pattern(&game, pattern_filename)
    }

    match mode.as_str() {
        MODE_RENDER => ui::run_game(game),
        MODE_BENCH => bench::run_bench(game, BENCHMARK_STEPS),
        other => panic!(
            "Bad mode {}, expected one of {}\n{}",
            other, available_sims, usage
        ),
    };
}

#[allow(unused_variables)]
fn load_and_apply_pattern(game: &Box<GameOfLife>, pattern_filename: &str) {
    //FIXME fill this out for the step 2 exercise :)
    unimplemented!("Pattern loading from file is not implemented yet!");
}
