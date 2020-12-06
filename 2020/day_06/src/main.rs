use std::io::{stdin, BufRead};

// NOTE: this whole solution needs tests, structure and refactoring.

fn main() {
    let stdin = stdin();
    let lines = stdin.lock().lines();

    let lines: Vec<String> = lines
        .into_iter()
        // Bad solution but in this case easy because data won't contain any numbers.
        .take_while(|line| line.as_ref().unwrap() != "1")
        .map(|line| String::from(line.unwrap()))
        .collect();

    // Part 1.
    let answers = group_answers(lines.clone(), "");

    let cumulative_answers = answers
        .iter()
        .map(|group| {
            let mut g: Vec<char> = group.chars().collect();
            g.sort();
            g.dedup();
            g
        })
        .fold(0, |acc, group| acc + group.len());

    println!("CUMULATIVE: {}", cumulative_answers);

    // Part 2.
    let answers = group_answers(lines, " ");

    // answers.iter().map(|group| {
    //     let g: Vec<String> = group.split_whitespace().collect();
    // })

    let mut counts: Vec<u32> = vec![];

    for group in answers {
        let n = group.chars().filter(|char| char.is_whitespace()).count() + 1;
        let s: String = group.split_whitespace().collect();
        let mut s: Vec<char> = s.chars().collect();
        s.sort();
        let s: String = s.iter().collect();

        let mut count = 0;

        // This iteration is a bit of a crappy solution, there must be something more elegant...
        // regex?
        for char in vec![
            "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q",
            "r", "s", "t", "u", "v", "w", "x", "y", "z",
        ] {
            if s.contains(char.repeat(n).as_str()) {
                count += 1
            }
        }

        counts.push(count);
    }

    let sum: u32 = counts.iter().sum();
    println!("SUM: {}", sum);
}

fn group_answers(lines: Vec<String>, splitter: &str) -> Vec<String> {
    lines
        .split(|line| line.is_empty())
        .map(|line| line.join(splitter))
        .collect()
}
