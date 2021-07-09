use input;
use std::io::Result;

fn main() -> Result<()> {
    let content = input::load_file("src/day12/input.txt")?;

    let mut ship = Ship::from(&content);
    ship.execute_instructions();
    println!("part1: {}", ship.manhatten_distance());

    Ok(())
}

pub type Value = isize;

pub type Degrees = isize;

/// Action N means to move north by the given value.
/// Action S means to move south by the given value.
/// Action E means to move east by the given value.
/// Action W means to move west by the given value.
/// Action L means to turn left the given number of degrees.
/// Action R means to turn right the given number of degrees.
/// Action F means to move forward by the given value in the direction the ship is currently facing.

#[derive(Clone, Copy)]
pub enum Action {
    N(Value),
    S(Value),
    E(Value),
    W(Value),
    L(Degrees),
    R(Degrees),
    F(Value),
}

impl Action {
    pub fn from(s: &str) -> Self {
        let v = (&s[1..]).parse::<Value>().expect("failed to parse value");

        // unwrap can't fail if the parse above succeeded.
        match s.chars().next().unwrap() {
            'N' => Self::N(v),
            'S' => Self::S(v),
            'E' => Self::E(v),
            'W' => Self::W(v),
            'L' => Self::L(v),
            'R' => Self::R(v),
            'F' => Self::F(v),
            _ => panic!("bad action"),
        }
    }
}

type NavigationInstructions = Vec<Action>;

pub struct Ship {
    instructions: NavigationInstructions,
    direction: Degrees,
    position: (isize, isize),
}

impl Ship {
    pub fn no_instructions() -> Self {
        Self {
            instructions: Vec::new(),
            direction: 0,
            position: (0, 0),
        }
    }

    pub fn from(s: &str) -> Self {
        Self {
            instructions: s.lines().map(|line| Action::from(line)).collect(),
            direction: 0,
            position: (0, 0),
        }
    }

    pub fn execute_instructions(&mut self) {
        (0..self.instructions.len())
            .for_each(|i| self.do_action(*self.instructions.get(i).unwrap()));
    }

    fn do_action(&mut self, a: Action) {
        match a {
            Action::N(v) => self.position.1 += v,
            Action::S(v) => self.position.1 -= v,
            Action::E(v) => self.position.0 += v,
            Action::W(v) => self.position.0 -= v,
            Action::L(d) => self.direction = (self.direction + d) % 360,
            Action::R(d) => self.direction = (self.direction - d + 360) % 360,
            Action::F(v) => match self.direction {
                0 => self.position.0 += v,
                90 => self.position.1 += v,
                180 => self.position.0 -= v,
                270 => self.position.1 -= v,
                d => panic!("bad direction: {}", d),
            },
        }
    }

    pub fn manhatten_distance(&self) -> usize {
        (self.position.0.abs() + self.position.1.abs()) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_action() {
        let mut ship = Ship::no_instructions();

        // F10
        ship.do_action(Action::F(10));
        assert_eq!(ship.position, (10, 0));
        assert_eq!(ship.direction, 0);

        // N3
        ship.do_action(Action::N(3));
        assert_eq!(ship.position, (10, 3));
        assert_eq!(ship.direction, 0);

        // F7
        ship.do_action(Action::F(7));
        assert_eq!(ship.position, (17, 3));
        assert_eq!(ship.direction, 0);

        // R90
        ship.do_action(Action::R(90));
        assert_eq!(ship.position, (17, 3));
        assert_eq!(ship.direction, 270);

        // F11
        ship.do_action(Action::F(11));
        assert_eq!(ship.position, (17, -8));
        assert_eq!(ship.direction, 270);
    }

    #[test]
    fn test_part1() {
        let s = "F10\nN3\nF7\nR90\nF11";
        let mut ship = Ship::from(s);
        ship.execute_instructions();
        assert_eq!(ship.manhatten_distance(), 25);
    }
}
