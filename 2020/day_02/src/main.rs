#![feature(option_result_contains)]

use std::io::BufRead;
use std::ops::Range;
use std::str::FromStr;

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let passwords: Vec<Password> = lines
        .into_iter()
        .take_while(|line| !line.as_ref().unwrap().is_empty())
        .map(|line| Password::from_str(&line.unwrap()))
        .collect();

    let count = count_valid_passwords(passwords);
    println!("COUNT: {:?}", count);
}

#[derive(Debug, PartialEq)]
struct Password {
    range: Range<u32>,
    char: char,
    password: String,
}

impl Password {
    fn from_str(password: &str) -> Self {
        let components: Vec<&str> = password.split_whitespace().collect();

        let bounds: Vec<u32> = components[0]
            .split("-")
            .map(|bound| u32::from_str(bound).unwrap())
            .collect();

        let char = components[1]
            .strip_suffix(":")
            .unwrap()
            .chars()
            .next()
            .unwrap();

        let password = String::from(components[2]);

        Password {
            // We need to add one as Range exludes upper bound.
            range: bounds[0]..(bounds[1] + 1),
            char,
            password,
        }
    }

    fn is_valid(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|char| *char == self.char)
            .count();

        if self.range.contains(&(count as u32)) {
            true
        } else {
            false
        }
    }

    // Part 2.
    fn is_valid_revisited(&self) -> bool {
        let chars = self.password.chars();

        let at_first = chars
            .clone()
            .nth(self.range.start as usize - 1)
            .contains(&self.char);

        let at_second = chars
            .clone()
            .nth(self.range.end as usize - 2)
            .contains(&self.char);

        at_first ^ at_second
    }
}

fn count_valid_passwords(passwords: Vec<Password>) -> usize {
    let sum = passwords.iter().fold(0, |acc, password| {
        if password.is_valid_revisited() {
            acc + 1
        } else {
            acc
        }
    });

    sum
}

#[cfg(test)]
mod tests {

    use super::*;

    // Example input:
    //
    // 1-3 a: abcde
    // 1-3 b: cdefg
    // 2-9 c: ccccccccc

    #[test]
    fn password_from_str() {
        let str = "1-3 a: abcde";

        assert_eq!(
            Password::from_str(str),
            Password {
                range: 1..4,
                char: 'a',
                password: String::from("abcde")
            }
        );
    }

    #[test]
    fn password_is_valid() {
        let password = Password {
            range: 1..4,
            char: 'a',
            password: String::from("abcde"),
        };

        assert_eq!(password.is_valid(), true);
    }

    #[test]
    fn password_is_invalid() {
        let password = Password {
            range: 1..4,
            char: 'b',
            password: String::from("cdef"),
        };

        assert_eq!(password.is_valid(), false);
    }

    #[test]
    fn password_is_valid_revisited() {
        let password = Password {
            range: 1..4,
            char: 'a',
            password: String::from("abcde"),
        };

        assert_eq!(password.is_valid_revisited(), true);
    }

    #[test]
    fn counts_number_of_valid_passwords() {
        let passwords = vec![
            Password {
                range: 2..4,
                char: 'a',
                password: String::from("abcde"),
            },
            Password {
                range: 1..3,
                char: 'e',
                password: String::from("hello"),
            },
            Password {
                range: 2..9,
                char: 'c',
                password: String::from("ccccccc"),
            },
        ];

        assert_eq!(count_valid_passwords(passwords), 2);
    }
}
