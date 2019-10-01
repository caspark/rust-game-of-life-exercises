#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::{BatchSize, BenchmarkId};

//TODO import your implementation here
use conway::{BrokenGame, GameOfLife, GameOfLifeSolution};

const NUM_TICKS: u32 = 50;

fn bench_tick(c: &mut Criterion) {
    let mut group = c.benchmark_group("Tick");

    for &i in [8, 64].iter() {
        {
            let broken = BrokenGame::new(i, i);
            group.bench_function(BenchmarkId::new("Broken", i), move |b| {
                b.iter_batched(
                    || broken.clone(),
                    |mut game| {
                        for _ in 0..NUM_TICKS {
                            game.tick()
                        }
                    },
                    BatchSize::SmallInput,
                )
            });
        }

        {
            let solution = GameOfLifeSolution::new(i, i);
            group.bench_function(BenchmarkId::new("Solution", i), move |b| {
                b.iter_batched(
                    || solution.clone(),
                    |mut game| {
                        for _ in 0..NUM_TICKS {
                            game.tick()
                        }
                    },
                    BatchSize::SmallInput,
                )
            });
        }

        //TODO copy paste and modify one of the examples above to benchmark your implementation
    }
    group.finish();
}

criterion_group!(benches, bench_tick);
criterion_main!(benches);
