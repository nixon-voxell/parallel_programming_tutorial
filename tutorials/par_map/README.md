# Parallel Map

Similar to our [serial map](../serial_map) exercise, here are the example inputs and outputs:

Example input:
```
0, 1, 2, 3, 4, 5, 6, 7, 8, 9
```

Example output:
```
0, 1, 4, 9, 16, 25, 36, 49, 64, 81
```

However, this time, we want to perform the operations in parallel!
The main idea here is that each element in the list will be mutated in a seprate thread.

Here is the boiler plate to start with:

```rust
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
```

## Initial Attempt

Being the happy person you are right now, you proceed to spawn in the threads and tried to mutate/reference the elements inside the `vec` list.

```rust
fn main() {
    // ...
    // Spawn threads
    pool.scope(|s| {
        for thread_index in 0..SIZE {
            s.spawn(async move {
                // Operation inside the thread
                vec[0] = 1;
            });
        }
    });
    // ...
}
```

Oh no! You encountered your first error (and we are just mutating the first element in the list!):

```
error[E0382]: use of moved value: `vec`
  --> src\main.rs:22:21
   |
22 |               s.spawn(async move {
   |  _____________________^
23 | |                 // Operation inside the thread
24 | |                 vec[0] = 1;
   | |                 --- use occurs due to use in coroutine
25 | |             });
   | |_____________^ value moved here, in previous iteration of loop
   |
   = note: move occurs because `vec` has type `Vec<i32>`, which does not implement the `Copy` trait
```

Rust compiler is complaining that because `Vec<T>` (which is the type of `vec`) cannot be copied implicitly, the compiler is forced to "move" the value into the thread.

This means that when we are spawning the first thread, it consumes the `vec` entirely!
Leaving no data for other threads, or even the main thread itself!

We have to think of a much clever way...

If you are a Rust developer you might saw this error and say to yourself: "Oh, why not just `clone()` it???".

Bad news, if you did that, every thread will receive a completely new copy of `vec` and what you did inside the thread remains inside the thread and is not refected in the main thread!

## Pro Rustacean's Journey

However, being the "pro" you are, some of you may think to yourself: "Ahhh, I see where this is going, let's use an `Arc<Mutex<T>>`! What can go wrong?!".
And you begin implementing it...

<details>
<summary>What is `Arc<Mutex<T>>`?</summary>

For those who are new to Rust, you may not understand what it means. Here is how it works:

Imagine you have a treasure chest (the shared data) that you want to protect because many adventurers (threads) are trying to open it at the same time.
To keep things organized, you give each adventurer a key, but there's a rule: only _**one**_ adventurer who has the key can open the chest at any given time.
This prevents them from bumping into each other and making a mess.

Now, the `Arc<Mutex<T>>` is like having a special kind of key ring (Arc) that can make copies of the key, so multiple adventurers can have their own copy of the key.
However, they still have to follow the rule (Mutex) that only one adventurer can use their key to open the chest at a time.
This way, the treasure remains safe, and the adventurers can take turns accessing it without causing chaos.

</details>

```rust
fn main() {
    // ...
    // Initialize `Arc<Mutex<T>>`
    let arc_vec = Arc::new(Mutex::new(&mut vec));
    // Spawn threads
    pool.scope(|s| {
        for thread_index in 0..SIZE {
            let thread_arc_vec = arc_vec.clone();
            s.spawn(async move {
                // Operation inside the thread
                let mut vec = thread_arc_vec.lock().unwrap();
                let element = vec.get_mut(thread_index).unwrap();
                // Perform squaring on the element
                *element = (*element) * (*element);
            });
        }
    });
    // ...
}
```

Everything seems fine, output is showing the correct results...
Then, you suddenly remember there is something more going on when you call the mysterious `lock` function.
You head over to the Rust doucmentation and found [THIS](https://doc.rust-lang.org/std/sync/struct.Mutex.html#method.lock)!

```rust
pub fn lock(&self) -> LockResult<MutexGuard<'_, T>>
```
```
Acquires a mutex, blocking the current thread until it is able to do so.
```

All the while, you were thinking that you are writing a parallel program, but instead, it turns into a "serial" program!
Each thread is in fact waiting for their turn to perform the square operation!
In other words, your threads are not capable of running simultaneously!

## Dark Magic

To solve this issue, we need to introduce a dark magic in Rust, _**`unsafe`**_.
The idea is that using `unsafe`, we can freely access our data and represent them as pointers.
However, as a pro Rustacean, we wouldn't want this dark magic to be polluting our code base everywhere (we don't want to have to write `unsafe` everytime we want to access our `vec`!).

When creating this tutorial, I came up with a solution by introducing a new type which contains the `unsafe` code.
Introducing the `I32VecPtr`:

```rust
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
```

And this is how you use it:

```rust
fn main() {
    // ...
    // Create handle
    let mut vec_handle = I32VecHandle(&mut vec as I32VecPtr);

    // Spawn threads
    pool.scope(|s| {
        for thread_index in 0..vec.len() {
            s.spawn(async move {
                // Get vec
                let vec = vec_handle.get_mut();
                // Perform map operation
                vec[thread_index] = vec[thread_index] * vec[thread_index];
            });
        }
    });
    // ...
}
```

Now, we can finally be rest assured that no threads is waiting for any other threads.
However, with great power comes great responsibility, now that we can mutate our `vec` anywhere we want, it is now our responsibility to:

- Keep our `vec` in scope so that we don't get a null reference.
- Make sure no 2 threads are writing to the same location in `vec` (we never know who's going to end up being the last one writing to the location).
- Make sure each thread is not reading from locations in `vec` that are being mutated in other threads (we never know if it's going to be mutated first or read first).

<details>
<summary>Here is the complete solution:</summary>

```rust
use bevy_tasks::TaskPoolBuilder;

const SIZE: usize = 10;

fn main() {
    // Initialize the list
    let mut vec = vec![0; SIZE];

    for i in 0..vec.len() {
        vec[i] = i as i32;
    }

    // Create handle
    let mut vec_handle = I32VecHandle(&mut vec as I32VecPtr);

    // Initialize thread pool
    let threads = bevy_tasks::available_parallelism();
    let pool = TaskPoolBuilder::new().num_threads(threads).build();

    // Spawn threads
    pool.scope(|s| {
        for thread_index in 0..vec.len() {
            s.spawn(async move {
                // Get vec
                let vec = vec_handle.get_mut();
                // Perform map operation
                vec[thread_index] = vec[thread_index] * vec[thread_index];
            });
        }
    });

    // Verify the output
    for element in vec {
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
```
</details>
