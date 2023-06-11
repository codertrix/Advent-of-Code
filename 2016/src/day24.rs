// ---------------------------------------------------
//  Advent of Code 2016
//  Day 24: Air Duct Spelunking
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!("day24.in");

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n-----------------------------");
    println!(" Day 24: Air Duct Spelunking");
    println!("-----------------------------");

    let map: Vec<Vec<char>> = INPUT
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let (result_one, result_two) = puzzle_one_two(&map);

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}

fn permutations(from: &[usize]) -> Vec<Vec<usize>> {
    if from.is_empty() {
        return vec![Vec::new()];
    }

    let first = from[0];
    let rest = &from[1..];

    let rest_permutations = permutations(rest);
    let mut all_permutations = Vec::new();

    for perm in rest_permutations {
        for idx in 0..=perm.len() {
            let mut new_perm = perm.clone();
            new_perm.insert(idx, first);
            all_permutations.push(new_perm);
        }
    }

    all_permutations
}

fn steps_between(x: usize, y: usize, to: char, map: &[Vec<char>]) -> usize {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    queue.push_back((x, y, 0));

    while let Some((x, y, steps)) = queue.pop_front() {
        if seen.contains(&(x, y)) {
            continue;
        }

        seen.insert((x, y));

        if map[y][x] == to {
            return steps;
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (xx, yy) = ((x as isize + dx) as usize, (y as isize + dy) as usize);

            if map[yy][xx] != '#' {
                queue.push_back((xx, yy, steps + 1));
            }
        }
    }

    0
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(map: &[Vec<char>]) -> (usize, usize) {
    let mut locations = Vec::new();

    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c.is_ascii_digit() {
                locations.push((c.to_digit(10).unwrap(), x, y));
            }
        }
    }

    locations.sort_unstable();

    let mut matrix = vec![vec![0; locations.len()]; locations.len()];

    for from in 0..locations.len() {
        for to in from + 1..locations.len() {
            let (_, x, y) = locations[from];
            let steps = steps_between(x, y, char::from(to as u8 + 48), map);

            matrix[from][to] = steps;
            matrix[to][from] = steps;
        }
    }

    let locations: Vec<_> = locations
        .into_iter()
        .map(|loc| loc.0 as usize)
        .collect();
    let permutations = permutations(&locations[1..]);
    let mut min_steps_one = usize::MAX;
    let mut min_steps_two = usize::MAX;

    for permutation in permutations {
        let mut steps = matrix[0][permutation[0]];

        for idx in 0..permutation.len() - 1 {
            steps += matrix[permutation[idx]][permutation[idx + 1]];
        }

        min_steps_one = min_steps_one.min(steps);
        min_steps_two = min_steps_two.min(steps + matrix[permutation[permutation.len() - 1]][0]);
    }

    (min_steps_one, min_steps_two)
}
