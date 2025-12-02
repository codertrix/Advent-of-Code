// ---------------------------------------------------
//  Advent of Code 2025
//  Day 2: Gift Shop
//
//  Copyright © 2025 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day02.in");

pub fn run() {
    let ranges: Vec<(usize, usize)> = INPUT
        .trim()
        .split(',')
        .map(|r| r.split_once('-').unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();

    println!("Result for puzzle 1: {}", puzzle_one(&ranges));
    println!("Result for puzzle 2: {}", puzzle_two(&ranges));
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(ranges: &[(usize, usize)]) -> usize {
    let mut sum_one = 0;

    for (a, b) in ranges {
        for n in *a..=*b {
            let digit_count = n.ilog(10) + 1;
            let div = 10usize.pow(digit_count / 2);

            if n / div == n % div {
                sum_one += n;
            }
        }
    }

    sum_one
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two(ranges: &[(usize, usize)]) -> usize {
    let mut sum_two = 0;

    for (a, b) in ranges {
        for n in *a..=*b {
            let mut div = 10;

            while n > div {
                let sequence = n % div;

                if !is_valid_id(sequence, n, div) {
                    sum_two += n;
                    break;
                }

                div *= 10;
            }
        }
    }

    sum_two
}

fn is_valid_id(sequence: usize, n: usize, div: usize) -> bool {
    if sequence < div / 10 {
        return true;
    }

    let mut n = n / div;

    while n >= sequence {
        if sequence == n {
            return false;
        }

        if sequence != n % div {
            return true;
        }

        n /= div;
    }

    true
}
