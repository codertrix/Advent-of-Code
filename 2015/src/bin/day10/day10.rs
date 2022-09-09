// ---------------------------------------------------
//  Advent of Code 2015
//  Day 10: Elves Look, Elves Say
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = "1113222113";

pub fn run() {
    let digits: Vec<_> = INPUT
        .as_bytes()
        .iter()
        .map(|b| b - 48)
        .collect();

    let result_one = puzzle_one_two(digits, 40);
    let result_two = puzzle_one_two(result_one.0, 10);

    println!("Result for puzzle 1: {}", result_one.1);
    println!("Result for puzzle 2: {}", result_two.1);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(mut digits: Vec<u8>, n: usize) -> (Vec<u8>, usize) {
    for _ in 0..n {
        let mut dd = digits.iter();
        let mut current = dd.next().unwrap();
        let mut count = 1;
        let mut new_digits = Vec::new();

        for d in dd {
            if d == current {
                count += 1;
            } else {
                new_digits.push(count);
                new_digits.push(*current);
                current = d;
                count = 1;
            }
        }

        new_digits.push(count);
        new_digits.push(*current);
        digits = new_digits;
    }

    (digits.to_vec(), digits.len())
}
