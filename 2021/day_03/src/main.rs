// Sample:
//
// 00100
// 11110
// 10110
// 10111
// 10101
// 01111
// 00111
// 11100
// 10000
// 11001
// 00010
// 01010
//
// Totals in first col: 5x0 and 7x1 -> gamma bits[0] is 1 and epsilon bits[0] is 0.
//
// Another possible solution could use in-place matrix transposition before summing over N chunks.

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    // Read input file.
    let bytes = &std::fs::read("input.txt")?;

    // Newline is ascii 10, so we can reliably match on the chunks.
    let chunk_size = String::from_utf8_lossy(&bytes).find("\n").unwrap();

    dbg!(chunk_size);

    // Filter out the newline characters.
    let mut filtered_bytes: Vec<u8> = bytes
        .into_iter()
        .filter(|&byte| !byte.is_ascii_whitespace())
        .copied()
        .collect();

    let chunk_count = filtered_bytes.len() / chunk_size;

    // Represent the binary chunks as u32.
    let bit_chunks: Vec<u32> = filtered_bytes
        .chunks(chunk_size)
        .map(|chunk| u32::from_str_radix(&String::from_utf8_lossy(chunk), 2).unwrap())
        .collect();

    // Do the bit-twiddling.
    let mask: u32 = !(u32::MAX - 1);
    let ones_counts: Vec<u32> = (1..=chunk_size)
        .into_iter()
        .map(|offset| {
            bit_chunks
                .iter()
                .map(|chunk| (chunk >> chunk_size - offset) & mask)
                .sum()
        })
        .collect();

    // Gamma: if there is a majority of 1s for N chunks, bit at that index is 1.
    let gamma_string: String = ones_counts
        .iter()
        .map(|&count| {
            if count as usize > chunk_count / 2 {
                "1"
            } else {
                "0"
            }
        })
        .collect();

    let epsilon_string: String = ones_counts
        .iter()
        .map(|&count| {
            if count as usize > chunk_count / 2 {
                "0"
            } else {
                "1"
            }
        })
        .collect();

    let gamma = u32::from_str_radix(&gamma_string, 2)?;
    let epsilon = u32::from_str_radix(&epsilon_string, 2)?;

    dbg!(gamma);
    dbg!(epsilon);

    dbg!(gamma * epsilon);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn masking() {
        let mask = !(u32::MAX - 1);
        assert_eq!(13u32 & mask, 1);
    }
}
