use std::time::{Duration, Instant};

pub fn time_function<F, R>(f: F) -> Duration
where
    F: FnOnce() -> R,
{
    let start_time = Instant::now();
    f();

    Instant::now() - start_time
}

pub fn get_mean_execution_time<F, R>(f: F) -> Duration
where
    F: FnOnce() -> R + Copy,
{
    const NUM_ITERATIONS: usize = 10000;

    let mut total_time = Duration::from_secs(0);

    for _ in 0..NUM_ITERATIONS {
        let result = time_function(f);
        total_time += result;
    }

    total_time / NUM_ITERATIONS as u32
}


mod wini;
