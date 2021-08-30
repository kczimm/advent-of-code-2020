use input;
use std::io::Result;

fn main() -> Result<()> {
    let data = input::load_file("src/day3/input.txt")?;

    let map = Map::from(data.as_ref());
    let toboggan = Toboggan::new(3, 1);

    println!("part1: {}", map.num_trees_traversed(&toboggan));

    let toboggans = [
        Toboggan::new(1, 1),
        Toboggan::new(3, 1),
        Toboggan::new(5, 1),
        Toboggan::new(7, 1),
        Toboggan::new(1, 2),
    ];

    println!("part2: {}", map.product_of_trees_from_toboggans(&toboggans));

    Ok(())
}

#[derive(Debug, PartialEq)]
enum GridSpace {
    Open,
    Tree,
}

impl From<char> for GridSpace {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Open,
            '#' => Self::Tree,
            c => panic!("bad character: {}", c),
        }
    }
}

#[derive(Debug)]
struct Map {
    rows: usize,
    cols: usize,
    grid: Vec<GridSpace>,
}

impl Map {
    fn product_of_trees_from_toboggans(&self, toboggans: &[Toboggan]) -> usize {
        toboggans
            .iter()
            .map(|t| self.num_trees_traversed(t))
            .product()
    }

    fn num_trees_traversed(&self, toboggan: &Toboggan) -> usize {
        let mut num_trees = 0;
        let (mut row, mut col) = (0, 0);

        while row < self.rows {
            if *self.get_space(row, col) == GridSpace::Tree {
                num_trees += 1;
            }
            row += toboggan.down;
            col += toboggan.right;
        }

        num_trees
    }

    fn get_space(&self, row: usize, col: usize) -> &GridSpace {
        // Because of arboreal genetics and biome stability, the same pattern repeats to the right many times
        let col = col % self.cols;
        let i = row * self.cols + col;
        self.grid.get(i).expect("bad in Map.grid")
    }
}

impl From<&str> for Map {
    fn from(s: &str) -> Self {
        let mut cols = 0;
        let mut grid = Vec::new();

        for line in s.lines() {
            cols = line.len();
            line.chars().for_each(|c| grid.push(GridSpace::from(c)));
        }

        let rows = grid.len() / cols;

        Self { rows, cols, grid }
    }
}

struct Toboggan {
    right: usize,
    down: usize,
}

impl Toboggan {
    fn new(right: usize, down: usize) -> Self {
        Self { right, down }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn test_map_product_of_trees_from_toboggans() {
        let map = Map::from(INPUT);

        let toboggans = [
            Toboggan::new(1, 1),
            Toboggan::new(3, 1),
            Toboggan::new(5, 1),
            Toboggan::new(7, 1),
            Toboggan::new(1, 2),
        ];

        assert_eq!(map.product_of_trees_from_toboggans(&toboggans), 336);
    }

    #[test]
    fn test_map_num_trees_traversed() {
        let map = Map::from(INPUT);

        let toboggan = Toboggan::new(3, 1);

        assert_eq!(map.num_trees_traversed(&toboggan), 7);
    }

    #[test]
    fn test_map_from_str() {
        let map = Map::from(".#\n#.");

        assert_eq!(map.rows, 2);
        assert_eq!(map.cols, 2);
    }

    #[test]
    fn test_gridspace_from_char() {
        assert_eq!(GridSpace::from('.'), GridSpace::Open);
        assert_eq!(GridSpace::from('#'), GridSpace::Tree);
    }
}
