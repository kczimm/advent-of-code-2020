use input;
use std::io::Result;

use std::collections::HashMap;

fn main() -> Result<()> {
    let content = input::load_file("src/day7/input.txt")?;

    println!(
        "part1: {}",
        Rules::from(&content).num_bags_contain("shiny gold")
    );

    println!(
        "part2: {}",
        Rules::from(&content).num_bags_hold("shiny gold")
    );

    Ok(())
}

#[derive(Debug)]
pub struct Rules<'a> {
    rules: HashMap<&'a str, Vec<(usize, &'a str)>>,
}

impl<'a> Rules<'a> {
    pub fn from(text: &'a str) -> Self {
        let mut rules = HashMap::new();

        for rule in text.lines() {
            let spaces: Vec<usize> = rule
                .char_indices()
                .filter_map(|(i, c)| match c {
                    ' ' => Some(i),
                    _ => None,
                })
                .collect();

            let key_bag = &rule[..*spaces.get(1).unwrap()];
            match spaces.len() {
                6 => {
                    rules.insert(key_bag, vec![]);
                }
                n if (n - 7) % 4 == 0 => {
                    let num_bags = (n - 7) / 4 + 1;

                    let mut carry_abilities = vec![];
                    for i in 0..num_bags {
                        let j = i * 4 + 3;
                        let start = *spaces.get(j).unwrap() + 1;
                        let end = *spaces.get(j + 1).unwrap();
                        let capacity = (&rule[start..end]).parse::<usize>().unwrap();

                        let start = *spaces.get(j + 1).unwrap() + 1;
                        let end = *spaces.get(j + 3).unwrap();
                        let value_bag = &rule[start..end];

                        carry_abilities.push((capacity, value_bag));
                    }
                    rules.insert(key_bag, carry_abilities);
                }
                _ => unreachable!(),
            }
        }

        Self { rules }
    }

    pub fn this_can_contain_that(&self, this: &str, that: &str) -> bool {
        match self.rules.get(this) {
            Some(bags) => bags
                .iter()
                .any(|(_, bag)| *bag == that || self.this_can_contain_that(bag, that)),
            None => false,
        }
    }

    pub fn num_bags_contain(&self, that: &str) -> usize {
        self.rules
            .keys()
            .filter(|this| self.this_can_contain_that(this, that))
            .count()
    }

    pub fn num_bags_hold(&self, bag: &str) -> usize {
        match self.rules.get(bag) {
            Some(bags) => bags
                .iter()
                .map(|(num, b)| num + num * self.num_bags_hold(b))
                .sum(),
            None => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        let rules = Rules::from(&input);
        assert_eq!(rules.num_bags_contain("shiny gold"), 4);
    }

    #[test]
    fn test_part2() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        let rules = Rules::from(&input);
        assert_eq!(rules.num_bags_hold("shiny gold"), 32);

        let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

        let rules = Rules::from(&input);
        assert_eq!(rules.num_bags_hold("shiny gold"), 126);
    }
}
