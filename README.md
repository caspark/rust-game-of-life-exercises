Game of Life Exercise
=====================

An intro to Rust using Conway's Game of Life (heavily based on the SDL2 Game of Life example).

Install Rust
------------

    # install Rust by following the prompts
    curl https://sh.rustup.rs -sSf | sh

    # install the Rust Language Server (for IDE assistance), Rust code formatter, and the stdlib source 
    rustup component add rls-preview rustfmt-preview rust-src

If you want IDE support:

* IntelliJ IDEA (community edition) w/ the JetBrains Rust plugin is the best experience as of 2018 June.
* VSCode's Rust mode is a close second: https://github.com/editor-rs/vscode-rust

Repo Setup
----------

### Ubuntu

Get some SDL onto your box:

    sudo apt install libsdl2-dev

### MacOS

Install SDL2 via HomeBrew:

    brew install sdl2

Then set up the header locations by adding `/usr/local/lib` to your path (you can skip this if it's already in your path):

    echo 'export LIBRARY_PATH="$LIBRARY_PATH:/usr/local/lib"' >> ~/.bash_profile

Running
-------

Run the simulation with

    cargo run -- render solution

then hit <kbd>Space</kbd> to pause/unpause.

You can also toggle the state of cells using the mouse's left click.

Hit <kbd>Escape</kbd> or <kbd>Q</kbd> to quit.

Development tips
----------------

Get faster type errors with `cargo check` (it skips actually building the resulting binary).

Run tests with `cargo test`.

Format your code with `cargo fmt`.

You can also install the cargo-watch tool if you like continuous feedback in your terminal:

    cargo install cargo-watch
    cargo watch                         # continuously runs `cargo check`
    cargo watch --exec test --exec fmt  # continuously runs `cargo test && cargo fmt`

Exercises
---------

### Step 1 - Get the simulation working

Goal: get started with Rust by implementing the game logic rules for Conway's Game of Life.

When you run

    cargo run -- render mine

then each game tick should cause the following behavior:

* Any live cell with fewer than two live neighbors dies, as if by under population.
* Any live cell with two or three live neighbors lives on to the next generation.
* Any live cell with more than three live neighbors dies, as if by overpopulation.
* Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.

To get you started, there's a broken implementation of Game of Life which you can run with:

    cargo run -- render broken

Tips:

* Look at `main.rs` and `game_of_life.rs`, to see how to implement and wire up your own implementation of the `GameOfLife` trait.
* You can store an array of data using the [Vec data type](https://doc.rust-lang.org/std/vec/struct.Vec.html).

Helpful doc links:

* Lots of examples at [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html) .
* There are full tutorials at [the Rust Book](https://doc.rust-lang.org/book/second-edition/index.html).
* If you get really stuck, try [the Rust Reference](https://doc.rust-lang.org/reference/index.html).

### Step 2 - Implement loading patterns from files

Goal: get more comfortable with Rust by implementing a basic file format parser.

It's common to share Game of Life patterns via run length encoded text strings, so let's implement a parser which loads these files into the simulation.

* Lines starting with `#` are comments and so can be ignored.
* The first line of content is a header of the form `x = m, y = n` to indicate the dimensions of the pattern (this is sometimes suffixed by `, rule = abc` to support other cellular automata but you can ignore that).
* Subsequent content lines are formatted as repetitions of `ab` (possibly separated by whitespace) where `a` is the number of times that `b` repeats and `b` is an indicator of either alive cells (`o`), dead cells (`b`), or start-new-line (`$`).
* The content always ends with `!` as an end-of-file separator.

For example content of

```
bo$2b
o$3o!
```

gives (with `O` indicating live cells and whitespace indicating dead cells):

```
 O
  O
OOO
```

Here's a sample of a full file format:

```
#C This is a glider.
x = 3, y = 3
bo$2bo$3o!
```

This pattern file is at `patterns/gosper-glider-gun.txt`, and there are more patterns in the same directory.

More info on the file format is at http://www.conwaylife.com/w/index.php?title=Run_Length_Encoded

You can also find far more patterns at http://www.conwaylife.com/wiki/Category:Patterns

Tip: You will probably get into error handling with `Some` (Rust's version of returning `null`) and `Result` (Rust's alternative to exceptions) types. The quick and dirty way to get at the contents of each is to call `unwrap()` or `expect()` (which will panic your app if there is no content); read [the book's error handling section](https://doc.rust-lang.org/book/second-edition/ch09-00-error-handling.html) for more info.

### Step 3 - Performance tuning

Goal: learn about performance tuning by competing to see who can build the fastest implementation of the game of life update step.

Rust by default builds & runs unoptimized debug code - pass the `--release` flag to Cargo for a 10-100x speedup.

You can benchmark the official solution through a million game steps with:

    cargo build --release && time game-of-life bench solution

or bench your solution for the same with

    cargo build --release && time game-of-life bench mine

Try to get your real time elapsed for a run as low as possible.

Tips:

* On Linux, [`perf` and Valgrind's `cachegrind` are good profiling tools](https://doc.rust-lang.org/book/second-edition/ch16-01-threads.html); you're on your own for MacOS.
* Parallelism is easiest to do via [Rust's](https://doc.rust-lang.org/book/second-edition/ch16-01-threads.html) support.
* If you need to communicate between threads, use channels (`mpsc::channel` or the `chan` crate) or `std::sync`'s primitives (like `Mutex`).
* A vector (`Vec`) of booleans is fairly efficient but a bit vector might be more efficient.
* You might want to look into Rust's SIMD support if you have experience with that.
