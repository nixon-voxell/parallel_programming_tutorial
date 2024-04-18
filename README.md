# Parallel Programming Tutorial

## Running Prebuilt Programs

To run any of the prebuilt programs in the repository, simply run the command: `cargo run --bin 'filename'` (e.g. `cargo run --bin spawn_threads`).

## Tutorial Structure

This tutorial is separated into multiple parts inside the [`tutorials`](./tutorials) folder.
Each folder (inside the tutorial folder) contains a markdown (`README.md`) file that explains what you should learn in that section.
Each folder is also a new rust project which means you can do your exercise inside the `src/main.rs` file.

Example for `spawn_threads`:

- Tutorial file: [`tutorials/spawn_threads/spawn_threads.md`](./tutorials/spawn_threads/README.md)
- Exercise file: [`tutorials/spawn_threads/src/main.rs`](./tutorials/spawn_threads/src/main.rs)

### Order of Tutorial

It is highly encouraged to follow the order of this list:

1. [Spawning Threads](./tutorials/spawn_threads)
2. [Single Threaded Map](./tutorials/single_map)
3. [Parallel Map](./tutorials/parallel_map)
4. [Single Threaded Stencil](./tutorials/single_stencil)
5. [Parallel Stencil](./tutorials/parallel_stencil)
