//! Native baseline: step time for N persons with 1..=T threads.
//! Usage: bench [entities] [ticks] [work] [threads...]
//! Prints one JSON line per thread count.

use sim_core::World;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let entities: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(10_000);
    let ticks: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(2_000);
    let work: u32 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(0);
    let mut threads: Vec<usize> = args[4.min(args.len())..]
        .iter()
        .filter_map(|s| s.parse().ok())
        .collect();
    if threads.is_empty() {
        threads = vec![1, 2, 4];
    }
    for t in threads {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(t)
            .build()
            .expect("thread pool");
        let setup = Instant::now();
        let mut w = World::new(entities, 42);
        w.work = work;
        let setup_ms = setup.elapsed().as_secs_f64() * 1e3;
        let chunk = entities.div_ceil(t * 4);
        let mut samples = Vec::with_capacity(ticks);
        pool.install(|| {
            for _ in 0..ticks {
                let s = Instant::now();
                if t == 1 {
                    w.step_serial();
                } else {
                    w.step_parallel(chunk);
                }
                samples.push(s.elapsed().as_secs_f64() * 1e6);
            }
        });
        samples.sort_by(f64::total_cmp);
        let mean = samples.iter().sum::<f64>() / samples.len() as f64;
        let p = |q: f64| samples[((samples.len() - 1) as f64 * q) as usize];
        println!(
            "{{\"variant\":\"native\",\"entities\":{entities},\"ticks\":{ticks},\"work\":{work},\"threads\":{t},\"setup_ms\":{setup_ms:.1},\"step_us_mean\":{mean:.1},\"step_us_p50\":{:.1},\"step_us_p99\":{:.1},\"checksum\":\"{:016x}\"}}",
            p(0.5),
            p(0.99),
            w.checksum()
        );
    }
}
