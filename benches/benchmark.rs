#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::{BatchSize, BenchmarkId};

//TODO import your implementation here
use conway::{apply_default_pattern, GameOfLife, GameOfLifeSolution, GameOfLiveBroken};

const NUM_TICKS: u32 = 50;

fn bench_tick(c: &mut Criterion) {
    let mut group = c.benchmark_group("Tick");

    for &(w, h) in [(8, 8), (64, 64)].iter() {
        {
            let mut game = GameOfLiveBroken::new(w, h);
            apply_default_pattern(&mut game);
            group.bench_function(
                BenchmarkId::new("Broken", format!("{}x{}", w, h)),
                move |b| {
                    b.iter_batched(
                        || game.clone(),
                        |mut g| {
                            for _ in 0..NUM_TICKS {
                                g.tick()
                            }
                        },
                        BatchSize::SmallInput,
                    )
                },
            );
        }

        {
            let mut game = GameOfLifeSolution::new(w, h);
            apply_default_pattern(&mut game);
            group.bench_function(
                BenchmarkId::new("Solution", format!("{}x{}", w, h)),
                move |b| {
                    b.iter_batched(
                        || game.clone(),
                        |mut g| {
                            for _ in 0..NUM_TICKS {
                                g.tick()
                            }
                        },
                        BatchSize::SmallInput,
                    )
                },
            );
        }

        //TODO copy paste and modify one of the examples above to benchmark your implementation
    }
    group.finish();
}

criterion_group!(benches, bench_tick);
criterion_main!(benches);
