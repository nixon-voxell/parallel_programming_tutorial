//! This sample demonstrates creating a thread pool with 4 tasks and spawning 40 tasks that spin
//! for 100ms. It's expected to take about a second to run (assuming the machine has >= 4 logical
//! cores)

use bevy_tasks::TaskPoolBuilder;
use web_time::Instant;

fn main() {
    // let threads = 4;
    let threads = bevy_tasks::available_parallelism();

    let pool = TaskPoolBuilder::new()
        // .num_threads(bevy_tasks::available_parallelism())
        .num_threads(threads)
        .build();

    let count = 100000000;

    let mut arr0 = vec![1; count];
    let mut arr1 = arr0.clone();

    let mut arr0_ptr = &mut arr0 as *mut Vec<i32> as usize;
    let mut arr1_ptr = &mut arr1 as *mut Vec<i32> as usize;

    let scan_count = usize::ilog2(arr0.len()) as usize;

    let batch_size = count / threads;
    println!("batch size: {}", batch_size);
    let loop_count = (count + batch_size - 1) / batch_size;

    let start_time = Instant::now();

    for s in 0..=scan_count {
        let offset = 1 << s;
        println!("{}", offset);

        pool.scope(|s| {
            for l in 0..loop_count {
                s.spawn(async move {
                    let index_start = l * batch_size;
                    let index_end = usize::min(index_start + batch_size, count);

                    for i in index_start..index_end {
                        unsafe {
                            prefix_sum_element(
                                i,
                                offset,
                                arr0_ptr as *mut Vec<i32>,
                                arr1_ptr as *mut Vec<i32>,
                            );
                        }
                    }
                })
            }
        });
        // unsafe {
        //     prefix_sum_element(i, offset, arr0_ptr, arr1_ptr);
        // }

        let temp_ptr = arr0_ptr;
        arr0_ptr = arr1_ptr;
        arr1_ptr = temp_ptr;
    }

    println!(
        "Parallel task finished in {} secs",
        (Instant::now() - start_time).as_secs_f32()
    );

    println!(
        "Total sum: {}",
        if scan_count % 2 == 0 {
            arr1[count - 1]
        } else {
            arr0[count - 1]
        }
    );
}

#[inline(always)]
unsafe fn prefix_sum_element(
    index: usize,
    offset: usize,
    arr_in: *const Vec<i32>,
    arr_out: *mut Vec<i32>,
) {
    let arr_in = &*arr_in;
    let arr_out = &mut *arr_out;

    if offset > index {
        arr_out[index] = arr_in[index];
    } else {
        arr_out[index] = arr_in[index] + arr_in[index - offset];
    }
}
