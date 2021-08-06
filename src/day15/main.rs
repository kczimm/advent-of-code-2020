use input;

use std::collections::HashMap;
use std::io;

fn main() -> io::Result<()> {
    let contents = input::load_file_by_lines("src/day15/input.txt")?;

    let mut game = MemoryGame::from(contents.iter().nth(0).unwrap().as_ref());

    game.take_turns(2020);

    println!("part1: {}", game.last_number());

    game.take_turns(30_000_000);

    println!("part2: {}", game.last_number());

    Ok(())
}

#[derive(Debug)]
struct MemoryGame {
    numbers: Vec<usize>,
    last: HashMap<usize, usize>,
}

impl MemoryGame {
    fn do_turn(&mut self) {
        let previous_turn = self.numbers.len();
        let last_number = self.numbers.last().expect("no numbers in the memory game");
        // have we seen the last number before?
        let next = match self.last.get(last_number) {
            Some(i) => previous_turn - i,
            None => 0,
        };
        self.last.insert(*last_number, previous_turn);
        self.numbers.push(next);
    }

    fn take_turns(&mut self, num_turns: usize) {
        (self.numbers.len()..num_turns).for_each(|_| self.do_turn());
    }

    fn last_number(&self) -> usize {
        *self.numbers.last().expect("game has no numbers")
    }
}

impl From<&str> for MemoryGame {
    fn from(s: &str) -> Self {
        let numbers: Vec<usize> = s
            .split(",")
            .map(|x| x.parse::<usize>().expect("failed to parse a number"))
            .collect();

        let mut last = HashMap::new();
        // record indices for all but the last number.
        (0..numbers.len() - 1).for_each(|i| {
            last.insert(numbers[i], i + 1);
        });

        Self { numbers, last }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memorygame_from() {
        let game = MemoryGame::from("0,3,6");

        assert_eq!(game.numbers, vec![0, 3, 6]);
    }

    #[test]
    fn test_memorygame_doturn() {
        let mut game = MemoryGame::from("0,3,6");

        game.do_turn();

        assert_eq!(*game.numbers.last().unwrap(), 0);

        game.do_turn();

        assert_eq!(*game.numbers.last().unwrap(), 3);

        game.do_turn();

        assert_eq!(*game.numbers.last().unwrap(), 3);
    }

    #[test]
    fn test_memorygame_taketurns() {
        let mut game = MemoryGame::from("0,3,6");

        game.take_turns(10);

        assert_eq!(*game.numbers.last().unwrap(), 0);
    }

    #[test]
    fn test_memorygame_2020_number() {
        let mut game = MemoryGame::from("0,3,6");

        game.take_turns(2020);

        assert_eq!(*game.numbers.last().unwrap(), 436);
    }
}
