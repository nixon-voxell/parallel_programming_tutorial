use bevy_tasks::TaskPoolBuilder;

const SIZE: usize = 10;

fn main() {
    // Initialize the list
    let mut array = vec![0; SIZE];

    for i in 0..array.len() {
        array[i] = i as i32;
    }

    // Create handle
    let mut arr_handle = I32VecHandle(&mut array as I32VecPtr);

    // Initialize thread pool
    let threads = bevy_tasks::available_parallelism();
    let pool = TaskPoolBuilder::new().num_threads(threads).build();

    // Spawn threads
    pool.scope(|s| {
        for thread_index in 0..array.len() {
            s.spawn(async move {
                // Get array
                let arr = arr_handle.get_mut();
                // Perform map operation
                arr[thread_index] = arr[thread_index] * arr[thread_index];
            });
        }
    });

    // Verify the output
    for element in array {
        println!("{:?}", element);
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
