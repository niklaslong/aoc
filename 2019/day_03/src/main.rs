// Initial solution idea.
//
// The paths are encoded with U, D, L, R instructions as well as a distance for each of these.
// Finding the intersections would mean computing each coordinate and seeing if there's an overlap.
// Then calculating the distance to the origin for each of these, closest wins.
//
// A slightly less naïve solution would be to calculate the general equation (requires starting
// and ending coordinates) for each line of the wire paths and calculate the intersection points
// for each of these. Then repeat the distance calculation mentioned above.

use ndarray::prelude::*;
use ndarray_linalg::Solve;

fn main() {
    let a = array![[2., 1.], [-1., 1.]];
    let b = array![5., 2.];
    let x = a.solve(&b).unwrap();

    println!("X: {:?}", x);
}

struct Line {
    /// Start and finish coordinates.
    coordinates: [(i64, i64), 2],
    /// General form, ax + by + c.
    equation: [i64, 3]
}

fn path() {}

fn line() {
    // Two tuples with the start and finish coordinates.
    // The struct could contain the general form as well.
}

fn coordinates() {
    // Maybe an array of lines?
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
