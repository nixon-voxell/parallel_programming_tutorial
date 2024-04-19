use bevy_tasks::TaskPoolBuilder;

const SIZE: usize = 5;
const BATCH_COUNT: usize = 3;

fn main() {
    // Initialize thread pool
    let threads = bevy_tasks::available_parallelism();
    let pool = TaskPoolBuilder::new().num_threads(threads).build();

    // Perform batch operation in parallel
    // ===================================
    // CODE GOES HERE
    // ===================================
}
