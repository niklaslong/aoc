type Memory = Vec<u64>;

fn main() {
    // STEPS:
    // 1. read stdin to vec (memory)
    // 2. opcode calc

    let mut input = String::new();
    let stdin = std::io::stdin();

    use std::io::BufRead;
    stdin.lock().read_line(&mut input).unwrap();

    // This will panic if input ends with anything but a number.
    use std::str::FromStr;
    let mut memory: Memory = input
        .trim_end()
        .split(',')
        .map(|val| u64::from_str(val).unwrap())
        .collect();

    // Run the program based on opcodes (every 4 starting at i = 0):
    // 1 => addition,
    // 2 => multiplication,
    // 99 => exit.
    for i in (0..(memory.len() - 1)).step_by(4) {
        let mem_1 = memory[i + 1] as usize;
        let mem_2 = memory[i + 2] as usize;

        let res = match memory[i] {
            1 => memory[mem_1] + memory[mem_2],
            2 => memory[mem_1] * memory[mem_2],
            99 => break,
            _ => panic!(),
        };

        let write_address = memory[i + 3] as usize;
        memory[write_address] = res;
    }
    println!("{:?}", memory);
}

