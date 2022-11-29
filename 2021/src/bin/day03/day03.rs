// ---------------------------------------------------
//  Advent of Code 2021
//  Day 3: Binary Diagnostic
//
//  Copyright © 2021 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day03.in");

pub fn run() {
    println!("Result for puzzle 1: {}", puzzle_one());
    println!("Result for puzzle 2: {}", puzzle_two());
}


// -----------
//  Puzzle 1
// -----------

fn puzzle_one() -> usize {
    let numbers: Vec<&str> = INPUT
        .lines()
        .collect();
    let threshold = numbers.len() / 2;
    let mut bit_counts = [0; 12];

    for number in numbers {
        number
            .chars()
            .enumerate()
            .for_each(|(idx, bit)| if bit == '1' { bit_counts[idx] += 1 });
    }

    let gamma_rate = bit_counts
        .iter()
        .fold(0, |x, &count| x * 2 + usize::from(count > threshold));
    let epsilon_rate = gamma_rate ^ 0b111111111111;

    gamma_rate * epsilon_rate
}


// -----------
//  Puzzle 2
// -----------

fn puzzle_two() -> usize {
    fn common_value<C>(numbers: &str, compare: C) -> usize where C: Fn(usize, usize) -> bool {
        let mut numbers: Vec<&str> = numbers.lines().collect();
        let mut idx = 0;

        while numbers.len() > 1 {
            let mut ones = Vec::new();
            let mut zeros = Vec::new();

            for number in numbers {
                if number.chars().nth(idx) == Some('1') {
                    ones.push(number);
                } else {
                    zeros.push(number);
                }
            }

            numbers = if compare(ones.len(), zeros.len()) { ones } else { zeros };
            idx += 1;
        }

        usize::from_str_radix(numbers[0], 2).unwrap()
    }

    let osygen_generator_rating = common_value(INPUT, |a, b| a >= b);
    let co2_scrubber_rating = common_value(INPUT, |a, b| a < b);

    osygen_generator_rating * co2_scrubber_rating
}
