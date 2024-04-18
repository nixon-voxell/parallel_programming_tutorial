# Spawning Threads

The first entry into parallel programming (or concurrent programming for that matter) is to spawn worker threads.
In this tutorial, we won't be manually spawning threads using the standard library, instead we will utilize the [`bevy_tasks`](https://docs.rs/bevy_tasks/latest/bevy_tasks/) crate to create a thread pool and spawn our threads using the thread pool.
The allows us to abstract away the need for thread management and ease you into learning some basic parallel programming concepts faster.

In your `Cargo.toml` file, add this line under the `[dependencies]` section:

```toml
bevy_tasks = { version = "0.13", features = ["multi-threaded"] }
```

We are going to use the [`TaskPoolBuilder`](https://docs.rs/bevy_tasks/latest/bevy_tasks/struct.TaskPoolBuilder.html) from `bevy_tasks` to create the thread pool using the builder pattern:

```rust
use bevy_tasks::TaskPoolBuilder;

fn main() {}
```

Inside the main function, we create our thread pool:

```rust
use bevy_tasks::TaskPoolBuilder;

fn main() {
    // Initialize thread pool according to the number
    // of logical cores available in your CPU
    let threads = bevy_tasks::available_parallelism();
    println!("Logical core count: {threads:?}");
    let pool = TaskPoolBuilder::new().num_threads(threads).build();
}
```

Next, we define a constant number `SIZE` that determines the number of threads that we are going to spawn:

```rust
const SIZE: usize = 10;

fn main() {
    // ...
}
```

Using this constant number, we spawn our threads:

```rust
fn main() {
    // ...
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
```

Try and run the program and see the output!

## Full Code

```rust
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
```
