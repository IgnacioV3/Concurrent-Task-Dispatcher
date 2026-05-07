use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;

use crate::state::{
    SharedState,
    NUM_TASKS,
};

// ======================================================
// MONITOR THREAD
// ======================================================

pub fn monitor(state: SharedState) {

    let mut file =
        File::create("monitor_log.csv").unwrap();

    writeln!(
        file,
        "time_ms,cpu_usage,active_workers,completed"
    ).unwrap();

    let mut print_counter = 0;

    loop {

        {
            let mut st = state.lock().unwrap();
            print_counter += 1;

            // collect averages
            st.cpu_sum += st.cpu_usage as u64;
            st.worker_sum += st.active_workers as u64;
            st.samples += 1;

            // live logging
            if print_counter % 100 == 0 {
                println!(
                    "CPU: {}% | Workers: {} | Done: {}",
                    st.cpu_usage,
                    st.active_workers,
                    st.completed
                );
            }

            // csv logging
            let elapsed =
                st.start.elapsed().as_millis();

            writeln!(
                file,
                "{},{},{},{}",
                elapsed,
                st.cpu_usage,
                st.active_workers,
                st.completed
            ).unwrap();

            // shutdown
            if st.completed >= NUM_TASKS {
                break;
            }
        }

        thread::sleep(Duration::from_millis(10));
    }
}