// ---------------------------------------------------
//  Advent of Code 2024
//  Day 9: Disk Fragmenter
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day09.in");

pub fn run() {
    let mut idx: usize = 0;
    let mut is_file = true;

    let disk: Vec<(Option<usize>, usize)> = INPUT
        .trim()
        .chars()
        .map(|c| {
            let id = if is_file { Some(idx) } else { None };
            let length = c.to_digit(10).unwrap() as usize;

            if is_file {
                idx += 1;
            }

            is_file = !is_file;
            (id, length)
        })
        .collect();

    println!("Result for puzzle 1: {}", puzzle_one(&disk));
    println!("Result for puzzle 2: {}", puzzle_two(&disk));
}

fn checksum(disk: &[(Option<usize>, usize)]) -> usize {
    let mut idx = 0;
    let mut checksum = 0;

    for (id, length) in disk {
        if let Some(id) = id {
            for _ in 0..*length {
                checksum += idx * id;
                idx += 1;
            }
        } else {
            idx += length;
        }
    }

    checksum
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(disk: &[(Option<usize>, usize)]) -> usize {
    let mut compacted_disk = Vec::new();
    let mut back_idx = disk.len() - 1;
    let (mut back_id, mut back_len) = disk[back_idx];

    for idx in 0..disk.len() {
        if idx >= back_idx {
            if back_len > 0 {
                compacted_disk.push((back_id, back_len));
            }
            break;
        }

        let (id, mut length) = disk[idx];

        if id.is_some() {
            compacted_disk.push((id, length));
        } else {
            if back_len == 0 {
                back_idx -= 2;
                (back_id, back_len) = disk[back_idx];
            }
            while length > back_len {
                compacted_disk.push((back_id, back_len));
                length -= back_len;
                back_idx -= 2;
                (back_id, back_len) = disk[back_idx];
            }

            if length > 0 {
                compacted_disk.push((back_id, length));
                back_len -= length;
            }
        }
    }

    checksum(&compacted_disk)
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two(disk: &[(Option<usize>, usize)]) -> usize {
    let mut disk = disk.to_owned();

    disk.push((None, 0));

    for back_idx in (2..disk.len()).rev() {
        let (back_id, back_len) = disk[back_idx];

        if back_id.is_none() {
            continue;
        }

        for idx in 1..back_idx {
            if disk[idx].0.is_some() || disk[idx].1 < back_len {
                continue;
            }

            disk[back_idx].0 = None;

            if disk[back_idx + 1].0.is_none() {
                disk[back_idx].1 += disk[back_idx + 1].1;
                disk.remove(back_idx + 1);
            }

            if disk[back_idx - 1].0.is_none() {
                disk[back_idx].1 += disk[back_idx - 1].1;
                disk.remove(back_idx - 1);
            }

            disk[idx].1 -= back_len;

            if disk[idx].1 == 0 {
                disk.remove(idx);
            }

            disk.insert(idx, (back_id, back_len));
            break;
        }
    }

    checksum(&disk)
}
