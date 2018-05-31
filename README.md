Game of Life Exercise
=====================

An intro to Rust using Conway's Game of Life (heavily based on the SDL2 Game of Life example).

## Setup

### Ubuntu

```
sudo apt install libsdl2-dev
```

### MacOS

Install SDL2 via HomeBrew:

    brew install sdl2

Then set up the header locations by adding `/usr/local/lib` to your path:

    echo 'export LIBRARY_PATH="$LIBRARY_PATH:/usr/local/lib"' >> ~/.bash_profile

## Running

Run the simulation with

    cargo run -- render solution

then hit <kbd>Space</kbd> to pause/unpause.

You can also toggle the state of cells using the mouse's left click.

Hit <kbd>Escape</kbd> or <kbd>Q</kbd> to quit.

## Exercises

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

Look at `main.rs` and `game_of_life.rs`, to see how to implement and wire up your own implementation of the `GameOfLife` trait.

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

And a larger sample:

```
#N Gosper glider gun
#C This was the first gun discovered.
#C As its name suggests, it was discovered by Bill Gosper.
x = 36, y = 9, rule = B3/S23
24bo$22bobo$12b2o6b2o12b2o$11bo3bo4b2o12b2o$2o8bo5bo3b2o$2o8bo3bob2o4b
obo$10bo5bo7bo$11bo3bo$12b2o!
```

More info at http://www.conwaylife.com/w/index.php?title=Run_Length_Encoded

You can also find far more examples at http://www.conwaylife.com/wiki/Category:Patterns

### Step 3 - Performance tuning

Goal: learn about performance tuning by competing to see who can build the fastest implementation of the game of life update step.

You can benchmark the official solution through a million game steps with:

    cargo build --release && time game-of-life bench solution

or bench your solution for the same with

    time cargo run --release -- bench mine

Try to get your real time elapsed for a run as low as possible.

# TODO

* Add cargo format and clippy + fix all resulting warnings
* Put some sample RLE patterns into the repo, including the clock pattern
* Cap sim update rate based on time elapsed rather than based on frame rate
* Take snapshots of step 0, 1, 2 and save them as git branches
