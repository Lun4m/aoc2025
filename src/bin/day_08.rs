use std::{collections::HashSet, fs::read_to_string};

struct Point([f32; 3]);

impl Point {
    fn distance(&self, other: &Self) -> f32 {
        self.0
            .iter()
            .zip(other.0)
            .map(|(s, o)| (s - o).powi(2))
            .sum::<f32>()
            .sqrt()
    }
}

fn merge_circuits(mut sets: Vec<HashSet<usize>>) -> Vec<HashSet<usize>> {
    let mut skip = Vec::new();

    for i in 0..sets.len() {
        if skip.contains(&i) {
            continue;
        }

        for j in i + 1..sets.len() {
            if sets[i].is_disjoint(&sets[j]) {
                continue;
            };

            let union = sets[i].union(&sets[j]).copied().collect();
            sets[i] = union;

            skip.push(j);
        }
    }

    sets.into_iter()
        .enumerate()
        .filter_map(|(i, s)| (!skip.contains(&i)).then_some(s))
        .collect()
}

fn parse_points(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let coords = line
                .split(',')
                .map(|coord| coord.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            Point(coords)
        })
        .collect()
}

fn compute_distances(points: Vec<Point>) -> Vec<(usize, usize, f32)> {
    // Compute distances
    let mut dists = Vec::with_capacity(points.len() - 1);
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let dist = points[i].distance(&points[j]);
            dists.push((i, j, dist));
        }
    }

    // Sort by distance
    dists.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    dists
}

fn part_one(input: &str, n_max_pairs: usize) -> u64 {
    let points = parse_points(input);
    let dists = compute_distances(points);

    // Create circuits
    let &(i, j, _) = dists.first().unwrap();
    let mut circuits = vec![HashSet::from([i, j])];

    for (i, j, _) in dists.iter().skip(1).take(n_max_pairs - 1) {
        let mut inserted = false;
        for circuit in circuits.iter_mut() {
            if circuit.contains(i) {
                circuit.insert(*j);
                inserted = true;
            } else if circuit.contains(j) {
                circuit.insert(*i);
                inserted = true;
            }
        }

        if !inserted {
            circuits.push(HashSet::from([*i, *j]));
        }
    }

    // Merge the circuits
    let mut prev_len = 0;
    while prev_len != circuits.len() {
        prev_len = circuits.len();
        circuits = merge_circuits(circuits);
    }

    // Sort by set size
    circuits.sort_by_key(|set| std::cmp::Reverse(set.len()));

    circuits
        .iter()
        .take(3)
        .map(|set| set.len())
        .product::<usize>() as u64
}

fn part_two(input: &str) -> u64 {
    let points = parse_points(input);
    let dists = compute_distances(points);

    // Create circuits
    let &(i, j, _) = dists.first().unwrap();
    let mut circuits = vec![HashSet::from([i, j])];

    for (i, j, _) in dists.iter().skip(1) {
        let mut inserted = Vec::new();
        for (n, circuit) in circuits.iter_mut().enumerate() {
            if circuit.contains(i) {
                circuit.insert(*j);
                inserted.push(n);
            } else if circuit.contains(j) {
                circuit.insert(*i);
                inserted.push(n);
            }
        }

        if inserted.len() > 1 {
            for n in inserted {}
        }

        if inserted.is_empty() {
            circuits.push(HashSet::from([*i, *j]));
        }
    }
    todo!()
}

fn main() {
    let input = read_to_string("inputs/day_08.txt").unwrap();

    let solution = part_one(&input, 1000);
    println!("Part one solution is {solution}");

    // let solution = part_two();
    // println!("Part two solution is {solution}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "162,817,812\n\
                         57,618,57\n\
                         906,360,560\n\
                         592,479,940\n\
                         352,342,300\n\
                         466,668,158\n\
                         542,29,236\n\
                         431,825,988\n\
                         739,650,466\n\
                         52,470,668\n\
                         216,146,977\n\
                         819,987,18\n\
                         117,168,530\n\
                         805,96,715\n\
                         346,949,466\n\
                         970,615,88\n\
                         941,993,340\n\
                         862,61,35\n\
                         984,92,344\n\
                         425,690,689";

    #[test]
    fn test_part_one() {
        let expected = 40;
        let result = part_one(INPUT, 10);

        assert_eq!(result, expected)
    }

    #[test]
    fn test_part_two() {
        let expected = 25272;
        let result = part_two();

        assert_eq!(result, expected)
    }
}
