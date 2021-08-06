use input;

use std::io;

fn main() -> io::Result<()> {
    let contents = input::load_file_by_lines("src/day15/input.txt")?;

    let mut game = MemoryGame::from(contents.iter().nth(0).unwrap().as_ref());

    game.take_turns(2020);

    println!("part1: {}", *game.numbers.last().unwrap());

    Ok(())
}

#[derive(Debug)]
struct MemoryGame {
    numbers: Vec<usize>,
}

impl MemoryGame {
    fn do_turn(&mut self) {
        let previous_turn = self.numbers.len();
        let last = self.numbers.last().expect("no numbers in the memory game");
        // have we seen the last number before?
        let mut indices: Vec<usize> = self
            .numbers
            .iter()
            .enumerate()
            .filter_map(|(i, x)| if *x == *last { Some(i + 1) } else { None })
            .collect();

        indices.pop(); // we don't want to include the last one.

        if indices.is_empty() {
            self.numbers.push(0);
        } else {
            // unwrap is OK because we know it is not empty.
            self.numbers.push(previous_turn - indices.last().unwrap());
        }
    }

    fn take_turns(&mut self, num_turns: usize) {
        (self.numbers.len()..num_turns).for_each(|_| self.do_turn());
    }
}

impl From<&str> for MemoryGame {
    fn from(s: &str) -> Self {
        let numbers = s
            .split(",")
            .map(|x| x.parse::<usize>().expect("failed to parse a number"))
            .collect();

        Self { numbers }
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
