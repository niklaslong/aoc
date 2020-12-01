fn main() {
    // STEPS:
    // 1. read stdin to vec (memory)
    // 2. opcode calc

    let mut input = String::new();
    let stdin = std::io::stdin();

    use std::io::BufRead;
    stdin.lock().read_line(&mut input).unwrap();

    // This will panic if input ends with anything but a number.
    let mut memory = Memory::from_str(&input).unwrap();

    // Runs the computation based on the opcodes.
    memory.compute();

    println!("{:?}", memory.0[0]);
}

#[derive(Debug, Clone)]
struct Memory(Vec<u64>);

use std::num::ParseIntError;
use std::str::FromStr;

impl FromStr for Memory {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let memory: Vec<u64> = s
            .trim_end()
            .split(',')
            .map(|val| u64::from_str(val).unwrap())
            .collect();

        Ok(Memory(memory))
    }
}

impl Memory {
    // Using an out parameter here, Vec.push uses this as well. For more information see
    // https://steveklabnik.com/writing/are-out-parameters-idiomatic-in-rust.
    fn compute(&mut self) {
        // Run the program based on opcodes (every 4 starting at i = 0):
        // 1 => addition,
        // 2 => multiplication,
        // 99 => exit.
        for i in (0..(self.0.len() - 1)).step_by(4) {
            let mem_1 = self.0[i + 1] as usize;
            let mem_2 = self.0[i + 2] as usize;
            let val_1 = self.0[mem_1];
            let val_2 = self.0[mem_2];

            let res = match self.0[i] {
                1 => val_1 + val_2,
                2 => val_1 * val_2,
                99 => break,
                _ => panic!(),
            };

            let write_address = self.0[i + 3] as usize;
            self.0[write_address] = res;
        }
    }

    fn noun_and_verb(&self, output: u64) -> (u64, u64) {
        let mut noun_and_verb = (0u64, 0u64);

        'outer: for i in 0..99 {
            for j in 0..99 {
                let mut memory = self.clone();

                noun_and_verb = (i, j);

                memory.0[1] = i;
                memory.0[2] = j;

                memory.compute();

                if memory.0[0] == output {
                    break 'outer;
                };
            }
        }

        noun_and_verb
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_from_input() {
        let input = String::from("1,1,1,4,99,5,6,0,99");
        let memory = Memory::from_str(&input).unwrap();

        assert_eq!(memory.0, vec![1, 1, 1, 4, 99, 5, 6, 0, 99])
    }

    #[test]
    fn calculates_new_state() {
        let mut memory = Memory(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        memory.compute();

        assert_eq!(memory.0, vec![30, 1, 1, 4, 2, 5, 6, 0, 99])
    }

    #[test]
    fn finds_noun_and_verb_for_output() {
        let output: u64 = 19690720;
        let memory = Memory(vec![
            1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 9, 19, 1, 5, 19, 23, 1, 6, 23,
            27, 1, 27, 10, 31, 1, 31, 5, 35, 2, 10, 35, 39, 1, 9, 39, 43, 1, 43, 5, 47, 1, 47, 6,
            51, 2, 51, 6, 55, 1, 13, 55, 59, 2, 6, 59, 63, 1, 63, 5, 67, 2, 10, 67, 71, 1, 9, 71,
            75, 1, 75, 13, 79, 1, 10, 79, 83, 2, 83, 13, 87, 1, 87, 6, 91, 1, 5, 91, 95, 2, 95, 9,
            99, 1, 5, 99, 103, 1, 103, 6, 107, 2, 107, 13, 111, 1, 111, 10, 115, 2, 10, 115, 119,
            1, 9, 119, 123, 1, 123, 9, 127, 1, 13, 127, 131, 2, 10, 131, 135, 1, 135, 5, 139, 1, 2,
            139, 143, 1, 143, 5, 0, 99, 2, 0, 14, 0,
        ]);

        let (noun, verb) = memory.noun_and_verb(output);

        // These are the correct values.
        assert_eq!(noun, 82);
        assert_eq!(verb, 50);
    }
}
