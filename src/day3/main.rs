use input;
use std::io::Result;
fn main() -> Result<()> {
    let map = input::load_file("src/day3/input.txt")?;

    let direction = (1, 3);

    println!("part1: {}", part1(&map, direction));

    Ok(())
}

fn part1(map: &Vec<String>, direction: (usize, usize)) -> u32 {
    let mut num_trees = 0;
    let mut r = direction.0;
    let mut c = direction.1;

    let max_cols: usize = map.get(0).expect("no row in map").len();

    loop {
        if let Some(line) = map.get(r) {
            let mut chars = line.chars();
            let spot = chars.nth(c).expect("column out of bounds");
            if spot == '#' {
                num_trees += 1;
            }
            r += direction.0;
            c = (c + direction.1) % max_cols;
        } else {
            break;
        }
    }

    num_trees
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let map = vec![
            "..##.......".to_owned(),
            "#...#...#..".to_owned(),
            ".#....#..#.".to_owned(),
            "..#.#...#.#".to_owned(),
            ".#...##..#.".to_owned(),
            "..#.##.....".to_owned(),
            ".#.#.#....#".to_owned(),
            ".#........#".to_owned(),
            "#.##...#...".to_owned(),
            "#...##....#".to_owned(),
            ".#..#...#.#".to_owned(),
        ];

        let direction = (1, 3);

        assert_eq!(part1(&map, direction), 7);
    }
}
