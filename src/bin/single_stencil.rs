const SIZE: usize = 10;
const STENCIL_COUNT: usize = 3;

fn main() {
    // Initialize the array
    let mut array = vec![0; SIZE * STENCIL_COUNT];
    let mut output_array = vec![0; SIZE];

    for i in 0..array.len() {
        array[i] = i;
    }

    // Stencil operation
    for i in 0..SIZE {
        // Record the sum
        let mut sum = 0;
        for s in 0..STENCIL_COUNT {
            // Calculate the index of the element inside `array`
            let index = i * STENCIL_COUNT + s;
            sum += array[index];
        }

        // write the result to the `output_array`
        output_array[i] = sum;
    }

    // Verify the output
    for element in output_array {
        println!("{:?}", element);
    }
}
