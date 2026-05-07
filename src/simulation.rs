use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Instant;

use crate::generator::generator;
use crate::dispatcher::dispatcher;
use crate::worker::worker;
use crate::monitor::monitor;

use crate::state::{
    State,
    NUM_WORKERS,
    NUM_TASKS,
    CPU_CAP,
};

// ======================================================
// RUN ONE SIMULATION
// ======================================================

pub fn run_simulation(
    title: &str,
    io_ratio: f64,
    optimized: bool,
) {

    println!("\n== {} ==", title);

    println!(
        "{} tasks, {:.0}% IO / {:.0}% CPU, {} workers, cap {}%",
        NUM_TASKS,
        io_ratio * 100.0,
        (1.0 - io_ratio) * 100.0,
        NUM_WORKERS,
        CPU_CAP
    );

    let state = Arc::new(Mutex::new(State {

        cpu_usage: 0,
        active_workers: 0,
        completed: 0,

        cpu_completed: 0,
        io_completed: 0,

        total_wait: 0,
        total_turnaround: 0,

        max_wait: 0,
        max_wait_task: 0,

        cpu_sum: 0,
        worker_sum: 0,
        samples: 0,

        start: Instant::now(),
        end: None,
    }));

    // channels
    let (gen_tx, gen_rx) = mpsc::channel();
    let (worker_tx, worker_rx) = mpsc::channel();

    // generator thread
    let gen_thread = {
        let tx = gen_tx.clone();

        thread::spawn(move || {
            generator(tx, io_ratio)
        })
    };

    // dispatcher thread
    let dispatcher_thread = {
        let tx = worker_tx.clone();
        let st = state.clone();

        thread::spawn(move || {
            dispatcher(gen_rx, tx, st, optimized)
        })
    };

    // worker pool
    let shared_rx =
        Arc::new(Mutex::new(worker_rx));

    let mut workers = vec![];

    for _ in 0..NUM_WORKERS {

        let rx = shared_rx.clone();
        let st = state.clone();

        workers.push(
            thread::spawn(move || {
                worker(rx, st)
            })
        );
    }

    // monitor thread
    let monitor_thread = {
        let st = state.clone();

        thread::spawn(move || {
            monitor(st)
        })
    };

    // wait for generator
    gen_thread.join().unwrap();

    // wait for dispatcher
    dispatcher_thread.join().unwrap();

    // close worker channel
    drop(worker_tx);

    // wait for workers
    for w in workers {
        w.join().unwrap();
    }

    // wait for monitor
    monitor_thread.join().unwrap();

    // ==================================================
    // RESULTS
    // ==================================================

    let st = state.lock().unwrap();

    let total_runtime =
        st.start.elapsed().as_millis();

    let makespan =
        st.end.unwrap()
            .duration_since(st.start)
            .as_millis();

    let avg_wait =
        st.total_wait as f64
            / NUM_TASKS as f64;

    let avg_turnaround =
        st.total_turnaround as f64
            / NUM_TASKS as f64;

    let avg_cpu =
        st.cpu_sum as f64
            / st.samples as f64;

    let avg_workers =
        st.worker_sum as f64
            / st.samples as f64;

    println!("\n— results —");

    println!(
        "total_runtime        : {} ms",
        total_runtime
    );

    println!(
        "makespan             : {} ms",
        makespan
    );

    println!(
        "tasks completed      : {} (IO={}, CPU={})",
        st.completed,
        st.io_completed,
        st.cpu_completed
    );

    println!(
        "avg wait time        : {:.2} ms",
        avg_wait
    );

    println!(
        "avg turnaround time  : {:.2} ms",
        avg_turnaround
    );

    println!(
        "max wait time        : {} ms (task #{})",
        st.max_wait,
        st.max_wait_task
    );

    println!(
        "avg CPU usage        : {:.2} %",
        avg_cpu
    );

    println!(
        "avg workers active   : {:.2} / {}",
        avg_workers,
        NUM_WORKERS
    );

    println!(
        "monitor samples      : {}",
        st.samples
    );

    println!(
        "monitor csv          : monitor_log.csv"
    );
}