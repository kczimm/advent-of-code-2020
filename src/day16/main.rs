use input;

use std::io;
use std::ops::RangeInclusive;

fn main() -> io::Result<()> {
    let contents = input::load_file("src/day16/input.txt")?;

    let notes = Notes::from(contents.as_ref());

    println!("part1: {}", notes.ticket_scanning_error_rate());

    Ok(())
}

#[derive(Debug, PartialEq)]
struct Rule<'a> {
    field_name: &'a str,
    ranges: (RangeInclusive<usize>, RangeInclusive<usize>),
}

impl<'a> Rule<'a> {
    fn is_valid_value(&self, value: usize) -> bool {
        self.ranges.0.contains(&value) || self.ranges.1.contains(&value)
    }
}

impl<'a> From<&'a str> for Rule<'a> {
    fn from(s: &'a str) -> Self {
        let mut parts = s.split(": ");
        let field_name = parts.nth(0).unwrap();
        let mut parts = parts.nth(0).unwrap().split(' ');
        let first = parts.nth(0).unwrap();
        let second = parts.nth(1).unwrap();

        let mut parts = first.split('-');
        let first = RangeInclusive::new(
            parts.nth(0).unwrap().parse().unwrap(),
            parts.nth(0).unwrap().parse().unwrap(),
        );

        let mut parts = second.split('-');
        let second = RangeInclusive::new(
            parts.nth(0).unwrap().parse().unwrap(),
            parts.nth(0).unwrap().parse().unwrap(),
        );

        Self {
            field_name,
            ranges: (first, second),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Ticket(Vec<usize>);

impl Ticket {
    fn is_valid(&self, r: &Rule) -> bool {
        self.0
            .iter()
            .any(|value| r.ranges.0.contains(value) | r.ranges.1.contains(value))
    }
}

impl From<&str> for Ticket {
    fn from(s: &str) -> Self {
        Self(
            s.split(',')
                .map(|n| n.parse().expect("parsing ticket failed"))
                .collect(),
        )
    }
}

#[derive(Debug, PartialEq)]
struct Notes<'a> {
    rules: Vec<Rule<'a>>,
    your: Ticket,
    nearby: Vec<Ticket>,
}

impl<'a> Notes<'a> {
    fn ticket_scanning_error_rate(&self) -> usize {
        let mut invalid_values = Vec::new();

        for ticket in &self.nearby {
            for value in &ticket.0 {
                if !self.valid_for_any_field(*value) {
                    invalid_values.push(*value);
                }
            }
        }

        invalid_values.iter().sum()
    }

    fn valid_for_any_field(&self, value: usize) -> bool {
        self.rules.iter().any(|r| r.is_valid_value(value))
    }

    fn part2(&self) -> usize {
        let valid_tickets = self
            .nearby
            .iter()
            .filter(|&t| self.rules.iter().all(|r| t.is_valid(r)));

        

        0
    }
}

impl<'a> From<&'a str> for Notes<'a> {
    fn from(s: &'a str) -> Self {
        let mut parts = s.split("\n\n");

        let rules = parts
            .nth(0)
            .unwrap()
            .lines()
            .map(|l| Rule::from(l))
            .collect();

        let your = Ticket::from(parts.nth(0).unwrap().lines().nth(1).unwrap());

        let nearby = parts
            .nth(0)
            .unwrap()
            .lines()
            .skip(1)
            .map(|line| Ticket::from(line))
            .collect();

        Self {
            rules,
            your,
            nearby,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rules_from() {
        let rule = Rule::from("class: 1-3 or 5-7");

        assert_eq!(
            rule,
            Rule {
                field_name: "class",
                ranges: (1..=3, 5..=7)
            }
        );

        let rule = Rule::from("row: 6-11 or 33-44");

        assert_eq!(
            rule,
            Rule {
                field_name: "row",
                ranges: (6..=11, 33..=44)
            }
        );

        let rule = Rule::from("seat: 13-40 or 45-50");

        assert_eq!(
            rule,
            Rule {
                field_name: "seat",
                ranges: (13..=40, 45..=50)
            }
        );

        let rule = Rule::from("departure location: 36-363 or 377-962");

        assert_eq!(
            rule,
            Rule {
                field_name: "departure location",
                ranges: (36..=363, 377..=962)
            }
        );
    }

    #[test]
    fn test_ticket_from() {
        let ticket = Ticket::from("7,1,14");

        assert_eq!(ticket.0, vec![7, 1, 14]);
    }

    #[test]
    fn test_notes_from() {
        let notes = Notes::from(
            "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12",
        );

        assert_eq!(notes.your.0, vec![7, 1, 14]);
    }

    #[test]
    fn test_notes_ticket_scanning_error_rate() {
        let notes = Notes::from(
            "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12",
        );

        assert_eq!(notes.ticket_scanning_error_rate(), 71);
    }
}
