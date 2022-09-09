// ---------------------------------------------------
//  Advent of Code 2015
//  Day 24: It Hangs in the Balance
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day24.in");

pub fn run() {
    let packages: Vec<_> = INPUT
        .lines()
        .map(|n| n.parse().unwrap_or(0))
        .rev()
        .collect();

    println!("Result for puzzle 1: {}", puzzle_one_two(&packages, 3));
    println!("Result for puzzle 2: {}", puzzle_one_two(&packages, 4));
}

fn group_lengths_for_size(group_size: usize, packages: &[usize], count: &mut usize, mut group: (usize, usize), group_lengths: &mut Vec<(usize, usize)>) {
    if group.0 > *count {
        return;
    }

    group.1 *= packages[0];

    if group_size == packages[0] {
        group_lengths.push(group);
        *count = group.0.min(*count);
        return;
    }

    group.0 += 1;

    for idx in 1..packages.len() {
        if packages[0] + packages[idx] > group_size {
            continue;
        }

        group_lengths_for_size(group_size - packages[0], &packages[idx..], count, group, group_lengths);
    }
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(packages: &[usize], number_of_groups: usize) -> usize {
    let group_size = packages
        .iter()
        .sum::<usize>() / number_of_groups;
    let mut count = usize::MAX;
    let mut groups = Vec::new();

    for idx in 0..packages.len() {
        group_lengths_for_size(group_size, &packages[idx..], &mut count, (1, 1), &mut groups);
    }

    groups
        .into_iter()
        .min_by_key(|x| x.1)
        .unwrap().1
}
