use std::{fs::read_to_string, str::Lines};

fn part_one(mut lines: Lines) -> u64 {
    let mut prev: Vec<_> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| matches!(c, 'S'))
        .collect();

    let splits = lines.map(|line| {
        line.chars().enumerate().fold(0, |mut acc, (i, c)| {
            if prev[i] && c == '^' {
                prev[i] = false;
                prev[i - 1] = true;
                prev[i + 1] = true;
                acc += 1;
            }
            acc
        })
    });

    splits.sum()
}

fn part_two(mut lines: Lines) -> u64 {
    let mut prev: Vec<_> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| if matches!(c, 'S') { 1 } else { 0 })
        .collect();

    lines.for_each(|line| {
        line.chars()
            .enumerate()
            .filter(|&(_, c)| c == '^')
            .for_each(|(i, _)| {
                prev[i - 1] += prev[i];
                prev[i + 1] += prev[i];
                prev[i] = 0;
            });
    });

    prev.iter().sum()
}

fn main() {
    let input = read_to_string("inputs/day_07.txt").unwrap();

    let solution = part_one(input.lines());
    println!("Part one solution is {solution}");

    let solution = part_two(input.lines());
    println!("Part two solution is {solution}");

    assert!(solution > 3110);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ".......S.......\n\
                         ...............\n\
                         .......^.......\n\
                         ...............\n\
                         ......^.^......\n\
                         ...............\n\
                         .....^.^.^.....\n\
                         ...............\n\
                         ....^.^...^....\n\
                         ...............\n\
                         ...^.^...^.^...\n\
                         ...............\n\
                         ..^...^.....^..\n\
                         ...............\n\
                         .^.^.^.^.^...^.\n\
                         ...............";

    #[test]
    fn test_part_one() {
        let expected = 21;
        let result = part_one(INPUT.lines());

        assert_eq!(result, expected)
    }

    #[test]
    fn test_part_two() {
        let expected = 40;
        let result = part_two(INPUT.lines());

        assert_eq!(result, expected)
    }
}
