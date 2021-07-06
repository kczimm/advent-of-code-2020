use input;
use std::io::Result;

use std::collections::HashMap;

fn main() -> Result<()> {
    let content = input::load_file("src/day6/input.txt")?;

    println!("part1: {}", part1(&content));

    println!("part2: {}", part2(&content));

    Ok(())
}

pub fn part1(text: &str) -> usize {
    let plane = Plane::from(&text);
    plane.sum_of_anyone_answers()
}

pub fn part2(text: &str) -> usize {
    let plane = Plane::from(&text);
    plane.sum_of_everyone_answers()
}

pub struct Plane<'a>(Vec<Group<'a>>);

impl<'a> Plane<'a> {
    fn from(text: &'a str) -> Self {
        Self(text.split("\n\n").map(|g| Group::from(g)).collect())
    }

    fn sum_of_anyone_answers(&self) -> usize {
        self.0.iter().map(|g| g.anyone_yes_answers()).sum()
    }

    fn sum_of_everyone_answers(&self) -> usize {
        self.0.iter().map(|g| g.everyone_yes_answers()).sum()
    }
}

pub struct Group<'a> {
    answers: Vec<MemberAnswer<'a>>,
}

impl<'a> Group<'a> {
    pub fn from(text: &'a str) -> Self {
        Self {
            answers: text.lines().collect(),
        }
    }

    pub fn anyone_yes_answers(&self) -> usize {
        let mut answers = HashMap::new();
        for member_answer in &self.answers {
            for c in member_answer.chars() {
                *answers.entry(c).or_insert(1) += 1;
            }
        }
        answers.keys().count()
    }

    pub fn everyone_yes_answers(&self) -> usize {
        let mut answers = HashMap::new();
        for member_answer in &self.answers {
            for c in member_answer.chars() {
                *answers.entry(c).or_insert(0) += 1;
            }
        }
        let group_size = self.answers.len();
        answers.values().filter(|&v| *v == group_size).count()
    }
}

type MemberAnswer<'a> = &'a str;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group() {
        let text = "abcx
abcy
abcz";
        let group = Group::from(text);
        assert_eq!(group.anyone_yes_answers(), 6);
    }

    #[test]
    fn test_part2() {
        let text = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let plane = Plane::from(text);
        assert_eq!(plane.sum_of_everyone_answers(), 6);
    }
}
