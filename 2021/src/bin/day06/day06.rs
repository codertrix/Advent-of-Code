// ---------------------------------------------------
//  Advent of Code 2021
//  Day 6: Laternfish
//
//  Copyright © 2021 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashMap;

const INPUT: &str = include_str!("day06.in");

pub fn run() {
    let counter: Vec<usize> = INPUT
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .fold(vec![0, 0, 0, 0, 0], |mut counter, fish: usize| {
            counter[fish - 1] += 1;
            counter
        });

    println!("Result for puzzle 1: {}", count_of_fish(&counter, 80));
    println!("Result for puzzle 2: {}", count_of_fish(&counter, 256));
}


// --------------
//  Puzzle 1 + 2
// --------------

fn fish_after_days(mut state: usize, days: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(&count) = cache.get(&(state, days)) {
        return count;
    }

    let previous_state = state;
    let mut count = 1;

    for day in 0..days {
        if state == 0 {
            count += fish_after_days(8, days - day - 1, cache);
            state = 6;
        } else {
            state -= 1;
        }
    }

    cache.insert((previous_state, days), count);
    count
}

fn count_of_fish(counter: &[usize], days: usize) -> usize {
    counter
        .iter()
        .enumerate()
        .map(|(state, count)| count * fish_after_days(state + 1, days, &mut HashMap::new()))
        .sum()
}
