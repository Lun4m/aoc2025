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
        .skip(1)
        .collect();

    let operations: Vec<&str> = operations.split_whitespace().collect();
    // println!("{split:?} {}", split[0].len());

    let mut total = 0;
    let mut start = 0;
    for (op, offset) in operations.iter().zip(lenghts) {
        // Only four lines in the input
        let a = &split[0][start..start + offset];
        let b = &split[1][start..start + offset];
        let c = &split[2][start..start + offset];
        let d = &split[3][start..start + offset];

        print!("{offset} |{a}|{b}|{c}| => ");

        // TODO: make it work with variable sized splits
        // let numbers: Vec<_> = split
        //     .iter()
        //     .map(|line| &line[start..start + offset])
        //     .collect();

        // |123 |64 |
        // | 45 |23 |
        // |  6 |314|
        // let nums = numbers.iter().map(|n| n.chars().rev().collect());

        // TODO: make it work with variable sized splits
        let nums: Vec<_> = a
            .chars()
            .rev()
            .zip(b.chars().rev())
            .zip(c.chars().rev())
            .zip(d.chars().rev())
            .filter_map(|(((a_, b_), c_), d_)| {
                let s = format!("{a_}{b_}{c_}{d_}");
                match s.trim() {
                    "" => None,
                    v => Some(v.parse::<u64>().unwrap()),
                }
            })
            .collect();

        println!("{nums:?}");

        match *op {
            "+" => total += nums.iter().sum::<u64>(),
            "*" => total += nums.iter().product::<u64>(),
            _ => unreachable!(),
        };

        start += offset;
    }

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
