// ---------------------------------------------------
//  Advent of Code 2023
//  Day 9: Mirage Maintenance
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day09.in");

pub fn run() {
    let (result_one, result_two) = puzzle_one_two();
    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two() -> (isize, isize) {
    let mut result_one = 0;
    let mut result_two = 0;

    for line in INPUT.lines() {
        let mut dataset: Vec<isize> = line.split(' ').map(|v| v.parse().unwrap()).collect();
        let mut sets = Vec::new();

        'outer: loop {
            sets.push(dataset);
            dataset = sets[sets.len() - 1]
                .windows(2)
                .map(|v| v[1] - v[0])
                .collect();

            for v in &dataset {
                if *v != 0 {
                    continue 'outer;
                }
            }
            break;
        }

        let mut next = 0;
        let mut first = 0;

        for idx in (0..sets.len()).rev() {
            next += sets[idx].iter().last().unwrap();
            first = sets[idx][0] - first;
        }

        result_one += next;
        result_two += first;
    }

    (result_one, result_two)
}
