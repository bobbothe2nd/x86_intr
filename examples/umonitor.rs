use std::arch::x86_64::_rdtsc;
use std::hint::black_box;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::{Duration, Instant};

use x86_intr::{_umonitor, _umwait, is_cpuid_feature_detected};

const WARMUP: usize = 10;
const SAMPLES: usize = 100;

const DELAYS: &[Duration] = &[
    Duration::from_micros(1),
    Duration::from_micros(10),
    Duration::from_micros(100),
    Duration::from_millis(1),
    Duration::from_millis(10),
    Duration::from_millis(100),
];

#[derive(Default)]
struct Stats {
    samples: Vec<Duration>,
    iterations: u64,
}

impl Stats {
    fn add(&mut self, elapsed: Duration, iterations: u64) {
        self.samples.push(elapsed);
        self.iterations += iterations;
    }

    fn print(&mut self, name: &str) {
        self.samples.sort_unstable();

        let total: Duration = self.samples.iter().sum();
        let mean = total / self.samples.len() as u32;

        let median = self.samples[self.samples.len() / 2];
        let p95 = self.samples[self.samples.len() * 95 / 100];
        let p99 = self.samples[self.samples.len() * 99 / 100];

        let avg_iterations = self.iterations as f64 / self.samples.len() as f64;

        println!(
            "{name:<12} \
             mean {:>10.3?} \
             median {:>10.3?} \
             p95 {:>10.3?} \
             p99 {:>10.3?} \
             iterations {:>10.1}",
            mean, median, p95, p99, avg_iterations,
        );
    }
}

fn wait_spin(flag: &AtomicBool) -> u64 {
    let mut iterations = 0;

    while !flag.load(Ordering::Acquire) {
        iterations += 1;
        std::hint::spin_loop();
    }

    iterations
}

fn wait_umonitor(flag: &AtomicBool) -> u64 {
    let mut iterations = 0;

    while !flag.load(Ordering::Acquire) {
        iterations += 1;

        unsafe {
            _umonitor(flag as *const AtomicBool as *const u8);

            let deadline = _rdtsc().wrapping_add(100_000);
            _umwait(0, deadline);
        }
    }

    iterations
}

fn run_park(delay: Duration, samples: usize) -> Stats {
    let mut stats = Stats::default();

    for _ in 0..samples {
        let writer = thread::spawn(|| {
            let iterations = 0;
            thread::park();
            iterations
        });

        let start = Instant::now();

        thread::sleep(delay);

        writer.thread().unpark();

        let iterations = writer.join().unwrap();

        stats.add(start.elapsed(), iterations);
    }

    stats
}

fn run_wait(delay: Duration, samples: usize, wait: fn(&AtomicBool) -> u64) -> Stats {
    let mut stats = Stats::default();

    for _ in 0..samples {
        let flag = Arc::new(AtomicBool::new(false));
        let ready = Arc::new(Barrier::new(2));

        let flag_writer = Arc::clone(&flag);
        let ready_writer = Arc::clone(&ready);

        let writer = thread::spawn(move || {
            ready_writer.wait();

            if !delay.is_zero() {
                thread::sleep(delay);
            }

            flag_writer.store(true, Ordering::Release);
        });

        ready.wait();

        let start = Instant::now();
        let iterations = wait(&flag);
        let elapsed = start.elapsed();

        black_box(flag.load(Ordering::Acquire));

        writer.join().unwrap();

        stats.add(elapsed, iterations);
    }

    stats
}

fn main() {
    if !is_cpuid_feature_detected!("waitpkg") {
        panic!("requires `waitpkg` target feature");
    }

    println!("Synchronization benchmark");
    println!("samples: {SAMPLES}, warmup: {WARMUP}");

    for &delay in DELAYS {
        run_wait(delay, WARMUP, wait_spin);
        run_wait(delay, WARMUP, wait_umonitor);
        run_park(delay, WARMUP);

        let mut spin = run_wait(delay, SAMPLES, wait_spin);
        let mut _umonitor = run_wait(delay, SAMPLES, wait_umonitor);
        let mut park = run_park(delay, SAMPLES);

        println!();
        println!("delay: {delay:?}");

        print!("  ");
        spin.print("spin");

        print!("  ");
        _umonitor.print("_umonitor");

        print!("  ");
        park.print("park");
    }
}
