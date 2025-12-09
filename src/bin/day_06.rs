use std::{fs::read_to_string, str::Lines};

fn part_one(lines: Lines) -> u64 {
    let mut split: Vec<Vec<&str>> = lines
        .map(|line| line.split_whitespace().collect())
        .collect();

    let operations = split.pop().unwrap();

    // Initialize column results
    let mut results: Vec<u64> = operations
        .iter()
        .map(|op| match *op {
            "+" => 0,
            "*" => 1,
            _ => unreachable!(),
        })
        .collect();

    for line in split {
        for (i, (num, op)) in line.iter().zip(&operations).enumerate() {
            let num: u64 = num.parse().unwrap();

            match *op {
                "+" => results[i] += num,
                "*" => results[i] *= num,
                _ => unreachable!(),
            };
        }
    }

    results.iter().sum()
}

fn part_two(lines: Lines) -> u64 {
    let mut split: Vec<&str> = lines.collect();
    let operations = split.pop().unwrap();

    let lenghts: Vec<_> = operations
        .split(&['*', '+'])
        .map(|s| s.len() + 1)
        // The first len is 0 because the first operation char is at pos 0
        .skip(1)
        .collect();

    let operations: Vec<&str> = operations.split_whitespace().collect();

    let (total, _) = operations
        .iter()
        .zip(lenghts)
        // Init total and start of string slice
        .fold((0, 0), |(total, start), (op, offset)| {
            let nums = (0..offset).filter_map(|i| {
                let idx = start + i;
                let n: String = split.iter().map(|s| &s[idx..idx + 1]).collect();

                match n.trim() {
                    "" => None,
                    v => Some(v.parse::<u64>().unwrap()),
                }
            });

            let result = match *op {
                "+" => nums.sum::<u64>(),
                "*" => nums.product::<u64>(),
                _ => unreachable!(),
            };

            (total + result, start + offset)
        });

    total
}

fn main() {
    let input = read_to_string("inputs/day_06.txt").unwrap();

    let solution = part_one(input.lines());
    println!("Part one solution is {solution}");

    let solution = part_two(input.lines());
    println!("Part two solution is {solution}");
    assert!(solution > 2047589286843);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part_one() {
        let expected = 4277556;
        let result = part_one(INPUT.lines());

        assert_eq!(result, expected)
    }

    #[test]
    fn test_part_two() {
        let expected = 3263827;
        let result = part_two(INPUT.lines());

        assert_eq!(result, expected)
    }
}
