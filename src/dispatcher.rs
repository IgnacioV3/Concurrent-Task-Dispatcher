use std::collections::VecDeque;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::task::{Task, TaskType};
use crate::state::{
    SharedState,
    CPU_CAP,
    NUM_TASKS,
    NUM_WORKERS,
};

// ======================================================
// DISPATCHER THREAD
// ======================================================

pub fn dispatcher(
    rx: mpsc::Receiver<Task>,
    worker_tx: mpsc::Sender<Task>,
    state: SharedState,
    optimized: bool,
) {

    // FIFO queue
    let mut queue: VecDeque<Task> = VecDeque::new();

    loop {

        // receive all incoming tasks
        while let Ok(task) = rx.try_recv() {
            queue.push_back(task);
        }

        // look at next task
        if let Some(front_task) = queue.front() {

            let mut st = state.lock().unwrap();

            // can we schedule this task?
            let enough_cpu =
                st.cpu_usage + front_task.cpu_cost <= CPU_CAP;

            let worker_available =
                st.active_workers < NUM_WORKERS as u8;

            if enough_cpu && worker_available {

                let task = queue.pop_front().unwrap();

                // optimized scheduler:
                // if CPU already high, delay CPU-heavy tasks
                if optimized && st.cpu_usage > 70 {

                    if let TaskType::CPU = task.kind {
                        queue.push_back(task);
                        drop(st);

                        thread::sleep(Duration::from_millis(1));
                        continue;
                    }
                }

                st.cpu_usage += task.cpu_cost;
                st.active_workers += 1;

                worker_tx.send(task).unwrap();
            }
        }

        // shutdown condition
        if state.lock().unwrap().completed >= NUM_TASKS {
            break;
        }

        thread::sleep(Duration::from_millis(1));
    }
}