// ---------------------------------------------------
//  Advent of Code 2024
//  Day 23: LAN Party
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("day23.in");

pub fn run() {
    let mut connections = HashMap::new();

    for line in INPUT.lines() {
        let (a, b) = line.split_once('-').unwrap();

        connections.entry(a).or_insert(HashSet::new()).insert(b);
        connections.entry(b).or_insert(HashSet::new()).insert(a);
    }

    println!("Result for puzzle 1: {}", puzzle_one(&connections));
    println!("Result for puzzle 2: {}", puzzle_two(&connections));
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(connections: &HashMap<&str, HashSet<&str>>) -> usize {
    let computers: Vec<_> = connections.keys().collect();
    let mut groups = HashSet::new();

    for (idx, &a) in computers.iter().enumerate() {
        for &b in computers.iter().skip(idx) {
            let peers_a = connections.get(a).unwrap();

            if peers_a.contains(b) {
                let peers_b = connections.get(b).unwrap();
                for c in peers_a.intersection(peers_b) {
                    let mut group = vec![a, b, c];

                    group.sort_unstable();
                    groups.insert(group);
                }
            }
        }
    }

    groups
        .into_iter()
        .filter(|peers| peers.iter().any(|peer| peer.starts_with('t')))
        .count()
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two(connections: &HashMap<&str, HashSet<&str>>) -> String {
    let computers: Vec<_> = connections.keys().collect();
    let mut password = Vec::new();

    for &a in &computers {
        let mut group = vec![*a];
        let mut peers_a = connections.get(a).unwrap().clone();

        while let Some(b) = peers_a.iter().next() {
            let peers_b = connections.get(b).unwrap();

            if peers_b.contains(a) {
                group.push(*b);
                peers_a = peers_a
                    .intersection(peers_b)
                    .copied()
                    .collect::<HashSet<_>>()
                    .clone();
            }
        }

        if password.len() < group.len() {
            password = group;
        }
    }

    password.sort_unstable();
    password.join(",")
}
