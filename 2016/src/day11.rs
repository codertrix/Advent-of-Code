// ---------------------------------------------------
//  Advent of Code 2016
//  Day 11: Radioisotope Thermoelectric Generators
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::{VecDeque, HashSet};

const INPUT: &str = include_str!("day11.in");

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n------------------------------------------------");
    println!(" Day 11: Radioisotope Thermoelectric Generators");
    println!("------------------------------------------------");

    let floors = parse_input();

    println!("Result for puzzle 1: {}", puzzle_one_two(&floors, false));
    println!("Result for puzzle 2: {}", puzzle_one_two(&floors, true));

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}


// --------------
//  Puzzle 1 + 2
// --------------

fn is_save(floor: &[usize]) -> bool {
    if floor.len() < 2 {
        return true;
    }

    let microchips: Vec<usize> = floor
        .iter()
        .filter_map(|object| if *object & 1 == 1 { Some(*object) } else { None })
        .collect();

    if microchips.is_empty() || microchips.len() == floor.len() {
        return true;
    }

    microchips
        .into_iter()
        .fold(true, |valid, microchip| valid & floor.contains(&(microchip - 1)))
}

fn next_state(current: &[Vec<usize>], elevator_cargo: &[usize], current_floor: usize, next_floor: usize) -> Option<Vec<Vec<usize>>> {
    let mut next = current.to_owned();

    for (idx, object) in current[current_floor].iter().enumerate().rev() {
        if elevator_cargo.contains(object) {
            let moved_object = next[current_floor].remove(idx);
            next[next_floor].push(moved_object);
        }
    }

    next[next_floor].sort_unstable();

    if is_save(&next[current_floor]) && is_save(&next[next_floor]) {
        return Some(next);
    }

    None
}

fn parse_input() -> Vec<Vec<usize>> {
    let mut isotopes = Vec::new();
    let mut floors = Vec::new();

    for line in INPUT.lines() {
        let mut floor = Vec::new();

        for part in line.split(" a ") {
            if part.starts_with("The ") {
                continue;
            }

            let parts: Vec<_> = part
                .splitn(2, |c| c == ' ' || c == '-')
                .collect();
            let object = 2 * if let Some(isotope) = isotopes.iter().position(|s| *s == parts[0]) {
                isotope
            } else {
                isotopes.push(parts[0]);
                isotopes.len() - 1
            };

            floor.push(if parts[1].starts_with("gen") { object } else { object + 1 });
        }

        floors.push(floor);
    }

    floors
}

fn possible_cargoes(floor: &[usize]) -> Vec<Vec<usize>> {
    let mut cargoes = Vec::new();

    for (idx1, first) in floor.iter().enumerate() {
        let mut floor_first = floor.to_owned();

        floor_first.remove(idx1);

        if is_save(&floor_first) {
            cargoes.push(vec![*first]);
        }

        for (idx2, second) in floor_first.iter().enumerate().skip(idx1) {
            let mut floor_second = floor_first.clone();

            floor_second.remove(idx2);

            if is_save(&floor_second) && is_save(&[*first, *second]) {
                cargoes.push(vec![*first, *second]);
            }
        }
    }

    cargoes
}

fn signature_for(state: &[Vec<usize>]) -> Vec<(usize, usize)> {
    state
        .iter()
        .map(|floor| {
            let microchips_count = floor
                .iter()
                .filter(|object| *object & 1 == 1)
                .count();
            (microchips_count, floor.len())
        })
        .collect()
}

fn puzzle_one_two(floors: &[Vec<usize>], extra_parts: bool) -> usize {
    let mut floors = floors.to_owned();
    let mut queue = VecDeque::new();
    let mut states = HashSet::new();

    if extra_parts {
        floors[0].append(&mut vec![100, 101, 102, 103]);
    }

    queue.push_back((floors.clone(), 0, 0));

    while let Some((current, elevator, step)) = queue.pop_front() {
        if current[0].is_empty() && current[1].is_empty() && current[2].is_empty() {
            return step;
        }

        let signature = (signature_for(&current), elevator);

        if states.contains(&signature) {
            continue;
        }

        states.insert(signature);

        let cargoes = possible_cargoes(&current[elevator]);

        for cargo in cargoes {
            if elevator < 3 {
                if let Some(state) = next_state(&current, &cargo, elevator, elevator + 1) {
                    queue.push_back((state, elevator + 1, step + 1));
                }
            }

            if elevator > 0 {
                if let Some(state) = next_state(&current, &cargo, elevator, elevator - 1) {
                    queue.push_back((state, elevator - 1, step + 1));
                }
            }
        }
    }

    unreachable!("The input has no solution!");
}
