use std::time::Duration;

#[derive(Debug)]
pub struct BenchOptions {
    pub times: usize,
}

pub fn bench<R>(fun: impl Fn() -> R) -> Duration {
    bench_with_options(BenchOptions { times: 10000 }, fun)
}

pub fn bench_with_options<R>(options: BenchOptions, fun: impl Fn() -> R) -> Duration {
    let runs = options.times;
    let mut times = Vec::with_capacity(runs);
    for _ in 0..runs {
        let start = std::time::Instant::now();
        let _ = fun();
        let end = std::time::Instant::now();
        let duration = end - start;
        times.push(duration);
    }
    times.iter().sum::<Duration>() / runs as u32
}
