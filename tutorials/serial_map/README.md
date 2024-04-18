# Serial Map

The map operation is a one to one operation, it does not take dependencies from any other elements.
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
<summary>

## Complete Solution:
</summary>

```rust
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
```
</details>
