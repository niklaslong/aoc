#![feature(option_result_contains)]

use std::io::{stdin, BufRead};

fn main() {
    // Thoughts:
    //
    // - Can probably use modulo arithmetic (wrapping) to account for inifinite repetition of pattern.
    // - We need to consider the forest "line" by "line".
    // - Vec of lines?

    let stdin = stdin();
    let lines = stdin.lock().lines();
    let lines: Vec<TreeLine> = lines
        .into_iter()
        .take_while(|line| !line.as_ref().unwrap().is_empty())
        .map(|line| TreeLine(String::from(line.unwrap())))
        .collect();

    let forest = Forest(lines);
    let count_1 = forest.traverse(1, 1);
    let count_3 = forest.traverse(3, 1);
    let count_5 = forest.traverse(5, 1);
    let count_7 = forest.traverse(7, 1);
    let count_1_2 = forest.traverse(1, 2);

    println!(
        "COUNT: {}",
        count_1 * count_3 * count_5 * count_7 * count_1_2
    );
}

#[derive(Debug)]
struct Forest(Vec<TreeLine>);

impl Forest {
    fn traverse(&self, right: usize, down: usize) -> usize {
        let (count, _) =
            self.0
                .iter()
                .step_by(down)
                .fold((0usize, 0usize), |(count, i), tree_line| {
                    if tree_line.nth_is_tree(i) {
                        (count + 1, i + right)
                    } else {
                        (count, i + right)
                    }
                });

        count
    }
}

#[derive(Debug)]
struct TreeLine(String);

impl TreeLine {
    fn nth_is_tree(&self, index: usize) -> bool {
        let chars = self.0.as_str().chars();
        let len = chars.count();

        // Modulo wrapping as TreeLine repeats infinitely.
        self.0.as_str().chars().nth(index % len).contains(&'#')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forest_traversal() {
        let forest = Forest(vec![
            TreeLine(String::from(".#...#....")),
            TreeLine(String::from("...#.#..#.")),
            TreeLine(String::from("..#....#..")),
            TreeLine(String::from("...#..##..")),
            TreeLine(String::from("..#...#...")),
        ]);

        assert_eq!(forest.traverse(3, 1), 2);
    }

    #[test]
    fn checks_for_tree_in_tree_line() {
        // Note: in this case length is 10 (see modulo wrapping).
        let tree_line = TreeLine(String::from(".#...#...."));

        assert_eq!(tree_line.nth_is_tree(0), false);
        assert_eq!(tree_line.nth_is_tree(5), true);

        // Check for modulo wrapping:
        assert_eq!(tree_line.nth_is_tree(11), true);
    }
}
