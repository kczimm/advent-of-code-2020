use input;
use std::io::Result;
fn main() -> Result<()> {
    let lines = input::load_file("src/day2/input.txt")?;

    println!("part1: {}", part1(&lines));

    println!("part2: {}", part2(&lines));

    Ok(())
}

struct CorporatePolicy {
    first: u32,
    second: u32,
    letter: char,
}

impl CorporatePolicy {
    fn new(text: &str) -> CorporatePolicy {
        let hyphen = text.find('-').expect("no hyphen in policy");
        let space = text.find(' ').expect("no space in policy");
        let first = text[0..hyphen]
            .parse::<u32>()
            .expect("parse of first failed");
        let second = text[hyphen + 1..space]
            .parse::<u32>()
            .expect("parse of second failed");
        let letter = text.chars().nth(space + 1).expect("no letter in policy");

        CorporatePolicy {
            first,
            second,
            letter,
        }
    }

    fn is_valid_low_high(&self, password: &str) -> bool {
        let mut count = 0;
        for letter in password.chars() {
            if letter == self.letter {
                count += 1;
            }
        }

        self.first <= count && count <= self.second
    }

    fn is_valid_positions(&self, password: &str) -> bool {
        let chars: Vec<char> = password.chars().collect();

        (chars[self.first as usize - 1] == self.letter)
            ^ (chars[self.second as usize - 1] == self.letter)
    }
}

fn part1(lines: &Vec<String>) -> usize {
    lines
        .iter()
        .filter(|&line| {
            let mut parts = line.split(": ");
            let policy = CorporatePolicy::new(parts.next().expect("missing policy in line"));
            let password = parts.next().expect("missing password in line");
            policy.is_valid_low_high(password)
        })
        .map(|_| 1)
        .sum()
}

fn part2(lines: &Vec<String>) -> usize {
    lines
        .iter()
        .filter(|&line| {
            let mut parts = line.split(": ");
            let policy = CorporatePolicy::new(parts.next().expect("missing policy in line"));
            let password = parts.next().expect("missing password in line");
            policy.is_valid_positions(password)
        })
        .map(|_| 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corporate_policy() {
        let policy = CorporatePolicy::new("1-3 a");
        assert!(policy.is_valid_low_high("abcde"));
        assert!(policy.is_valid_positions("abcde"));

        let policy = CorporatePolicy::new("1-3 b");
        assert!(!policy.is_valid_low_high("cdefg"));
        assert!(!policy.is_valid_positions("cdefg"));

        let policy = CorporatePolicy::new("2-9 c");
        assert!(policy.is_valid_low_high("ccccccccc"));
        assert!(!policy.is_valid_positions("ccccccccc"));
    }

    #[test]
    fn test_part1() {
        let lines = vec![
            "1-3 a: abcde".to_owned(),
            "1-3 b: cdefg".to_owned(),
            "2-9 c: ccccccccc".to_owned(),
        ];

        assert_eq!(part1(&lines), 2);
    }

    #[test]
    fn test_part2() {
        let lines = vec![
            "1-3 a: abcde".to_owned(),
            // "1-3 b: cdefg".to_owned(),
            // "2-9 c: ccccccccc".to_owned(),
        ];

        assert_eq!(part2(&lines), 1);
    }
}
