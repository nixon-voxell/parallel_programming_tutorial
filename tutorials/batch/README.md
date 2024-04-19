# Batching

When creating parallel programs with the number of unit tasks exceeding way beyond the number logical cores (hardware limitation) you have, it's better to batch some of your tasks together (despite being capable of running simultaneously).
Batching reduces overhead by minimizing the context-switching between tasks, maximizing CPU utilization, and improving overall efficiency in parallel processing.

The concept of batching looks somewhat similar to how [parallel stencil](../par_stencil) works.
You first spawn a number of threads to be ran in parallel.

```rust
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
                // Inside the thread
            });
        }
    });
}
```

Then, in each thread, loop through the tasks that you want to perform.

```rust
fn main() {
    // ...
    // Spawn threads
    pool.scope(|s| {
        for thread_index in 0..SIZE {
            s.spawn(async move {
                // A loop inside the thread to perform batch operations
                for batch_index in 0..BATCH_COUNT {
                    // Perform batch operation here
                }
            });
        }
    });
}
```

When performing batch execution, it's important to keep track of the global task index.
In our previous examples, we have been using the `thread_index` as the task index.
This time however, because each thread handles a batch of tasks, we cannot only rely on `thread_index`.

We can achieve this by using some simple math:

```rust
fn main() {
    // ...
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
    // ...
}
```

<details>
<summary>

## Complete Solution:
</summary>

```rust
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
```
</details>
