use input;
use std::io::Result;
fn main() -> Result<()> {
    let lines = input::load_file_by_lines("src/day5/input.txt")?;

    println!("part1: {}", part1(&lines));

    println!("part2: {}", part2(&lines));

    Ok(())
}

pub fn part1(lines: &Vec<String>) -> usize {
    let mut seat = Seat::default();
    lines
        .iter()
        .map(|l| {
            seat.from_mut(l);
            seat.seat_id()
        })
        .max()
        .expect("iterator is empty")
}

pub fn part2(lines: &Vec<String>) -> usize {
    let mut seat = Seat::default();
    let mut ids: Vec<usize> = lines
        .iter()
        .map(|l| {
            seat.from_mut(l);
            seat.seat_id()
        })
        .collect();
    ids.sort();
    let mut myid = *ids.get(0).unwrap();
    for id in &ids {
        if *id == myid + 2 {
            break;
        } else {
            myid = *id;
        }
    }

    myid + 1
}
pub struct Seat {
    partitions: [BinaryPartition; 10],
}

impl Seat {
    fn default() -> Self {
        Self {
            partitions: [BinaryPartition::Front; 10],
        }
    }

    fn from(line: &str) -> Self {
        let mut s = Self::default();
        s.from_mut(line);
        s
    }

    fn from_mut(&mut self, line: &str) {
        line.char_indices().for_each(|(i, c)| {
            self.partitions[i] = match c {
                'F' => BinaryPartition::Front,
                'B' => BinaryPartition::Back,
                'L' => BinaryPartition::Left,
                'R' => BinaryPartition::Right,
                _ => panic!("bad binary partition: {}", c),
            }
        });
    }

    fn column(&self) -> usize {
        let mut start = 0;
        let mut end = 7;
        for i in 7..10 {
            match self.partitions[i] {
                BinaryPartition::Left => end = (start + end) / 2,
                BinaryPartition::Right => start = (start + end) / 2 + 1,
                p => panic!("bad partition in last three: {:?}", p),
            }
        }
        end
    }

    fn row(&self) -> usize {
        let mut start = 0;
        let mut end = 127;
        for i in 0..7 {
            match self.partitions[i] {
                BinaryPartition::Front => end = (start + end) / 2,
                BinaryPartition::Back => start = (start + end) / 2 + 1,
                p => panic!("bad partition in first seven: {:?}", p),
            }
        }
        start
    }

    fn seat_id(&self) -> usize {
        self.row() * 8 + self.column()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BinaryPartition {
    Front,
    Back,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let line = Seat::from("FBFBBFFRLR");
        assert_eq!(line.row(), 44);
        assert_eq!(line.column(), 5);
        assert_eq!(line.seat_id(), 357);
    }
}
