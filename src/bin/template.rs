use std::fs::read_to_string;

fn part_one() -> u64 {
    todo!()
}

fn part_two() -> u64 {
    todo!()
}

fn main() {
    let input = read_to_string("inputs/day_00.txt").unwrap();

    let solution = part_one();
    println!("Part one solution is {solution}");

    let solution = part_two();
    println!("Part two solution is {solution}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn test_part_one() {
        let expected = 0;
        let result = part_one();

        assert_eq!(result, expected)
    }

    #[test]
    fn test_part_two() {
        let expected = 0;
        let result = part_two();

        assert_eq!(result, expected)
    }
}
