const SIZE: usize = 10;

fn main() {
    // Initialize the list
    let mut array = vec![0; SIZE];

    for i in 0..array.len() {
        array[i] = i as i32;
    }

    // Map operation
    for i in 0..array.len() {
        array[i] = array[i] * array[i];
    }

    // Verify the output
    for element in array {
        println!("{:?}", element);
    }
}
