use input;
use std::io::Result;

use std::collections::VecDeque;

fn main() -> Result<()> {
    let content = input::load_file("src/day9/input.txt")?;

    let preamble_size = 25;
    let num = first_number(&content, preamble_size);
    println!("part1: {}", num);

    println!("part2: {}", sum_contiguous(&content, num));

    Ok(())
}

pub fn first_number(data: &str, preamble_size: usize) -> usize {
    let data: Vec<usize> = data.lines().map(|l| l.parse().unwrap()).collect();

    *data
        .iter()
        .enumerate()
        .skip(preamble_size)
        .find(|(i, c)| {
            for j in (i - preamble_size)..(i - 1) {
                for k in (j + 1)..*i {
                    let a = data.get(j).unwrap();
                    let b = data.get(k).unwrap();
                    if *a + *b == **c {
                        return false;
                    }
                }
            }
            true
        })
        .unwrap()
        .1
}

pub fn sum_contiguous(data: &str, num: usize) -> usize {
    let mut contiguous = VecDeque::new();
    let mut sum = 0;
    for line in data.lines() {
        let value = line.parse::<usize>().unwrap();
        contiguous.push_back(value);
        sum += value;
        if sum == num {
            break;
        } else if sum > num {
            while sum > num {
                // start removing items
                let value = contiguous.pop_front().unwrap();
                sum -= value;
            }
            if sum == num {
                break;
            }
        }
    }

    contiguous.iter().min().unwrap() + contiguous.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

        let preamble_size = 5;

        let num = first_number(&data, preamble_size);
        assert_eq!(num, 127);

        let s = sum_contiguous(&data, num);
        assert_eq!(s, 62);
    }
}
