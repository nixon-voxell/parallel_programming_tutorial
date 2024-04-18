use bevy_tasks::TaskPoolBuilder;
use web_time::Instant;

const SIZE: usize = 10000000;

fn main() {
    // Initialize the list
    let mut vec = vec![1; SIZE];
    // Create handle
    let vec_handle = I32VecHandle(&mut vec as I32VecPtr);

    // Initialize thread pool
    let threads = bevy_tasks::available_parallelism();
    let pool = TaskPoolBuilder::new().num_threads(threads).build();

    // Initialize batch size
    let batch_size = usize::max(SIZE / threads, 1);
    println!("batch size: {}", batch_size);
    let loop_count = (SIZE + batch_size - 1) / batch_size;

    let mut pre_sums = vec![0; loop_count];

    let start_time = Instant::now();

    pool.scope(|s| {
        for batch_index in 0..loop_count {
            s.spawn(async move {
                serial_prefix_sum(vec_handle, batch_index, batch_size);
            });
        }
    });

    for batch_index in 0..loop_count {
        for b in 0..batch_index {
            pre_sums[batch_index] += vec[b * batch_size + batch_size - 1];
        }
    }

    pool.scope(|s| {
        for batch_index in 0..loop_count {
            let pre_sum = pre_sums[batch_index];
            s.spawn(async move {
                batch_sum(vec_handle, pre_sum, batch_index, batch_size);
            });
        }
    });

    println!(
        "Parallel task finished in {} secs",
        (Instant::now() - start_time).as_secs_f32()
    );

    println!("Total sum: {}", vec[vec.len() - 1]);
}

fn serial_prefix_sum(mut arr: I32VecHandle, batch_index: usize, batch_size: usize) {
    let arr = arr.get_mut();

    let index_start = batch_index * batch_size + 1;
    let index_end = usize::min(index_start + batch_size - 1, arr.len());

    for i in index_start..index_end {
        arr[i] += arr[i - 1];
    }
}

fn batch_sum(mut arr: I32VecHandle, pre_sum: i32, batch_index: usize, batch_size: usize) {
    let arr = arr.get_mut();

    let index_start = batch_index * batch_size;
    let index_end = usize::min(index_start + batch_size, arr.len());

    for i in index_start..index_end {
        arr[i] += pre_sum;
    }
}

type I32VecPtr = *mut Vec<i32>;

#[derive(Clone, Copy)]
struct I32VecHandle(I32VecPtr);

#[allow(dead_code)]
impl I32VecHandle {
    pub fn get_mut(&mut self) -> &mut [i32] {
        unsafe { &mut *self.0 }
    }

    pub fn get(&self) -> &[i32] {
        unsafe { &*self.0 }
    }
}

unsafe impl Send for I32VecHandle {}
unsafe impl Sync for I32VecHandle {}
