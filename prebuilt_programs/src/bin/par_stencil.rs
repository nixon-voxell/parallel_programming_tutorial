use bevy_tasks::TaskPoolBuilder;

const SIZE: usize = 10;
const STENCIL_COUNT: usize = 3;

fn main() {
    // Initialize the list
    let mut vec = vec![0; SIZE * STENCIL_COUNT];
    let mut output_vec = vec![0; SIZE];

    for i in 0..vec.len() {
        vec[i] = i as i32;
    }

    // Create handle
    let vec_handle = I32VecHandle(&mut vec as I32VecPtr);
    let mut out_vec_handle = I32VecHandle(&mut output_vec as I32VecPtr);

    // Initialize thread pool
    let threads = bevy_tasks::available_parallelism();
    let pool = TaskPoolBuilder::new().num_threads(threads).build();

    // Spawn threads
    pool.scope(|s| {
        for thread_index in 0..SIZE {
            s.spawn(async move {
                // Get vec
                let vec = vec_handle.get();
                let output_vec = out_vec_handle.get_mut();

                // Perform map operation
                let mut sum = 0;
                for s in 0..STENCIL_COUNT {
                    // Calculate the index of the element inside `vec`
                    let index = thread_index * STENCIL_COUNT + s;
                    sum += vec[index];
                }

                output_vec[thread_index] = sum;
            });
        }
    });

    // Verify the output
    for element in output_vec {
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
