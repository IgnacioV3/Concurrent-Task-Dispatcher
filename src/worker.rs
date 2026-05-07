use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Instant;

use crate::task::{Task, TaskType};
use crate::state::{
    SharedState,
    NUM_TASKS,
};

// ======================================================
// WORKER THREAD
// ======================================================

pub fn worker(
    rx: Arc<Mutex<mpsc::Receiver<Task>>>,
    state: SharedState,
) {

    loop {

        // receive task
        let task = {
            let lock = rx.lock().unwrap();
            lock.recv()
        };

        // channel closed = shutdown
        if task.is_err() {
            break;
        }

        let task = task.unwrap();

        // task starts now
        let start_exec = Instant::now();

        let wait_time =
            start_exec
                .duration_since(task.arrival)
                .as_millis();

        // simulate execution
        thread::sleep(task.duration);

        // task finished
        let turnaround =
            Instant::now()
                .duration_since(task.arrival)
                .as_millis();

        let mut st = state.lock().unwrap();

        st.cpu_usage -= task.cpu_cost;
        st.active_workers -= 1;
        st.completed += 1;

        st.total_wait += wait_time;
        st.total_turnaround += turnaround;

        // max wait tracking
        if wait_time > st.max_wait {
            st.max_wait = wait_time;
            st.max_wait_task = task.id;
        }

        // IO vs CPU completed
        match task.kind {
            TaskType::IO => st.io_completed += 1,
            TaskType::CPU => st.cpu_completed += 1,
        }

        // final completion time
        if st.completed == NUM_TASKS {
            st.end = Some(Instant::now());
        }
    }
}