use std::fs;

fn main() {
    // Read input file.
    let input_bytes = fs::read("input.txt").unwrap();
    let instructions: Vec<Instruction> = String::from_utf8_lossy(&input_bytes)
        .split('\n')
        .filter(|substr| !substr.is_empty())
        .map(|substr| {
            let (instruction, val) = substr.split_once(' ').unwrap();
            let val = val.parse().unwrap();

            match instruction {
                "up" => Instruction::Up(val),
                "down" => Instruction::Down(val),
                "forward" => Instruction::Forward(val),
                _ => panic!("This instruction is not supported"),
            }
        })
        .collect();

    let (pos_without_aim, pos_with_aim) = Position::final_from_instructions(instructions);

    dbg!(pos_without_aim.x * pos_without_aim.y);
    dbg!(pos_with_aim.x * pos_with_aim.y);
}

#[derive(Copy, Clone)]
enum Instruction {
    Up(i64),
    Down(i64),
    Forward(i64),
}

struct Position {
    x: i64,
    y: i64,
    aim: i64,
}

impl Position {
    fn new() -> Self {
        Position { x: 0, y: 0, aim: 0 }
    }

    fn update_without_aim(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Up(val) => self.y -= val,
            Instruction::Down(val) => self.y += val,
            Instruction::Forward(val) => self.x += val,
        }
    }

    fn update_with_aim(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Up(val) => self.aim -= val,
            Instruction::Down(val) => self.aim += val,
            Instruction::Forward(val) => {
                self.x += val;
                self.y += self.aim * val
            }
        }
    }

    fn final_from_instructions(instructions: Vec<Instruction>) -> (Self, Self) {
        let mut pos_without_aim = Position::new();
        let mut pos_with_aim = Position::new();

        for instruction in instructions {
            pos_without_aim.update_without_aim(instruction);
            pos_with_aim.update_with_aim(instruction);
        }

        (pos_without_aim, pos_with_aim)
    }
}
