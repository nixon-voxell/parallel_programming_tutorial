use bevy_tasks::TaskPoolBuilder;

const SIZE: usize = 10;

fn main() {
    // Initialize thread pool according to the number
    // of logical cores available in your CPU
    let threads = bevy_tasks::available_parallelism();
    println!("Logical core count: {threads:?}");
    let pool = TaskPoolBuilder::new().num_threads(threads).build();

    // Spawn threads
    pool.scope(|s| {
        for thread_index in 0..SIZE {
            s.spawn(async move {
                // Operation inside the thread
                println!("Hello from thread #{thread_index:?}");
            });
        }
    });
}
