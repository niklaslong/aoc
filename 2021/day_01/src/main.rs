use std::fs;

fn main() {
    // Read input from file.
    let input_bytes = fs::read("input.txt").unwrap();
    let depths: Vec<u32> = String::from_utf8_lossy(&input_bytes)
        .split('\n')
        .filter(|substr| !substr.is_empty())
        .map(|num| num.parse().unwrap())
        .collect();

    // Count the increases in depth, iterate over contiguous pairs. Itertools is the obvious choice
    // but what about std only?

    // Method 1, not very elegant, uses indexes to peek the next values.
    let count = count_depth_increases(depths.clone());
    println!("1st method, part 1 count: {}", count);

    let triple_sums = sum_depth_triples(depths.clone());
    let count = count_depth_increases(triple_sums);
    println!("1st method, part 2 count: {}", count);

    println!("==");

    // Method 2, uses slice windows.
    let count = slice_method(depths.clone());
    println!("2nd method, part 1 count: {}", count);

    let count = slice_method_triples(depths);
    println!("2nd method, part 2 count: {}", count);
}

fn count_depth_increases(depths: Vec<u32>) -> usize {
    depths
        .iter()
        .enumerate()
        .filter(|(i, depth)| {
            if let Some(val) = depths.get(i + 1) {
                *depth < val
            } else {
                false
            }
        })
        .count()
}

fn sum_depth_triples(depths: Vec<u32>) -> Vec<u32> {
    depths
        .iter()
        .enumerate()
        .map(|(i, depth)| {
            if let (Some(next), Some(second_next)) = (depths.get(i + 1), depths.get(i + 2)) {
                depth + next + second_next
            } else {
                // Floor the sum in case there isn't a contiguous window of three depths.
                0
            }
        })
        .collect()
}

fn slice_method(depths: Vec<u32>) -> usize {
    depths
        .windows(2)
        .filter(|slice| slice[0] < slice[1])
        .count()
}

fn slice_method_triples(depths: Vec<u32>) -> usize {
    let sums = depths.windows(3).map(|slice| slice.iter().sum()).collect();
    slice_method(sums)
}
