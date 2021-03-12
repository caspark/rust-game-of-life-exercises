// import some types and macros we want to use
use clap::arg_enum;
use std::path::PathBuf;
use structopt::StructOpt;

// declare the modules that the Rust compiler should look for, which also imports them
mod ui;

// This is a Rust macro - you can tell because the invocation ends with "!". Rust macros basically
// generate code for you, similar to C/C++ macros, except without the many dangers and caveats. In
// this case, it's being used to define an Enum for command line argument parsing using standard
// Rust syntax, however that Enum will also be used to generate some boilerplate for our command
// line argument parsing.
arg_enum! {
    // a "derive" attribute asks the Rust compiler to automatically generate implementations of
    // Traits. You can look these up, but basically Debug is like Python's `repr()` in that it lets
    // you `println!("{:?}", foo)` a `foo` without saying how to `println!("{}", foo)` a `foo`,
    // Clone lets you make a deep copy of an object, and Copy lets you make a bitwise copy of an
    // object.
    #[derive(Debug, Clone, Copy)]
    enum Implementation {
        Broken,
        Solution,
        Mine,
    }
}

// We're using a crate called StructOpt to automatically implement command line argument parsing
// logic for us. Intuitively, we say "the fields on this struct are the options we want people to be
// able to pass to our app", and then StructOpt generates usage information, validates the args
// match the necessary types (e.g. you can't pass a string option for a command line option declared
// as a u8 field), and does the actual parsing of the options into a struct. Under the hood,
// StructOpt itself uses a library called Clap, but you don't really need to worry about that unless
// you want to extend this yourself a bunch.
#[derive(Debug, StructOpt)]
#[structopt(
    name = "game-of-life",
    about = "Renders various implementations of Conway's Game of Life, and allows interacting with them."
)]
struct Opt {
    /// Which Game of Life implementation to run.
    #[structopt(possible_values = &Implementation::variants(), case_insensitive = true)]
    implementation: Implementation,

    /// What pattern file should the renderer load as a starting file? If not specified, a default
    /// pattern generator will be used.
    #[structopt(parse(from_os_str))]
    pattern: Option<PathBuf>,

    /// How many game ticks per second should we aim for?
    #[structopt(long, default_value = "2")]
    fps: u8,

    /// Width of the game board (you should stick to 300 or less probably). The default gives a nice output with the default pattern generator.
    #[structopt(long, short, default_value = "49")]
    width: usize,

    /// Height of the game board (you should stick to 300 or less probably). The default gives a nice output with the default pattern generator.
    #[structopt(long, short, default_value = "40")]
    height: usize,

    /// How big (in pixels) should each cell be? Value must be between 1 and 32 (inclusive).
    #[structopt(long, default_value = "16")]
    cell_size: u8,

    /// Should the simulation start paused?
    #[structopt(long)]
    paused: bool,
}

pub fn main() {
    // invoke StructOpt's parsing routine and get the result
    let config = Opt::from_args();

    // The `println!` macro is how you typically output to stdout (Rust has a logging ecosystem too
    // but we haven't set that up). It works kind of like your friend `printf`. `{}` means print the
    // "to string" of a thing (the `Display` trait), `{:?}` means print the debug representation of
    // a thing (the `Debug` trait), and `{:#?}` means to pretty print the debug representation. You
    // can also use the same syntax to construct a string by using the `format!` macro.
    println!("Running renderer with configuration of: {:#?}", config);

    // overly large cell sizes will make a super large window, so put a stop to those
    if config.cell_size < 1 || config.cell_size > 32 {
        // the panic! macro explodes the app (and prints a stack trace if the RUST_BACKTRACE envvar
        // is set)
        panic!("Invalid cell size provided - must be between 1 and 32, inclusive")
    }

    // pattern match on the chosen implementation name to find the correct implementation, and store
    // that in a Box. Putting something in a Box basically means "move it from the stack to the
    // heap". We need to do this because we have multiple implementations of a trait (which we want
    // to refer to by forgetting which trait it is and treating it as the bare trait, called a
    // "Trait Object" - in other languages, this is like having a pointer to an interface without
    // knowing the concrete type that is satisfying the interface). Anyway, this means that Rust
    // can't know at compile time how much space to reserve in the stack for this amorphous blob
    // of possibilities, so consequently you always have to store Trait Objects in the heap.
    let mut game: Box<dyn conway::GameOfLife> = match config.implementation {
        Implementation::Broken => {
            Box::new(conway::GameOfLiveBroken::new(config.width, config.height))
        }
        Implementation::Solution => {
            Box::new(conway::GameOfLifeSolution::new(config.width, config.height))
        }
        Implementation::Mine => {
            //TODO reference your implementation here for the Part 1 exercise :)
            // the `unimplemented!` macro explodes the app just like the `panic!` macro - the only
            // difference is that it stands out for grepping and in code review as being unfinished
            // code!
            unimplemented!(
                "The {:?} simulation is not yet implemented!",
                Implementation::Mine
            );
        }
    };

    // here we combine pattern matching with an if statement; if the pattern on the left (just after
    // `if let`) matches, then the condition evaluates to true, and any free variables in the
    // pattern are bound to the relevant values on the right hand side.
    if let Some(pattern) = config.pattern {
        // `a.as_mut()` is basically the same as doing `&mut a`, but in this case our `a` is in a
        // Box which we would need to dereference first (by prepending it with `*`), so we use
        // `as_mut()` because it's a bit less "noisy" than writing out `&mut *a`; when you call a
        // function on an object, Rust "auto-dereferences" for your convenience, so using `as_mut()`
        // will handle the dereference for us.
        conway::load_and_apply_pattern(game.as_mut(), &pattern);
    } else {
        conway::apply_default_pattern(game.as_mut());
    }

    ui::run_game(
        game,
        &ui::UiOptions {
            millis_between_ticks: (1000.0 / (config.fps as f32)) as u64,
            square_size: config.cell_size,
            start_paused: config.paused,
        },
    )
}
