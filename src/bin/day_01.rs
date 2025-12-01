use std::{fs::read_to_string, str::Lines};

fn parse_sign_step(line: &str) -> (i32, i32) {
    let (dir, step) = line.split_at(1);

    let sign = match dir {
        "R" => 1,
        "L" => -1,
        _ => unreachable!(),
    };

    let step = step.parse::<i32>().unwrap();

    (sign, step)
}

fn part_one(lines: Lines) -> i32 {
    let (_, password) = lines.fold((50, 0), |(dial, acc), line| {
        let (sign, step) = parse_sign_step(line);

        let new = dial + sign * step;
        let new = new.rem_euclid(100);

        let pass = if new == 0 { 1 } else { 0 };

        (new, acc + pass)
    });

    password
}

fn part_two(lines: Lines) -> i32 {
    let (_, password) = lines.fold((50, 0), |(dial, acc), line| {
        let (sign, step) = parse_sign_step(line);

        let new = dial + sign * step;

        // Get number of total crossings
        let mut pass = (new / 100).abs();

        if dial != 0 && dial.signum() != new.signum() {
            pass += 1
        }

        #[cfg(test)]
        println!("{dial:3} -> {new:4} -> {new:3} => {pass}");

        (new.rem_euclid(100), acc + pass)
    });

    password
}

fn main() {
    let input = read_to_string("inputs/day_01.txt").unwrap();

    let password = part_one(input.lines());
    println!("Part one password is {password}");

    let password = part_two(input.lines());
    println!("Part two password is {password}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "L68\nL30\nR48\nL5\nR60\n\
                         L55\nL1\nL99\nR14\nL82\nR1000";

    #[test]
    fn test_part_one() {
        let expected = 3;
        let result = part_one(INPUT.lines());

        assert_eq!(result, expected)
    }

    #[test]
    fn test_part_two() {
        let expected = 16;
        let result = part_two(INPUT.lines());

        assert_eq!(result, expected)
    }
}
