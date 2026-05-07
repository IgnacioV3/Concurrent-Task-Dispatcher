mod task;
mod state;
mod generator;
mod dispatcher;
mod worker;
mod monitor;
mod simulation;

use simulation::run_simulation;

fn main() {

    run_simulation(
        "FIFO 70/30",
        0.70,
        false,
    );

    run_simulation(
        "FIFO 80/20",
        0.80,
        false,
    );

    run_simulation(
        "Optimized 70/30",
        0.70,
        true,
    );

    run_simulation(
        "Optimized 80/20",
        0.80,
        true,
    );
}