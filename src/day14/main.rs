use input;

use std::collections::HashMap;
use std::io;

fn main() -> io::Result<()> {
    let contents = input::load_file("src/day14/input.txt")?;

    let program: Program = contents.lines().map(|l| Instruction::from(l)).collect();
    let mut computer = ComputerSystem::new();
    program.iter().for_each(|i| computer.execute(i));

    println!("part1: {}", computer.sum_memory());

    Ok(())
}

type Program<'a> = Vec<Instruction<'a>>;
type Address = usize;
type Value = u64;

struct ComputerSystem<'a> {
    bitmask: Option<&'a BitMask<'a>>,
    memory: HashMap<Address, Value>,
}

impl<'a> ComputerSystem<'a> {
    fn new() -> Self {
        Self {
            bitmask: None,
            memory: HashMap::new(),
        }
    }

    fn execute(&mut self, instruction: &'a Instruction<'a>) {
        match instruction {
            Instruction::BitMask(mask) => {
                self.bitmask = Some(mask);
            }
            Instruction::MemWrite(address, value) => match &self.bitmask {
                Some(mask) => {
                    self.memory.insert(*address, mask.apply(*value));
                }
                None => {
                    self.memory.insert(*address, *value);
                }
            },
        }
    }

    fn sum_memory(&self) -> u64 {
        self.memory.values().sum()
    }
}

#[derive(Debug, PartialEq)]
enum Instruction<'a> {
    BitMask(BitMask<'a>),
    MemWrite(Address, Value),
}

impl<'a> Instruction<'a> {
    fn from(s: &'a str) -> Self {
        let mut parts = s.split(" = ");
        let first = parts.next().expect("no first part of instruction");
        let second = parts.next().expect("no second part of instruction");
        if first == "mask" {
            Self::BitMask(BitMask::from(second))
        } else {
            let i = first
                .find('[')
                .expect("no bracket in memory write instruction");
            let value = second.parse().expect("parse of value failed");
            let address = (&first[i + 1..first.len() - 1])
                .parse()
                .expect("parse of address failed");
            Self::MemWrite(address, value)
        }
    }
}

const BITMASK_LEN: usize = 36;

#[derive(Debug, PartialEq)]
struct BitMask<'a> {
    mask: &'a str,
}

impl<'a> BitMask<'a> {
    fn from(s: &'a str) -> Self {
        if s.len() != BITMASK_LEN {
            panic!("wrong length bitmask");
        }
        Self { mask: s }
    }

    /// a 0 or 1 overwrites the corresponding bit in the value, while an X leaves the bit in the value unchanged.
    fn apply(&self, value: u64) -> u64 {
        let mut output = value;
        for (i, c) in self.mask.chars().enumerate() {
            match c {
                '0' => {
                    output &= !(1 << (BITMASK_LEN - i - 1));
                }
                '1' => {
                    output |= 1 << (BITMASK_LEN - i - 1);
                }
                _ => {}
            }
        }
        output
    }

    fn apply_v2(&self, value: u64) -> Vec<u64> {
        let mut output = value;
        let mut outputs = Vec::new();
        // overwrite with 1 first
        for (i, c) in self.mask.chars().enumerate() {
            match c {
                '1' => {
                    output |= 1 << (BITMASK_LEN - i - 1);
                }
                _ => {}
            }
        }

        // then find X's and output both options
        for (i, c) in self.mask.chars().enumerate() {
            match c {
                'X' => {
                    let set = output | 1 << (BITMASK_LEN - i - 1);
                    let clear = output & !(1 << (BITMASK_LEN - i - 1));
                    outputs.push(set);
                    outputs.push(clear);
                }
                _ => {}
            }
        }
        outputs
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_instruction_from() {
        assert_eq!(
            Instruction::from("mask = X111000X0101100001000000100011X0000X"),
            Instruction::BitMask(BitMask::from("X111000X0101100001000000100011X0000X"))
        );

        assert_eq!(
            Instruction::from("mem[4812] = 133322396"),
            Instruction::MemWrite(4812, 133322396)
        );
    }

    #[test]
    fn test_bitmask_apply() {
        let bitmask = BitMask::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!(bitmask.apply(11), 73);
        assert_eq!(bitmask.apply(101), 101);
        assert_eq!(bitmask.apply(0), 64);
    }

    #[test]
    fn test_bitmask_apply_v2() {
        let bitmask = BitMask::from("000000000000000000000000000000X1001X");
        assert_eq!(bitmask.apply_v2(42), vec![26, 27, 58, 59]);
    }
}
