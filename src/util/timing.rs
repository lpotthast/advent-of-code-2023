pub fn time<R: std::fmt::Debug>(fun: impl FnOnce() -> R) -> (R, std::time::Duration) {
    let start = std::time::Instant::now();
    let result = fun();
    let end = std::time::Instant::now();
    let duration = end - start;
    tracing::info!(
        took = format!("{} μs", duration.as_micros()),
        ?result,
        "result"
    );
    (result, duration)
}
