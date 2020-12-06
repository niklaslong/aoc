use std::io::{stdin, BufRead};
use std::ops::Range;

fn main() {
    let stdin = stdin();
    let lines = stdin.lock().lines();

    let mut bps: Vec<BoardingPass> = lines
        .into_iter()
        .take_while(|line| !line.as_ref().unwrap().is_empty())
        .map(|line| {
            let mut bp = BoardingPass {
                binary: String::from(line.unwrap()),
                ..Default::default()
            };

            bp.decode();
            bp
        })
        .collect();

    // Find highest ID.
    bps.sort_by(|a, b| a.seat_id.unwrap().partial_cmp(&b.seat_id.unwrap()).unwrap());
    println!("HIGHEST: {:?}", bps.last());

    // Find missing ID.
    let ids: Vec<u32> = bps.iter().map(|bp| bp.seat_id.unwrap()).collect();

    let missing_id = ids
        .iter()
        .enumerate()
        .find(|(i, &id)| (*i as u32 + ids.first().unwrap() != id));

    println!(
        "MISSING ID: {:?}",
        missing_id.unwrap().0 as u32 + ids.first().unwrap()
    );
}

#[derive(Default, PartialEq, Debug)]
struct BoardingPass {
    binary: String,
    row: Option<u32>,
    column: Option<u32>,
    seat_id: Option<u32>,
}

impl BoardingPass {
    fn decode(&mut self) {
        let (encoded_row, encoded_column) = self.binary.split_at(7);

        // Row decoding.
        let row = binary_search(encoded_row, 0..128);
        self.row = Some(row);

        // Column decoding.
        let column = binary_search(encoded_column, 0..8);
        self.column = Some(column);

        self.seat_id = Some(self.row.unwrap() * 8 + self.column.unwrap());
    }
}

fn binary_search(s: &str, mut range: Range<u32>) -> u32 {
    for char in s.chars() {
        match char {
            'F' | 'L' => range.end -= (range.end - range.start) / 2,
            'B' | 'R' => range.start += (range.end - range.start) / 2,
            _ => panic!("Invalid input"),
        }
    }

    // Start should equal end returning start as it's the inclusive bound in the range (end is
    // exclusive).
    range.start
}

#[cfg(test)]
mod tests {
    use super::*;

    // Examples:
    //
    // BFFFBBFRRR: row 70, column 7, seat ID 567.
    // FFFBBBFRRR: row 14, column 7, seat ID 119.
    // BBFFBBFRLL: row 102, column 4, seat ID 820.

    #[test]
    fn completes_boarding_path_information() {
        let mut bp = BoardingPass {
            binary: String::from("BFFFBBFRRR"),
            ..Default::default()
        };

        let decoded_bp = BoardingPass {
            binary: String::from("BFFFBBFRRR"),
            row: Some(70),
            column: Some(7),
            seat_id: Some(567),
        };

        bp.decode();

        assert_eq!(bp, decoded_bp);
    }
}
