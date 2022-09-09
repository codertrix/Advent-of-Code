// ---------------------------------------------------
//  Advent of Code 2015
//  Day 12: JSAbacusFramework.io
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

mod json;

use json::{Json, JsonParser};

const INPUT: &str = include_str!("day12.in");

pub fn run() {
    let results = puzzle_one_two();

    println!("Result for puzzle 1: {}", results.0);
    println!("Result for puzzle 2: {}", results.1);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn sum_of_numbers(json: &Json) -> (isize, isize) {
    let mut sum = (0, 0);
    let mut dict_counts = true;

    if let Json::Array(array) = json {
        for item in array {
            let value: (isize, isize) = match item {
                Json::Number(n) => (*n, *n),
                Json::Array(_) => sum_of_numbers(item),
                Json::Dict(_) => sum_of_numbers(item),
                Json::Str(_) => continue
            };

            sum.0 += value.0;
            sum.1 += value.1;
        }
    }

    if let Json::Dict(dict) = json {
        if dict.values().any(|v| v == &Json::Str("red")) {
            dict_counts = false;
        }

        for item in dict.values() {
            let value: (isize, isize) = match item {
                Json::Number(n) => (*n, *n),
                Json::Array(_) => sum_of_numbers(item),
                Json::Dict(_) => sum_of_numbers(item),
                Json::Str(_) => continue
            };

            sum.0 += value.0;
            sum.1 += if dict_counts { value.1 } else { 0 };
        }
    }

    sum
}

fn puzzle_one_two() -> (isize, isize) {
    let mut json_parser = JsonParser::from_str(INPUT);

    if let Some(json) = json_parser.parse() {
        return sum_of_numbers(&json);
    }

    (0, 0)
}
