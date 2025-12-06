use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Range {
    lower: u64,
    upper: u64,
}

impl Range {
    fn overlap(&self, other: &Self) -> Option<Self> {
        let lower = self.lower.max(other.lower);
        let upper = self.upper.min(other.upper);

        (lower <= upper).then_some(Range {
            lower: self.lower.min(other.lower),
            upper: self.upper.max(other.upper),
        })
    }

    fn contains(&self, item: u64) -> bool {
        item >= self.lower && item <= self.upper
    }
}

fn remove_overlaps(ranges: Vec<Range>) -> Vec<Range> {
    let mut fixed = vec![];
    let mut overlapped = HashSet::new();

    for (i, range) in ranges.iter().enumerate() {
        if overlapped.contains(&i) {
            continue;
        }

        let curr_len = fixed.len();

        for (j, other) in ranges.iter().enumerate().skip(i + 1) {
            if let Some(overlap) = range.overlap(other) {
                overlapped.insert(j);
                fixed.push(overlap);
                break;
            }
        }

        if curr_len == fixed.len() {
            fixed.push(*range);
        }
    }

    fixed
}

fn part_one(items: &[u64], ranges: &[Range]) -> u64 {
    let mut frest_ingredients = 0;

    for id in items {
        for range in ranges {
            if range.contains(*id) {
                frest_ingredients += 1;
                break;
            }
        }
    }

    frest_ingredients
}

fn part_two(mut ranges: Vec<Range>) -> u64 {
    let mut prev_len = 0;

    while prev_len != ranges.len() {
        println!("{ranges:?}");
        prev_len = ranges.len();
        ranges = remove_overlaps(ranges);
    }

    ranges
        .iter()
        .map(|range| range.upper - range.lower + 1)
        .sum()
}

fn preproces(input: &str) -> (Vec<u64>, Vec<Range>) {
    let mut lines = input.lines();

    let ranges: Vec<_> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (left, right) = line.split_once('-').unwrap();
            Range {
                lower: left.parse().unwrap(),
                upper: right.parse().unwrap(),
            }
        })
        .collect();

    let items: Vec<u64> = lines.map(|line| line.parse().unwrap()).collect();

    (items, ranges)
}

fn main() {
    let input = read_to_string("inputs/day_05.txt").unwrap();

    let (items, ranges) = preproces(&input);

    let solution = part_one(&items, &ranges);
    println!("Part one solution is {solution}");

    let solution = part_two(ranges);
    println!("Part two solution is {solution}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3-5\n\
                         10-14\n\
                         16-20\n\
                         12-18\n\
                         \n\
                         1\n\
                         5\n\
                         8\n\
                         11\n\
                         17\n\
                         32\n";

    #[test]
    fn test_part_one() {
        let expected = 3;
        let (items, ranges) = preproces(INPUT);
        let result = part_one(&items, &ranges);

        assert_eq!(result, expected)
    }

    #[test]
    fn test_part_two() {
        let expected = 14;
        let (_, ranges) = preproces(INPUT);
        let result = part_two(ranges);

        assert_eq!(result, expected)
    }
}
