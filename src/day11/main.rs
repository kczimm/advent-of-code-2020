use input;
use std::io::Result;

fn main() -> Result<()> {
    let content = input::load_file("src/day11/input.txt")?;

    let mut layout = Layout::from(&content);
    println!("part1: {}", layout.simulate());

    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Position {
    Floor,
    Empty,
    Occupied,
}

impl Position {
    pub fn from(c: char) -> Self {
        match c {
            '.' => Self::Floor,
            'L' => Self::Empty,
            '#' => Self::Occupied,
            c => unreachable!("bad position: {}", c),
        }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Floor => '.',
                Self::Empty => 'L',
                Self::Occupied => '#',
            }
        )
    }
}

pub struct Layout {
    layout: Vec<Position>,
    dims: (usize, usize),
}

impl std::fmt::Display for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, p) in self.layout.iter().enumerate() {
            if i % self.dims.1 == 0 && i > 0 {
                write!(f, "\n")?;
            }
            write!(f, "{}", *p)?;
        }

        Ok(())
    }
}

impl Layout {
    pub fn from(text: &str) -> Self {
        let layout: Vec<Position> = text
            .chars()
            .filter(|c| *c != '\n')
            .map(|c| Position::from(c))
            .collect();

        let columns = text.lines().next().expect("no newlines").len();
        let rows = layout.len() / columns;

        Self {
            layout,
            dims: (rows, columns),
        }
    }

    pub fn simulate(&mut self) -> usize {
        let mut seat_changes = 1; // set to anything other than 0.

        // run rounds until the number of seat changes is zero.
        while seat_changes != 0 {
            seat_changes = self.do_round();
        }

        // count the number of occupied seats when the layout stabilizes.
        self.layout
            .iter()
            .filter(|p| match p {
                Position::Occupied => true,
                _ => false,
            })
            .count()
    }

    fn do_round(&mut self) -> usize {
        let mut num_changes = 0;

        // clone the layout to record the seat changes.
        // if we mutated the layout, the "changes" wouldn't be simultaneous.
        let mut new_layout = self.layout.clone();

        self.layout.iter().enumerate().for_each(|(i, p)| {
            let neighbors = self.num_neighbors_occupied(i);
            if let Some(p) = Layout::rules(*p, neighbors) {
                new_layout[i] = p;
                num_changes += 1;
            }
        });

        self.layout = new_layout;

        num_changes
    }

    /// implement the seat change rules applied simultaneously
    pub fn rules(p: Position, neighbors: usize) -> Option<Position> {
        match p {
            Position::Empty if neighbors == 0 => Some(Position::Occupied),
            Position::Occupied if neighbors >= 4 => Some(Position::Empty),
            _ => None,
        }
    }

    fn num_neighbors_occupied(&self, i: usize) -> usize {
        let x = (i % self.dims.1) as isize;
        let y = (i / self.dims.0) as isize;
        let mut neighbors = 0;

        for dx in -1..=1 {
            for dy in -1..=1 {
                let nx = x + dx;
                let ny = y + dy;
                if nx < 0 || ny < 0 || (dx == 0 && dy == 0) || nx as usize >= self.dims.1 {
                    continue;
                }
                let nx = nx as usize; // should be positive after ^ conditional
                let ny = ny as usize; // should be positive after ^ conditional
                let i = self.dims.1 * ny + nx;
                if let Some(p) = self.layout.get(i) {
                    match p {
                        Position::Occupied => neighbors += 1,
                        _ => {}
                    }
                }
            }
        }

        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_neighbors_occupied() {
        let text = "#.LL.L#.##\n#LLLLLL.L#\nL.L.L..L..\n#LLL.LL.L#\n#.LL.LL.LL\n#.LLLL#.##\n..L.L.....\n#LLLLLLLL#\n#.LLLLLL.L\n#.#LLLL.##";

        let layout = Layout::from(text);

        assert_eq!(layout.num_neighbors_occupied(0), 1);
        assert_eq!(layout.num_neighbors_occupied(6), 0);
        assert_eq!(layout.num_neighbors_occupied(9), 2);
    }

    #[test]
    fn test_rules() {
        // If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
        assert_eq!(Layout::rules(Position::Empty, 0), Some(Position::Occupied));
        assert_eq!(Layout::rules(Position::Empty, 1), None);

        // If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
        assert_eq!(Layout::rules(Position::Occupied, 0), None);
        assert_eq!(Layout::rules(Position::Occupied, 3), None);
        assert_eq!(Layout::rules(Position::Occupied, 4), Some(Position::Empty));
        assert_eq!(Layout::rules(Position::Occupied, 5), Some(Position::Empty));

        // Otherwise, the seat's state does not change.
        assert_eq!(Layout::rules(Position::Floor, 0), None);
        assert_eq!(Layout::rules(Position::Floor, 5), None);
    }

    #[test]
    fn test_layout_display() {
        let text = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL";

        let layout = Layout::from(&text);

        assert_eq!(text, format!("{}", layout));
    }

    #[test]
    fn test_part1() {
        let text = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL";

        let mut layout = Layout::from(&text);

        assert_eq!(layout.simulate(), 37);
    }

    #[test]
    fn test_round1() {
        let text = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL";
        let round1 = "#.##.##.##\n#######.##\n#.#.#..#..\n####.##.##\n#.##.##.##\n#.#####.##\n..#.#.....\n##########\n#.######.#\n#.#####.##";

        let mut layout = Layout::from(&text);
        layout.do_round();

        assert_eq!(round1, format!("{}", layout));
    }

    #[test]
    fn test_round2() {
        let text = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL";
        let round2 = "#.LL.L#.##\n#LLLLLL.L#\nL.L.L..L..\n#LLL.LL.L#\n#.LL.LL.LL\n#.LLLL#.##\n..L.L.....\n#LLLLLLLL#\n#.LLLLLL.L\n#.#LLLL.##";

        let mut layout = Layout::from(&text);
        layout.do_round();
        layout.do_round();

        assert_eq!(round2, format!("{}", layout));
    }

    #[test]
    fn test_round3() {
        let text = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL";
        let round3 = "#.##.L#.##\n#L###LL.L#\nL.#.#..#..\n#L##.##.L#\n#.##.LL.LL\n#.###L#.##\n..#.#.....\n#L######L#\n#.LL###L.L\n#.#L###.##";

        let mut layout = Layout::from(&text);
        layout.do_round();
        layout.do_round();
        layout.do_round();

        assert_eq!(round3, format!("{}", layout));
    }
}
