// ---------------------------------------------------
//  Advent of Code 2015
//  Day 19: Medicine for Rudolph
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("day19.in");

pub fn run() {
    let mut molecule = "";
    let mut replacements = HashMap::new();

    for line in INPUT.lines() {
        let parts: Vec<_> = line
            .split(" => ")
            .collect();

        match parts.len() {
            1 => molecule = parts[0],
            2 => _ = replacements.insert(parts[1], parts[0]),
            _ => ()
        }
    }

    println!("Result for puzzle 1: {}", puzzle_one(molecule, &replacements));
    println!("Result for puzzle 2: {}", puzzle_two(molecule, &replacements));
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(molecule: &str, replacements: &HashMap<&str, &str>) -> usize {
    let mut new_molecules = HashSet::new();

    for (destination, source) in replacements {
        let parts: Vec<_> = molecule
            .split(source)
            .collect();

        for position in 1..parts.len() {
            let mut new_molecule = String::from(parts[0]);

            for (idx, part) in parts.iter().enumerate().skip(1) {
                new_molecule.push_str(if idx == position { destination } else { source });
                new_molecule.push_str(part);
            }

            new_molecules.insert(new_molecule);
        }
    }

    new_molecules.len()
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two(molecule: &str, replacements: &HashMap<&str, &str>) -> usize {
    let mut steps = 0;
    let mut molecule = String::from(molecule);
    let mut sorted_source: Vec<_> = replacements.keys().collect();

    sorted_source.sort_unstable_by_key(|a| std::cmp::Reverse(a.len()));

    'outer: loop {
        for &source in sorted_source.iter() {
            if let Some(destination) = replacements.get(source) {
                steps += molecule.split(source).count() - 1;
                molecule = molecule.replace(source, destination);

                if molecule == "e" {
                    break 'outer;
                }
            }
        }
    }

    steps
}
