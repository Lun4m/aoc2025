use std::{collections::HashSet, fs::read_to_string, str::Lines};

/// Recursive implementation, initially implemented when solving part one
fn find_joltage_recursive(index: usize, bank: &[u8], already_seen: &mut HashSet<u8>) -> u64 {
    if (index + 1) == bank.len() {
        return 0;
    }

    let first = *bank.get(index).unwrap();
    let mut second = 0;
    let mut joltages = vec![];

    for i in index + 1..bank.len() {
        let battery = bank[i];

        if battery > first && already_seen.get(&battery).is_none() {
            already_seen.insert(battery);

            let joltage = find_joltage_recursive(i, bank, already_seen);
            joltages.push(joltage);
        }

        if battery > second {
            second = battery;
        }
    }

    let joltage = (10 * first + second) as u64;
    joltages.push(joltage);
    joltages.into_iter().max().unwrap()
}

// Implemented for part two, but actually also works for part one
// and it's easier to understand
fn find_joltage(bank: &[u8], n: usize) -> u64 {
    let mut joltage = 0;
    let mut idx = 0;

    for i in 0..n {
        let mut biggest = 0;

        for j in idx..bank.len() {
            if j + n - 1 - i == bank.len() {
                break;
            }

            let battery = bank[j];

            if battery > biggest {
                biggest = battery;
                // This should be fine given the first
                // if condition in the inner loop
                idx = j + 1;
            }
        }

        joltage += biggest as u64 * 10u64.pow((n - 1 - i) as u32);
    }

    joltage
}

fn part_one(banks: Lines) -> u64 {
    banks.fold(0, |acc, bank| {
        // Convert chars to u8
        let batteries: Vec<u8> = bank.as_bytes().iter().map(|b| b - 48).collect();

        // Keep track of already seen battery joltages,
        // so we avoid doing the same work multiple times
        let mut seen = HashSet::new();

        let joltage = find_joltage_recursive(0, &batteries, &mut seen);
        acc + joltage
    })
}

fn part_two(banks: Lines) -> u64 {
    banks.fold(0, |acc, bank| {
        // Convert chars to u8
        let batteries: Vec<u8> = bank.as_bytes().iter().map(|b| b - 48).collect();

        let joltage = find_joltage(&batteries, 12);
        acc + joltage
    })
}

fn main() {
    let input = read_to_string("inputs/day_03.txt").unwrap();

    let solution = part_one(input.lines());
    println!("Part one solution is {solution}");

    let solution = part_two(input.lines());
    println!("Part two solution is {solution}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "987654321111111\n\
                         811111111111119\n\
                         234234234234278\n\
                         818181911112111\n";

    #[test]
    fn test_part_one() {
        let expected: u64 = 357;
        let result = part_one(INPUT.lines());

        assert_eq!(result, expected)
    }

    #[test]
    fn test_part_two() {
        let expected = 3121910778619;
        let result = part_two(INPUT.lines());

        assert_eq!(result, expected)
    }
}
