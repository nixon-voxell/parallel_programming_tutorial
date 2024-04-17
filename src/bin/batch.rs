use bevy_tasks::TaskPoolBuilder;

const SIZE: usize = 5;
const BATCH_COUNT: usize = 3;

fn main() {
    // Initialize thread pool
    let threads = bevy_tasks::available_parallelism();
    let pool = TaskPoolBuilder::new().num_threads(threads).build();

    // Spawn threads
    pool.scope(|s| {
        for thread_index in 0..SIZE {
            s.spawn(async move {
                // A loop inside the thread to perform batch operations
                for batch_index in 0..BATCH_COUNT {
                    // Some expensive calculations
                    let mut num = 12345.678;
                    for _ in 0..10000 {
                        num = f32::sqrt(num);
                        num = f32::powf(num, 2.0);
                    }

                    println!(
                        "thread #{:?}, batch #{:?}, global #{:?}, result: {:?}",
                        thread_index,
                        batch_index,
                        batch_index + thread_index * BATCH_COUNT,
                        num
                    );
                }
            });
        }
    });
}
