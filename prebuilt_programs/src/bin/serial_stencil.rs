const SIZE: usize = 10;
const STENCIL_COUNT: usize = 3;

fn main() {
    // Initialize the list
    let mut vec = vec![0; SIZE * STENCIL_COUNT];
    let mut output_vec = vec![0; SIZE];

    for i in 0..vec.len() {
        vec[i] = i;
    }

    // Stencil operation
    for i in 0..SIZE {
        // Record the sum
        let mut sum = 0;
        for s in 0..STENCIL_COUNT {
            // Calculate the index of the element inside `vec`
            let index = i * STENCIL_COUNT + s;
            sum += vec[index];
        }

        // write the result to the `output_vec`
        output_vec[i] = sum;
    }

    // Verify the output
    for element in output_vec {
        println!("{:?}", element);
    }
}
