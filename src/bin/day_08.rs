use std::{collections::HashSet, fs::read_to_string};

type PointPair = (usize, usize, f32);
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

fn compute_distances(points: &[Point]) -> Vec<PointPair> {
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

fn part_one(dists: &[PointPair], n_max_pairs: usize) -> u64 {
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

// TODO: this is extremely slow, it takes ~13 seconds
fn part_two(points: &[Point], dists: &[PointPair]) -> u64 {
    let &(i, j, _) = dists.first().unwrap();
    let mut circuits = vec![HashSet::from([i, j])];

    for (i, j, _) in dists.iter().skip(1) {
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

        let mut prev_len = 0;
        while prev_len != circuits.len() {
            prev_len = circuits.len();
            circuits = merge_circuits(circuits);
        }

        // Check if all boxes are connected
        if circuits.first().unwrap().len() == points.len() {
            let x_0 = points[*i].0[0] as u64;
            let x_1 = points[*j].0[0] as u64;
            return x_0 * x_1;
        }
    }

    0
}

fn main() {
    let input = read_to_string("inputs/day_08.txt").unwrap();
    let points = parse_points(&input);
    let dists = compute_distances(&points);

    let solution = part_one(&dists, 1000);
    println!("Part one solution is {solution}");

    let solution = part_two(&points, &dists);
    println!("Part two solution is {solution}");

    assert!(solution > 673096640)
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
        let points = parse_points(INPUT);
        let dists = compute_distances(&points);
        let result = part_one(&dists, 10);

        assert_eq!(result, expected)
    }

    #[test]
    fn test_part_two() {
        let expected = 25272;
        let points = parse_points(INPUT);
        let dists = compute_distances(&points);
        let result = part_two(&points, &dists);

        assert_eq!(result, expected)
    }
}
