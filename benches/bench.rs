use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use timer_queue::TimerQueue;

fn bench(c: &mut Criterion) {
    let mut rng = SmallRng::from_seed([0xAB; 32]);
    {
        let mut group = c.benchmark_group("insert");
        for timers in [10, 1_000, 1_000_000] {
            for period in [10, 1_000, 1_000_000] {
                group.throughput(criterion::Throughput::Elements(timers));
                group.bench_function(format!("{} timers over {} ticks", timers, period), |b| {
                    b.iter_batched(
                        || TimerQueue::with_capacity(timers as usize),
                        |mut q| {
                            for _ in 0..timers {
                                q.insert(rng.gen_range(0..period), ());
                            }
                        },
                        BatchSize::SmallInput,
                    )
                });
            }
        }
    }
    {
        let mut group = c.benchmark_group("poll");
        for timers in [0, 10, 1_000, 1_000_000] {
            for period in [10, 1_000, 1_000_000] {
                group.throughput(criterion::Throughput::Elements(timers));
                group.bench_function(format!("{} timers over {} ticks", timers, period), |b| {
                    b.iter_batched(
                        || {
                            let mut q = TimerQueue::with_capacity(timers as usize);
                            for _ in 0..timers {
                                q.insert(rng.gen_range(0..period), ());
                            }
                            q
                        },
                        |mut q| {
                            while let Some(_) = q.poll(period) {}
                        },
                        BatchSize::SmallInput,
                    )
                });
            }
        }
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
