use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use crate::task::{Task, TaskType};
use crate::state::NUM_TASKS;

// ======================================================
// GENERATOR THREAD
// ======================================================

pub fn generator(tx: mpsc::Sender<Task>, io_ratio: f64) {

    // fixed seed for reproducibility
    let mut rng = StdRng::seed_from_u64(42);

    for id in 0..NUM_TASKS {

        let is_io = rng.gen_bool(io_ratio);

        let task = Task {
            id,
            arrival: Instant::now(),
            duration: Duration::from_millis(200),

            kind: if is_io {
                TaskType::IO
            } else {
                TaskType::CPU
            },

            cpu_cost: if is_io { 10 } else { 35 },
        };

        tx.send(task).unwrap();

        // simulate arrival intervals
        thread::sleep(Duration::from_millis(20));
    }
}