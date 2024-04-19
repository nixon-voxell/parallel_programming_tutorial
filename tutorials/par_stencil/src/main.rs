use bevy_tasks::TaskPoolBuilder;

const SIZE: usize = 10;

fn main() {
    // Initialize the list
    let mut vec = vec![0; SIZE];

    for i in 0..vec.len() {
        vec[i] = i as i32;
    }

    // Initialize thread pool
    let threads = bevy_tasks::available_parallelism();
    let pool = TaskPoolBuilder::new().num_threads(threads).build();

    // Perform map operation in parallel
    // ===================================
    // CODE GOES HERE
    // ===================================

    // Verify the output
    for element in vec {
        println!("{:?}", element);
    }
}
