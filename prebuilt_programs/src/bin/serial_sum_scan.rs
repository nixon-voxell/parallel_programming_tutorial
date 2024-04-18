use web_time::Instant;

const SIZE: usize = 10000000;

fn main() {
    // Initialize the list
    let mut vec = vec![1; SIZE];

    let start_time = Instant::now();

    for i in 1..vec.len() {
        vec[i] += vec[i - 1];
    }

    println!(
        "Serial task finished in {} secs",
        (Instant::now() - start_time).as_secs_f32()
    );

    println!("Total sum: {}", vec[vec.len() - 1]);
}
