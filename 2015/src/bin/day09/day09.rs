// ---------------------------------------------------
//  Advent of Code 2015
//  Day 9: All in a Single Night
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("day09.in");

pub fn run() {
    let mut distances = HashMap::new();
    let mut locations = HashSet::new();

    for line in INPUT.lines() {
        let line_parts: Vec<&str> = line
            .split_whitespace()
            .collect();
        let distance: usize = line_parts[4]
            .parse()
            .unwrap();

        distances.insert((line_parts[0], line_parts[2]), distance);
        distances.insert((line_parts[2], line_parts[0]), distance);
        locations.insert(line_parts[0]);
        locations.insert(line_parts[2]);
    }

    let locations: Vec<_> = locations
        .into_iter()
        .collect();
    let results = puzzle_one_two(&locations, &distances);

    println!("Result for puzzle 1: {}", results.0);
    println!("Result for puzzle 2: {}", results.1);
}

fn permutations<'a>(elements: &[&'a str]) -> Vec<Vec<&'a str>> {
    if elements.is_empty() {
        return vec![vec![]];
    }

    let mut all_permutations = Vec::new();

    for permutation in permutations(&elements[1..]) {
        for idx in 0..=permutation.len() {
            let mut new_permutation = permutation.to_vec();

            new_permutation.insert(idx, elements[0]);
            all_permutations.push(new_permutation);
        }
    }

    all_permutations
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(locations: &[&str], distances: &HashMap<(&str, &str), usize>) -> (usize, usize) {
    let tour_distances: Vec<_> = permutations(locations)
        .iter()
        .map(|tour| {
            let mut distance = 0;

            for location_pair in tour.windows(2) {
                if let Some(d) = distances.get(&(location_pair[0], location_pair[1])) {
                    distance += d;
                }
            }

            distance
        })
        .collect();

    (*tour_distances.iter().min().unwrap(), *tour_distances.iter().max().unwrap())
}
