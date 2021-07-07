use input;
use std::io::Result;

fn main() -> Result<()> {
    let content = input::load_file("src/day10/input.txt")?;

    println!("part1: {}", part1(&content));
    let bag = Bag::from(&content);
    println!("part2: {}", number_of_arrangements_recursion(&bag.0));

    Ok(())
}

pub fn part1(text: &str) -> usize {
    let bag = Bag::from(text);
    let (ones, threes) = bag.differences();
    ones * threes
}

pub type Adapter = usize;

pub struct Bag(Vec<Adapter>);

impl Bag {
    pub fn from(text: &str) -> Self {
        let mut adapters: Vec<Adapter> = text
            .lines()
            .map(|line| line.parse().expect("parse failed"))
            .collect();
        let mut with_zero = vec![0];
        with_zero.append(&mut adapters);
        with_zero.sort();
        Bag(with_zero)
    }

    pub fn differences(&self) -> (usize, usize) {
        let mut ones = 0;
        let mut threes = 1;

        for i in 1..self.0.len() {
            match self.0[i] - self.0[i - 1] {
                1 => {
                    ones += 1;
                }
                3 => {
                    threes += 1;
                }
                _ => {}
            }
        }

        (ones, threes)
    }
}

pub fn number_of_arrangements_recursion(adapters: &[Adapter]) -> usize {
    match adapters.len() {
        1 => 1,
        2 => 1,
        3 => {
            let a = if adapters[1] - adapters[0] <= 3 {
                number_of_arrangements_recursion(&adapters[1..])
            } else {
                0
            };
            let b = if adapters[2] - adapters[0] <= 3 {
                number_of_arrangements_recursion(&adapters[2..])
            } else {
                0
            };
            a + b
        }
        _ => {
            let a = if adapters[1] - adapters[0] <= 3 {
                number_of_arrangements_recursion(&adapters[1..])
            } else {
                0
            };
            let b = if adapters[2] - adapters[0] <= 3 {
                number_of_arrangements_recursion(&adapters[2..])
            } else {
                0
            };
            let c = if adapters[3] - adapters[0] <= 3 {
                number_of_arrangements_recursion(&adapters[3..])
            } else {
                0
            };
            a + b + c
        }
    }
}

pub fn number_of_arrangements(adapters: &[Adapter]) -> usize {
    let mut num = 1;
    let mut i = adapters.len() - 1;

    // going backwards through the list.
    while i >= 3 {
        let j = i - 2;
        let k = i - 3;

        if (adapters[i] - adapters[k]) <= 3 {
            num *= 4;
            i -= 2;
        } else if (adapters[i] - adapters[j]) <= 3 {
            num *= 2;
            i -= 1;
        } else {
            i -= 1;
        }
    }

    num
}

fn combos(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        2 => 1,
        n => combos(n - 1) + combos(n - 2) + combos(n - 3),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        let text = "16
10
15
5
1
11
7
19
6
12
4";
        let bag = Bag::from(&text);

        assert_eq!(bag.differences(), (7, 5));
    }

    #[test]
    fn test_part1_2() {
        let text = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        let bag = Bag::from(&text);

        assert_eq!(bag.differences(), (22, 10));
    }

    #[test]
    fn test_part2_1() {
        let text = "16
10
15
5
1
11
7
19
6
12
4";
        let bag = Bag::from(&text);

        assert_eq!(number_of_arrangements(&bag.0), 8);
    }

    #[test]
    fn test_part2_2() {
        let text = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        let bag = Bag::from(&text);

        assert_eq!(number_of_arrangements(&bag.0), 19208);
    }
}
