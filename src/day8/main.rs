use input;
use std::io::Result;

fn main() -> Result<()> {
    let content = input::load_file("src/day8/input.txt")?;

    let mut console = HandheldGameConsole::from(&content);
    println!("part1: {}", accumulator_before_inf_loop(&mut console));

    println!("part2: {}", accumulator_after_instruction_fix(&mut console));

    Ok(())
}

#[derive(Clone, Copy)]
pub enum Operation {
    Acc,
    Jmp,
    Nop,
}

pub type Argument = isize;

pub struct Instruction(Operation, Argument);

impl Instruction {
    pub fn from(line: &str) -> Self {
        let mut parts = line.split(' ');
        let op = parts.next().expect("no operation in line");
        let arg = parts.next().expect("no argument in line");
        let arg = arg.parse().expect("argument parse failed");
        Self(
            match op {
                "acc" => Operation::Acc,
                "jmp" => Operation::Jmp,
                "nop" => Operation::Nop,
                _ => unreachable!("bad operation"),
            },
            arg,
        )
    }
}

pub struct HandheldGameConsole {
    program_counter: usize,
    accumulator: isize,
    instructions: Vec<(bool, Instruction)>,
}

#[derive(Debug)]
pub enum ConsoleError {
    InfLoop,
    EndOfInstructions,
}

impl HandheldGameConsole {
    pub fn from(text: &str) -> Self {
        Self {
            program_counter: 0,
            accumulator: 0,
            instructions: text
                .lines()
                .map(|line| (false, Instruction::from(line)))
                .collect(),
        }
    }

    fn reset(&mut self) {
        self.program_counter = 0;
        self.accumulator = 0;
        self.instructions.iter_mut().for_each(|(e, _)| *e = false);
    }

    fn step(&mut self) -> std::result::Result<(), ConsoleError> {
        let (has_executed, instruction) = match self.instructions.get_mut(self.program_counter) {
            Some(instruction) => instruction,
            None => return Err(ConsoleError::EndOfInstructions),
        };

        if *has_executed {
            return Err(ConsoleError::InfLoop);
        } else {
            *has_executed = true;
        }

        match instruction.0 {
            Operation::Acc => {
                self.accumulator += instruction.1;
                self.program_counter += 1;
            }
            Operation::Jmp => {
                self.program_counter = (self.program_counter as isize + instruction.1) as usize
            }
            Operation::Nop => self.program_counter += 1,
        }

        Ok(())
    }

    fn flip_operation(&mut self, i: usize) {
        let (_, instruction) = self.instructions.get_mut(i).expect("bad instruction index");

        instruction.0 = match instruction.0 {
            Operation::Acc => Operation::Acc,
            Operation::Nop => Operation::Jmp,
            Operation::Jmp => Operation::Nop,
        };
    }
}

pub fn accumulator_before_inf_loop(console: &mut HandheldGameConsole) -> isize {
    loop {
        if let Err(e) = console.step() {
            match e {
                ConsoleError::InfLoop => break,
                _ => {}
            }
        }
    }

    console.accumulator
}

pub fn accumulator_after_instruction_fix(console: &mut HandheldGameConsole) -> isize {
    let num_instructions = console.instructions.len();
    for i in 0..num_instructions {
        match console
            .instructions
            .get(i)
            .expect("bad instruction index")
            .1
        {
            Instruction(Operation::Acc, _) => continue,
            _ => {}
        }

        console.reset();

        console.flip_operation(i);

        // test
        loop {
            if let Err(e) = console.step() {
                match e {
                    ConsoleError::EndOfInstructions => return console.accumulator,
                    ConsoleError::InfLoop => {
                        break;
                    }
                }
            }
        }

        // must not have been the right op.
        // flip it back
        console.flip_operation(i);
    }

    unreachable!("we never found the op");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let text = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        let mut console = HandheldGameConsole::from(&text);

        assert_eq!(accumulator_before_inf_loop(&mut console), 5);
    }

    #[test]
    fn test_part2() {
        let text = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        let mut console = HandheldGameConsole::from(&text);

        assert_eq!(accumulator_after_instruction_fix(&mut console), 8);
    }
}
