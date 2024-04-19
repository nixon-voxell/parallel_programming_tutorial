# Parallel Programming Tutorial

## Slides

You can find the associated PDF slides [here](./Introduction%20to%20Parallel%20Programming.pdf) or the Canva slides [here](https://www.canva.com/design/DAF-tYUrF-g/mkeJrLY4Sp-a51t100g0Yw/view?utm_content=DAF-tYUrF-g&utm_campaign=designshare&utm_medium=link&utm_source=editor).

## Running Prebuilt Programs

To run any of the prebuilt programs in the repository:

1. Head into [`prebuilt_programs`](./prebuilt_programs) folder by running: `cd prebuilt_programs`.
2. Run the command: `cargo run --bin 'filename'` (e.g. `cargo run --bin spawn_threads`).

## Tutorial Structure

This tutorial is separated into multiple parts inside the [`tutorials`](./tutorials) folder.
Each folder (inside the tutorial folder) contains a markdown (`README.md`) file that explains what you should learn in that section.
Each folder is also a new rust project which means you can do your exercise inside the `src/main.rs` file.

Example for [`spawn_threads`](./tutorials/spawn_threads):

- Tutorial file: [`tutorials/spawn_threads/README.md`](./tutorials/spawn_threads/README.md)
- Exercise file: [`tutorials/spawn_threads/src/main.rs`](./tutorials/spawn_threads/src/main.rs)

### Order of Tutorial

It is highly encouraged to follow the order of this list:

1. [Spawning Threads](./tutorials/spawn_threads)
2. [Serial Map](./tutorials/serial_map)
3. [Parallel Map](./tutorials/par_map)
4. [Serial Stencil](./tutorials/serial_stencil)
5. [Parallel Stencil](./tutorials/par_stencil)
6. [Batching](./tutorials/batch)
