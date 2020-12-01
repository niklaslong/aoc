use std::io::BufRead;
use std::str::FromStr;

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let expense_report: Vec<u64> = lines
        .take_while(|line| !line.as_ref().unwrap().is_empty())
        .map(|line| u64::from_str(&line.unwrap()).unwrap())
        .collect();

    let (a, b) = find_pair(expense_report);

    println!("Ouput: {}", a * b);
}

fn find_pair(v: Vec<u64>) -> (u64, u64) {
    let mut a = 0u64;
    let mut b = 0u64;

    'outer: for i in &v {
        for j in &v {
            a = *i;
            b = *j;

            if a + b == 2020 {
                break 'outer;
            }
        }
    }

    (a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_numbers() {
        let expense_report = vec![1721, 979, 366, 299, 675, 1456];

        let (a, b) = find_pair(expense_report);

        assert_eq!(a * b, 514579);
    }
}
