# Serial Map

Try to come up with a serial algorithm that takes in a list of integers and returns out the squared of those integers.

Example input:
```
0, 1, 2, 3, 4, 5, 6, 7, 8, 9
```

Example output:
```
0, 1, 4, 9, 16, 25, 36, 49, 64, 81
```

<details>
<summary>Here is the complete solution:</summary>

```rust
const SIZE: usize = 10;

fn main() {
    // Initialize the array
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
```
</details>
