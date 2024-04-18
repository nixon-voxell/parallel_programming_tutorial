const SIZE: usize = 10;

fn main() {
    // Initialize the list
    let mut vec = vec![0; SIZE];

    for i in 0..vec.len() {
        vec[i] = i as i32;
    }

    // Map operation
    for i in 0..vec.len() {
        vec[i] = vec[i] * vec[i];
    }

    // Verify the output
    for element in vec {
        println!("{:?}", element);
    }
}
