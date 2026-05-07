use std::time::{Duration, Instant};

// ======================================================
// TASK TYPES
// ======================================================

#[derive(Clone)]
pub enum TaskType {
    IO,
    CPU,
}

#[derive(Clone)]
pub struct Task {
    pub id: usize,
    pub arrival: Instant,
    pub duration: Duration,
    pub kind: TaskType,
    pub cpu_cost: u8,
}