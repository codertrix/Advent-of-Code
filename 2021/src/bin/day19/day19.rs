// ---------------------------------------------------
//  Advent of Code 2021
//  Day 19: Beacon Scanner
//
//  Copyright © 2021 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("day19.in");

type Position = (isize, isize, isize);

struct Scanner {
    position: Position,
    beacons: Vec<Position>,
    has_match: bool
}

impl Scanner {
    fn new() -> Self {
        Scanner {
            position: (0, 0, 0),
            beacons: Vec::new(),
            has_match: false
        }
    }

    fn roll(&mut self) {
        self.beacons
            .iter_mut()
            .for_each(|beacon| {
                let new_y = beacon.2;
                beacon.2 = -beacon.1;
                beacon.1 = new_y;
            })
    }

    fn turn(&mut self) {
        self.beacons
            .iter_mut()
            .for_each(|beacon| {
                let new_y = beacon.0;
                beacon.0 = -beacon.1;
                beacon.1 = new_y;
            })
    }

    fn align_axis(&mut self, scanner: &Scanner) {
        for _ in 0..2 {
            for _ in 0..3 {
                self.roll();

                if let Some((x, y, z)) = self.search_position(scanner) {
                    self.position = (scanner.position.0 + x, scanner.position.1 + y, scanner.position.2 + z);
                    return;
                }

                for _ in 0..3 {
                    self.turn();

                    if let Some((x, y, z)) = self.search_position(scanner) {
                        self.position = (scanner.position.0 + x, scanner.position.1 + y, scanner.position.2 + z);
                        return;
                    }
                }
            }

            self.roll();
            self.turn();
            self.roll();
        }
    }

    fn check_overlap(&mut self, scanner: &Scanner) {
        if let Some(pos) = self.search_position(scanner) {
            self.has_match = true;
            self.position = pos;
            self.align_axis(scanner);
        }
    }

    fn search_position(&self, scanner: &Scanner) -> Option<Position> {
        let mut matches: HashMap<Position, usize> = HashMap::new();

        for (x1, y1, z1) in &scanner.beacons {
            for (x2, y2, z2) in &self.beacons {
                *matches.entry((x1 - x2, y1 - y2, z1 - z2)).or_default() += 1;
            }
        }

        if let Some(pos) = matches.iter().filter(|&(_, v)| *v >= 12).map(|(k, _)| *k).next() {
            return Some(pos);
        }

        None
    }
}

impl Clone for Scanner {
    fn clone(&self) -> Self {
        Scanner {
            position: self.position,
            beacons: self.beacons.clone(),
            has_match: self.has_match
        }
    }
}

pub fn run() {
    let mut scanners = vec![Scanner::new()];
    let mut idx = 0;

    for line in INPUT.lines().skip(1) {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("---") {
            idx += 1;
            scanners.push(Scanner::new());
        } else {
            let beacon: Vec<isize> = line
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect();

            scanners[idx].beacons.push((beacon[0], beacon[1], beacon[2]));
        }
    }

    println!("Result for puzzle 1: {}", puzzle_one(&mut scanners));
    println!("Result for puzzle 2: {}", puzzle_two(&scanners));
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(scanners: &mut [Scanner]) -> usize {
    let mut matched = vec![0];
    scanners[0].has_match = true;

    while matched.len() < scanners.len() {
        'match_search:
        for to_match_idx in matched.clone() {
            for idx in 1..scanners.len() {
                if scanners[idx].has_match { continue; }

                let scanner_to_match = scanners[to_match_idx].clone();

                for _ in 0..2 {
                    for _ in 0..3 {
                        scanners[idx].roll();

                        scanners[idx].check_overlap(&scanner_to_match);
                        if scanners[idx].has_match {
                            matched.push(idx);
                            continue 'match_search;
                        }

                        for _ in 0..3 {
                            scanners[idx].turn();

                            scanners[idx].check_overlap(&scanner_to_match);
                            if scanners[idx].has_match {
                                matched.push(idx);
                                continue 'match_search;
                            }
                        }
                    }

                    scanners[idx].roll();
                    scanners[idx].turn();
                    scanners[idx].roll();
                }
            }
        }
    }

    let mut beacons = HashSet::new();

    for scanner in scanners {
        for (x, y, z) in &scanner.beacons {
            beacons.insert((scanner.position.0 + x, scanner.position.1 + y, scanner.position.2 + z));
        }
    }

    beacons.len()
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two(scanners: &[Scanner]) -> isize {
    let mut max_distance = 0;

    for point1 in scanners.iter().map(|s| s.position) {
        for point2 in scanners.iter().map(|s| s.position) {
            let distance = (point1.0 - point2.0).abs() + (point1.1 - point2.1).abs() + (point1.2 - point2.2).abs();
            max_distance = max_distance.max(distance);
        }
    }

    max_distance
}
