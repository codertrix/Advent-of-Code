// ---------------------------------------------------
//  Advent of Code 2021
//  Day 12: Passage Pathing
//
//  Copyright © 2021 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("day12.in");

pub fn run() {
    let mut caves: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in INPUT.lines() {
        let parts = line.split_once('-').unwrap();

        caves
            .entry(parts.0)
            .or_default()
            .push(parts.1);
        caves
            .entry(parts.1)
            .or_default()
            .push(parts.0);
    }

    println!("Result for puzzle 1: {}",path_count_for_node("start", false, &caves, &mut HashSet::new()));
    println!("Result for puzzle 2: {}",path_count_for_node("start", true, &caves, &mut HashSet::new()));
}


fn path_count_for_node<'a>(node: &'a str, allow_twice: bool, caves: &HashMap<&str, Vec<&str>>, seen: &mut HashSet<&'a str>) -> usize {
    if node == "end" {
        return 1;
    }

    let mut path_count = 0;

    if node.chars().next().unwrap().is_lowercase() {
        seen.insert(node);
    }

    if let Some(dest_nodes) = caves.get(node) {
        for dest_node in dest_nodes {
            let mut allow_twice = allow_twice;
            let mut seen = seen.clone();

            if seen.contains(dest_node) {
                if allow_twice && *dest_node != "start" && *dest_node != "end" {
                    allow_twice = false;
                } else {
                    continue;
                }
            }

            path_count += path_count_for_node(dest_node, allow_twice, caves, &mut seen);
        }
    }

    path_count
}
