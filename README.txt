# Rust Concurrent Task Dispatcher

## Project Overview

This project implements a concurrent task dispatcher simulation in Rust using multithreading, channels, shared state synchronization, and a bounded worker pool.

The system simulates how an operating-system-style scheduler receives tasks, places them into a queue, and dispatches them to workers while respecting CPU resource constraints.

The project demonstrates:

* Concurrent programming in Rust
* FIFO scheduling
* Worker pool architecture
* Shared state synchronization
* CPU vs IO workload behavior
* Runtime monitoring and instrumentation
* Clean thread shutdown

---

# System Architecture

The system contains the following major components:

| Component      | Responsibility                               |
| -------------- | -------------------------------------------- |
| Task Generator | Creates randomized tasks over time           |
| Dispatcher     | Maintains FIFO queue and schedules tasks     |
| Worker Pool    | Executes tasks concurrently                  |
| Monitor Thread | Collects runtime statistics and logs metrics |
| Shared State   | Stores global workload statistics            |

---

# Task Model

Each task contains:

* `id`
* `arrival_time`
* `kind` (`CPU` or `IO`)
* `duration`
* `cpu_cost`

Task behavior:

| Task Type | Duration | CPU Cost |
| --------- | -------- | -------- |
| IO Task   | 200ms    | 10%      |
| CPU Task  | 200ms    | 35%      |

---

# Scheduling Policy

The dispatcher uses a FIFO (First-In First-Out) scheduling policy implemented with:

```rust
VecDeque<Task>
```

Tasks are:

* inserted at the back of the queue
* removed from the front of the queue

The dispatcher only schedules tasks when:

* sufficient CPU capacity exists
* a worker thread is available

Global CPU usage is capped at 100%.

An optimized scheduling mode is also included which delays CPU-heavy tasks when system load is already high.

---

# Concurrency Design

The project uses the following Rust concurrency primitives:

| Primitive       | Purpose                           |
| --------------- | --------------------------------- |
| `thread::spawn` | Concurrent execution              |
| `mpsc::channel` | Message passing between threads   |
| `Arc`           | Shared ownership across threads   |
| `Mutex`         | Safe shared-state synchronization |

### Thread Roles

| Thread            | Purpose             |
| ----------------- | ------------------- |
| Main Thread       | System coordination |
| Generator Thread  | Produces tasks      |
| Dispatcher Thread | Schedules tasks     |
| 8 Worker Threads  | Execute tasks       |
| Monitor Thread    | Records metrics     |

Total concurrent threads: 12

---

# Shared State

Global runtime metrics are stored inside:

```rust
Arc<Mutex<State>>
```

Tracked statistics include:

* CPU usage
* active workers
* completed tasks
* wait time
* turnaround time
* max wait time
* worker utilization

---

# Experiments

The project compares FIFO and optimized scheduling policies under both balanced and stressed workloads to evaluate CPU utilization, worker utilization, wait time, and overall runtime behavior.

The following simulations were tested:

| Scheduler | Workload         |
| --------- | ---------------- |
| FIFO      | 70% IO / 30% CPU |
| FIFO      | 80% IO / 20% CPU |
| Optimized | 70% IO / 30% CPU |
| Optimized | 80% IO / 20% CPU |

## Observations

* IO-heavy workloads resulted in lower CPU pressure and higher concurrency.
* CPU-heavy workloads increased queue wait times and worker utilization.
* The optimized scheduler reduced congestion by delaying CPU-heavy tasks under high system load.
* FIFO scheduling preserved fairness by processing tasks in arrival order.

---

# Metrics Collected

The program records:

* total runtime
* makespan
* total completed tasks
* average wait time
* average turnaround time
* average CPU usage
* average worker utilization
* max wait time
* CPU task completions
* IO task completions

A CSV monitor log is also generated:

```text
monitor_log.csv
```

---

# Build Instructions

## Build Project

```bash
cargo build
```

## Run Project

```bash
cargo run
```

## Run Optimized Build

```bash
cargo run --release
```

---

# Command Examples

## Build project

```bash
cargo build
```

## Run project

```bash
cargo run
```

## Run optimized release build

```bash
cargo run --release
```

---

# Example Output

```text
--- results ---
total_runtime        : 33178 ms
makespan             : 33177 ms
tasks completed      : 1000 (IO=807, CPU=193)
avg wait time        : 6100.71 ms
avg turnaround time  : 6300.80 ms
max wait time        : 12879 ms (task #999)
avg CPU usage        : 89.38 %
avg workers active   : 6.03 / 8
monitor samples      : 3232
monitor csv          : monitor_log.csv
```

---

# Clean Shutdown

The system shuts down cleanly by:

* closing channels after task generation finishes
* allowing workers to detect closed channels
* joining all threads before program exit
* preventing infinite loops and hanging workers

---

# Tool Use Disclosure

This project was developed with the assistance of AI tools for:

* Rust syntax clarification
* concurrency design discussion
* debugging support
* architecture explanations

All code structure, debugging, testing, and final understanding were reviewed and verified manually.

Example accepted advice:

* Understanding how Arc<Mutex<_>> allows multiple threads to safely share and update global state.

Example modified/fixed advice:

* Improving the worker synchronization logic by ensuring the shared receiver mutex is only locked while receiving tasks, preventing unnecessary blocking between worker threads.

---

# Future Improvements

Potential future improvements include:

* Separate CPU and IO task queues
* Priority-based scheduling
* Aging to reduce starvation
* Dynamic worker scaling
* Real-time queue visualization
* More advanced load-balancing policies
* Additional fairness and throughput metrics

---

# Concepts Demonstrated

* Multithreading
* FIFO scheduling
* Message passing
* Shared-state synchronization
* Worker pools
* Resource-aware scheduling
* Concurrent task execution
* Runtime instrumentation
* Clean shutdown coordination
