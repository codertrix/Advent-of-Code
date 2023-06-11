// ---------------------------------------------------
//  Advent of Code 2016
//  Day 20: Firewall Rules
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day20.in");

fn main() {
    let start_time = std::time::SystemTime::now();

    println!("\n------------------------");
    println!(" Day 20: Firewall Rules");
    println!("------------------------");

    let (result_one, result_two) = puzzle_one_two();

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);

    let end_time = std::time::SystemTime::now();
    println!("{:?}", end_time.duration_since(start_time).unwrap());
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two() -> (u32, u32) {
    let mut input_ranges: Vec<(u32, u32)> = INPUT
        .lines()
        .map(|line| line
             .split_once('-')
             .map_or((0, 0), |(s, e)| (s.parse().unwrap(), e.parse().unwrap())))
        .collect();

    input_ranges.sort_unstable();

    let mut ip_ranges = Vec::new();
    let mut ip_range = input_ranges[0];
    let mut result_one = None;
    let mut result_two = 0;

    for input_range in input_ranges.into_iter().skip(1) {
        if ip_range.1 < input_range.0 && input_range.0 - ip_range.1 > 1 {
            if result_one.is_none() {
                result_one = Some(ip_range.1 + 1);
            }

            result_two += input_range.0 - ip_range.1 - 1;
            ip_ranges.push(ip_range);
            ip_range = input_range;
        } else {
            ip_range.1 = ip_range.1.max(input_range.1);
        }
    }

    (result_one.unwrap(), result_two)
}
