// ---------------------------------------------------
//  Advent of Code 2015
//  Day 13: Knights of the Dinner Table
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("day13.in");

pub fn run() {
    let mut changes = HashMap::new();
    let mut persons = HashSet::new();

    for line in INPUT.lines() {
        let parts: Vec<_> = line[..line.len() - 1]
            .split_whitespace()
            .collect();

        changes.insert((parts[0], parts[10]), parts[3].parse().unwrap_or(0) * if parts[2] == "gain" { 1 } else { -1 });
        persons.insert(parts[0]);
    }

    println!("Result for puzzle 1: {}", puzzle_one_two(&persons, &changes, false));
    println!("Result for puzzle 2: {}", puzzle_one_two(&persons, &changes, true));
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

fn puzzle_one_two(persons: &HashSet<&str>, changes: &HashMap<(&str, &str), isize>, with_me: bool) -> isize {
    let mut persons: Vec<_> = persons
        .iter()
        .copied()
        .collect();

    if with_me {
        persons.insert(0, "me");
    }

    let permutations = permutations(&persons[1..]);
    let last_idx = persons.len() - 2;

    permutations
        .iter()
        .map(|permutation| {
            let mut happiness_change = 0;

            for person in permutation.windows(2) {
                happiness_change += changes.get(&(person[0], person[1])).unwrap_or(&0);
                happiness_change += changes.get(&(person[1], person[0])).unwrap_or(&0);
            }

            happiness_change += changes.get(&(persons[0], permutation[0])).unwrap_or(&0);
            happiness_change += changes.get(&(persons[0], permutation[last_idx])).unwrap_or(&0);
            happiness_change += changes.get(&(permutation[0], persons[0])).unwrap_or(&0);
            happiness_change += changes.get(&(permutation[last_idx], persons[0])).unwrap_or(&0);

            happiness_change
        })
        .max()
        .unwrap_or(0)
}
