use std::fs::read_to_string;

fn check_row(row: &[char], idx: usize) -> i32 {
    let mut count = 0;

    if idx > 0 && row[idx - 1] == '@' {
        count += 1;
    }

    if row[idx] == '@' {
        count += 1;
    }

    if let Some('@') = row.get(idx + 1) {
        count += 1;
    }

    count
}

fn part_one(grid: &[Vec<char>]) -> u64 {
    let mut rolls = 0;

    for (i, current) in grid.iter().enumerate() {
        for (j, &col) in current.iter().enumerate() {
            if col == '.' {
                continue;
            }

            let mut count = check_row(current, j);

            if i != 0 {
                let above = &grid[i - 1];
                count += check_row(above, j)
            }

            if let Some(below) = &grid.get(i + 1) {
                count += check_row(below, j)
            }

            // 4 adjcent + 1 current element
            if count < 5 {
                rolls += 1;
            }
        }
    }

    rolls
}

fn get_cleaned_grid(grid: &[Vec<char>]) -> (u64, Vec<Vec<char>>) {
    let mut rolls = 0;
    let mut new_grid = Vec::new();

    for (i, current) in grid.iter().enumerate() {
        let mut cloned = current.clone();

        for (j, col) in cloned.iter_mut().enumerate() {
            if *col == '.' {
                continue;
            }

            let mut count = check_row(current, j);

            if i != 0 {
                let above = &grid[i - 1];
                count += check_row(above, j)
            }

            if let Some(below) = &grid.get(i + 1) {
                count += check_row(below, j)
            }

            // 4 adjcent + 1 current element
            if count < 5 {
                // Remove paper roll from current place
                *col = '.';

                rolls += 1;
            }
        }

        new_grid.push(cloned);
    }

    (rolls, new_grid)
}

fn part_two(mut grid: Vec<Vec<char>>) -> u64 {
    let mut total = 0;

    loop {
        let (rolls, new_grid) = get_cleaned_grid(&grid);

        if rolls == 0 {
            break;
        }

        grid = new_grid;
        total += rolls
    }

    total
}

fn to_grid(s: &str) -> Vec<Vec<char>> {
    s.lines().map(|line| line.chars().collect()).collect()
}

fn main() {
    let input = read_to_string("inputs/day_04.txt").unwrap();

    let grid: Vec<Vec<char>> = to_grid(&input);

    let solution = part_one(&grid);
    println!("Part one solution is {solution}");

    let solution = part_two(grid);
    println!("Part two solution is {solution}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "..@@.@@@@.\n\
                         @@@.@.@.@@\n\
                         @@@@@.@.@@\n\
                         @.@@@@..@.\n\
                         @@.@@@@.@@\n\
                         .@@@@@@@.@\n\
                         .@.@.@.@@@\n\
                         @.@@@.@@@@\n\
                         .@@@@@@@@.\n\
                         @.@.@@@.@.\n";

    #[test]
    fn test_part_one() {
        let expected = 13;
        let grid = to_grid(INPUT);
        let result = part_one(&grid);

        assert_eq!(result, expected)
    }

    #[test]
    fn test_part_two() {
        let expected = 43;
        let grid = to_grid(INPUT);
        let result = part_two(grid);

        assert_eq!(result, expected)
    }
}
