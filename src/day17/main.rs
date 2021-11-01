use input;

use std::collections::HashMap;
use std::io;

fn main() -> io::Result<()> {
    let _contents = input::load_file_by_lines("src/day17/input.txt")?;

    Ok(())
}

type Coordinate = (isize, isize, isize);

#[derive(Debug)]
struct PocketDimension {
    cubes: HashMap<Coordinate, Cube>,
}

impl PocketDimension {
    pub fn boot(&mut self) {
        (0..BOOT_CYCLES).for_each(|_| self.cycle());
    }

    fn active_cubes(&self) -> usize {
        self.cubes
            .values()
            .into_iter()
            .filter(|&c| *c == Cube::Active)
            .count()
    }

    /// If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
    /// If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.
    pub fn cycle(&mut self) {
        let mut cubes = self.cubes.clone();

        for coordinate in self.cubes.keys() {
            let num_neighbors = self.num_active_neighbors(*coordinate);
            cubes.insert(
                *coordinate,
                match self.get(*coordinate) {
                    Cube::Active => {
                        if num_neighbors == 2 || num_neighbors == 3 {
                            Cube::Active
                        } else {
                            Cube::Inactive
                        }
                    }
                    Cube::Inactive => {
                        if num_neighbors == 3 {
                            Cube::Active
                        } else {
                            Cube::Inactive
                        }
                    }
                },
            );
        }

        self.cubes = cubes;
    }

    fn num_active_neighbors(&self, coordinate: Coordinate) -> usize {
        let (xi, yi, zi) = coordinate;
        let mut num_active = 0;

        for x in xi - 1..xi + 1 {
            for y in yi - 1..yi + 1 {
                for z in zi - 1..zi + 1 {
                    if x == xi && x == xi && z == zi {
                        continue;
                    }
                    if self.get((x, y, z)) == Cube::Active {
                        num_active += 1;
                    }
                }
            }
        }

        num_active
    }

    pub fn get(&self, coordinate: Coordinate) -> Cube {
        *self.cubes.get(&coordinate).unwrap_or(&Cube::Inactive)
    }
}

const BOOT_CYCLES: usize = 6;

impl From<&str> for PocketDimension {
    fn from(s: &str) -> Self {
        let mut cubes = HashMap::new();
        let (mut x, mut y, z) = (0, 0, 0);

        for line in s.lines() {
            for c in line.chars() {
                cubes.insert((x, y, z), Cube::from(c));
                x += 1;
            }
            y += 1;
            x = 0;
        }

        Self { cubes }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Cube {
    Active,
    Inactive,
}

impl From<char> for Cube {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Active,
            '.' => Self::Inactive,
            c => panic!("{} is not a valid cube.", c),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube_from() {
        assert_eq!(Cube::from('#'), Cube::Active);
        assert_eq!(Cube::from('.'), Cube::Inactive);
    }

    #[test]
    #[should_panic]
    fn test_cube_from_panic() {
        Cube::from(' ');
    }

    #[test]
    fn test_pocket_dimension() {
        let pocket_dimension = PocketDimension::from(".#");
        assert_eq!(pocket_dimension.get((0, 0, 0)), Cube::Inactive);
        assert_eq!(pocket_dimension.get((1, 0, 0)), Cube::Active);
    }

    #[test]
    fn test_pocket_dimension_active_cubes() {
        let pocket_dimension = PocketDimension::from("##.##");
        assert_eq!(pocket_dimension.active_cubes(), 4);
    }

    #[test]
    fn test_pocket_dimension_num_active_neighbors() {
        let pocket_dimension = PocketDimension::from("##.##");
        assert_eq!(pocket_dimension.num_active_neighbors((0, 0, 0)), 1);
    }

    #[test]
    fn test_part1() {
        let mut pocket_dimension = PocketDimension::from(
            ".#.
..#
###",
        );

        pocket_dimension.boot();

        assert_eq!(pocket_dimension.active_cubes(), 112);
    }
}
