use std::{collections::HashSet, fs::read_to_string};

// Find divisors of len
fn find_divisors(n: usize) -> Vec<usize> {
    (1..=n / 2).filter(|i| n.is_multiple_of(*i)).collect()
}

fn check_from_lower(
    lower: &str,
    smaller: u64,
    bigger: u64,
    set: &mut HashSet<u64>,
    use_half_len: bool,
) {
    let len = lower.len();
    let divisors = if use_half_len {
        if !len.is_multiple_of(2) {
            return;
        }
        vec![len / 2]
    } else {
        find_divisors(len)
    };

    for d in divisors {
        let n = len / d;
        let (first, _) = lower.split_at(d);

        let rep = first.repeat(n);
        let mut a: u64 = rep.parse().unwrap();
        let mut running: u64 = first.parse().unwrap();

        while a <= bigger {
            if a >= smaller {
                set.insert(a);
            }

            running += 1;
            a = running.to_string().repeat(n).parse().unwrap();
        }
    }
}

fn check_from_upper(
    upper: &str,
    smaller: u64,
    bigger: u64,
    set: &mut HashSet<u64>,
    use_half_len: bool,
) {
    let len = upper.len();
    let divisors = if use_half_len {
        if !len.is_multiple_of(2) {
            return;
        }
        vec![len / 2]
    } else {
        find_divisors(len)
    };

    for d in divisors {
        let n = len / d;
        let (_, last) = upper.split_at(len - d);

        let rep = last.repeat(n);
        let mut a: u64 = rep.parse().unwrap();
        let mut running: u64 = last.parse().unwrap();

        while a >= smaller {
            if a <= bigger {
                set.insert(a);
            }

            running -= 1;
            a = running.to_string().repeat(n).parse().unwrap();
        }
    }
}

fn part_one(lines: &str) -> u64 {
    lines.split(',').fold(0, |acc, range| {
        let (lower, upper) = range.split_once('-').unwrap();

        let smaller: u64 = lower.parse().expect(lower);
        let bigger: u64 = upper.parse().expect(upper);

        let mut set = HashSet::new();

        check_from_lower(lower, smaller, bigger, &mut set, true);
        check_from_upper(upper, smaller, bigger, &mut set, true);

        acc + set.iter().sum::<u64>()
    })
}

fn part_two(lines: &str) -> u64 {
    lines.split(',').fold(0, |acc, range| {
        let (lower, upper) = range.split_once('-').unwrap();

        let smaller: u64 = lower.parse().expect(lower);
        let bigger: u64 = upper.parse().expect(upper);

        let mut set = HashSet::new();

        check_from_lower(lower, smaller, bigger, &mut set, false);
        check_from_upper(upper, smaller, bigger, &mut set, false);

        acc + set.iter().sum::<u64>()
    })
}

fn main() {
    let input = read_to_string("inputs/day_02.txt").unwrap();
    let trimmed = input.trim_end();

    let password = part_one(trimmed);
    println!("Part one solution is {password}");

    let password = part_two(trimmed);
    println!("Part two password is {password}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part_one() {
        let expected: u64 = 1227775554;
        let result = part_one(INPUT);

        assert_eq!(result, expected)
    }

    #[test]
    fn test_part_two() {
        let expected = 4174379265;
        let result = part_two(INPUT);

        assert_eq!(result, expected)
    }
}
