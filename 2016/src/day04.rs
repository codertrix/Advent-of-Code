// ---------------------------------------------------
//  Advent of Code 2016
//  Day 4: Security Through Obscurity
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashMap;

const INPUT: &str = include_str!("day04.in");

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n-----------------------------------");
    println!(" Day 4: Security Through Obscurity");
    println!("-----------------------------------");

    let (result_one, result_two) = puzzle_one_two();

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}


// --------------
//  Puzzle 1 + 2
// --------------

fn decrypt_room_name(parts: &[&str], shift: u8) -> String {
    let mut room_name = String::new();

    for part in parts {
        if !room_name.is_empty() {
            room_name.push(' ');
        }

        room_name.push_str(&part
                           .chars()
                           .map(|c| {
                               let b = c as u8 + shift;
                               (if b > 122 { b - 26 } else { b }) as char
                           })
                           .collect::<String>());
    }

    room_name
}

fn puzzle_one_two() -> (usize, usize) {
    let mut sector_id_sum = 0;
    let mut storage_id = 0;

    for line in INPUT.lines() {
        let mut letters: HashMap<char, usize> = HashMap::new();
        let parts: Vec<&str> = line
            .split('-')
            .collect();
        let id_checksum = parts
            .last()
            .unwrap()
            .split_once('[')
            .unwrap();
        let sector_id: usize = id_checksum.0.parse().unwrap();
        let checksum = &id_checksum.1[0..5];

        for part in parts.iter().take(parts.len() - 1) {
            for c in part.chars() {
                *letters.entry(c).or_insert(0) += 1;
            }
        }

        let mut letters: Vec<_> = letters
            .iter()
            .map(|(k, v)| (*k, *v))
            .collect();

        letters.sort_unstable_by(|a, b| {
            if a.1 != b.1 {
                b.1.cmp(&a.1)
            } else {
                a.0.cmp(&b.0)
            }
        });

        let calculated_checksum: String = letters
            .iter()
            .take(5)
            .map(|(k, _)| *k)
            .collect();

        if checksum == calculated_checksum {
            sector_id_sum += sector_id;

            if decrypt_room_name(&parts[0..parts.len() - 1], (sector_id % 26) as u8) == "northpole object storage" {
                storage_id = sector_id;
            }
        }
    }

    (sector_id_sum, storage_id)
}
