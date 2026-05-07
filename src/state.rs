use std::sync::{Arc, Mutex};
use std::time::Instant;

// ======================================================
// CONFIG
// ======================================================

pub const NUM_WORKERS: usize = 8;
pub const NUM_TASKS: usize = 1000;
pub const CPU_CAP: u8 = 100;

// ======================================================
// GLOBAL STATE
// ======================================================

pub struct State {
    // live system state
    pub cpu_usage: u8,
    pub active_workers: u8,
    pub completed: usize,

    // metrics
    pub cpu_completed: usize,
    pub io_completed: usize,

    pub total_wait: u128,
    pub total_turnaround: u128,

    pub max_wait: u128,
    pub max_wait_task: usize,

    pub cpu_sum: u64,
    pub worker_sum: u64,
    pub samples: u64,

    // timing
    pub start: Instant,
    pub end: Option<Instant>,
}

pub type SharedState = Arc<Mutex<State>>;