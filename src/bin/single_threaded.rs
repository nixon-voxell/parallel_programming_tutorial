use web_time::Instant;

fn main() {
    const COUNT: usize = 100000000;

    let mut prefix_sum = vec![1; COUNT];

    let start_time = Instant::now();

    for i in 1..prefix_sum.len() {
        prefix_sum[i] += prefix_sum[i - 1];
    }

    println!(
        "Serial task finished in {} secs",
        (Instant::now() - start_time).as_secs_f32()
    );

    println!("Total sum: {}", prefix_sum[prefix_sum.len() - 1]);
}
