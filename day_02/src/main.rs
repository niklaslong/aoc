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

    println!("{:?}", memory);
}

#[derive(Debug)]
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
}
