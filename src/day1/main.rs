use input;
use std::io::Result;
fn main() -> Result<()> {
    let expense_report = input::load_file_by_lines("src/day1/input.txt")?;
    let expense_report: Vec<u32> = expense_report
        .into_iter()
        .map(|a| a.parse::<u32>().expect("parse failed"))
        .collect();

    println!(
        "part1: {}",
        part1(&expense_report).expect("didn't get an answer for part1")
    );

    println!(
        "part2: {}",
        part2(&expense_report).expect("didn't get an answer for part2")
    );

    Ok(())
}

/// Before you leave, the Elves in accounting just need you to fix your expense report (your puzzle input); apparently, something isn't quite adding up.
/// Specifically, they need you to find the two entries that sum to 2020 and then multiply those two numbers together.
fn part1(expense_report: &Vec<u32>) -> Option<u32> {
    for i in 0..expense_report.len() - 1 {
        for j in i..expense_report.len() {
            let a = expense_report[i];
            let b = expense_report[j];
            if a + b == 2020 {
                return Some(a * b);
            }
        }
    }
    None
}

/// In your expense report, what is the product of the three entries that sum to 2020?
fn part2(expense_report: &Vec<u32>) -> Option<u32> {
    for i in 0..expense_report.len() - 2 {
        for j in i..expense_report.len() - 1 {
            for k in j..expense_report.len() {
                let a = expense_report[i];
                let b = expense_report[j];
                let c = expense_report[k];
                if a + b + c == 2020 {
                    return Some(a * b * c);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let expense_report = vec![1721, 979, 366, 299, 675, 1456];

        assert_eq!(part1(&expense_report), Some(514579));
    }

    #[test]
    fn test_part2() {
        let expense_report = vec![1721, 979, 366, 299, 675, 1456];

        assert_eq!(part2(&expense_report), Some(241861950));
    }
}
