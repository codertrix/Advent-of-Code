// ---------------------------------------------------
//  Advent of Code 2022
//  Day 20: Grove Positioning System
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day20.in");
const DECRYPTION_KEY: isize = 811589153;

pub fn run() {
    let encrypted: Vec<isize> = INPUT
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    println!("Result for puzzle 1: {}", puzzle_one(&encrypted));
    println!("Result for puzzle 2: {}", puzzle_two(&encrypted));
}

fn decrypt(mut encrypted: Vec<(usize, isize)>, mix_count: usize) -> isize {
    let length = (encrypted.len() - 1) as isize;

    for _ in 0..mix_count {
        for n_idx in 0..=length as usize {
            let old_idx = encrypted
                .iter()
                .enumerate()
                .find_map(|(idx, (nn_idx, _))| if n_idx == *nn_idx { Some(idx as isize) } else { None })
                .unwrap();
            let n = encrypted[old_idx as usize].1;
            let mut new_idx = (old_idx + n) % length;

            encrypted.remove(old_idx as usize);

            if new_idx <= 0 {
                new_idx += length;
            }

            encrypted.insert(new_idx as usize, (n_idx, n));
        }
    }

    let zero_idx = encrypted
        .iter()
        .enumerate()
        .find_map(|(idx, (_, nn))| if *nn == 0 { Some(idx) } else { None })
        .unwrap();

    [1000, 2000, 3000]
        .iter()
        .map(|offset| encrypted[(zero_idx + offset) % (length as usize + 1)].1)
        .sum()
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(encrypted: &[isize]) -> isize {
    let encrypted:Vec<_> = encrypted
        .iter()
        .enumerate()
        .map(|(idx, n)| (idx, *n))
        .collect();

    decrypt(encrypted, 1)
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two(encrypted: &[isize]) -> isize {
    let encrypted:Vec<_> = encrypted
        .iter()
        .enumerate()
        .map(|(idx, n)| (idx, *n * DECRYPTION_KEY))
        .collect();

    decrypt(encrypted, 10)
}
