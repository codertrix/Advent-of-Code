// ---------------------------------------------------
//  Advent of Code 2024
//  Day 7: Bridge Repair
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day07.in");

pub fn run() {
    let mut equations = Vec::new();

    for line in INPUT.lines() {
        let (value, numbers) = line.split_once(": ").unwrap();
        let value = value.parse().unwrap();
        let numbers = numbers.split(' ').map(|n| n.parse().unwrap()).collect();

        equations.push((value, numbers));
    }

    let (result_one, result_two) = puzzle_one_two(&equations);

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(equations: &[(usize, Vec<usize>)]) -> (usize, usize) {
    let mut result_one = 0;
    let mut result_two = 0;

    for (value, numbers) in equations {
        if is_valid_equation(*value, numbers, false) {
            result_one += *value;
        }

        if is_valid_equation(*value, numbers, true) {
            result_two += *value;
        }
    }

    (result_one, result_two)
}

fn is_valid_equation(value: usize, numbers: &[usize], with_concatenation: bool) -> bool {
    let mut values = vec![numbers[0]];

    for n in numbers.iter().skip(1) {
        let mut new_values = Vec::new();

        for v in values {
            let product = v * n;
            let sum = v + n;

            if product <= value {
                new_values.push(product);
            }

            if sum <= value {
                new_values.push(sum);
            }

            if with_concatenation {
                let concatenation = v * 10usize.pow(n.to_string().len() as u32) + n;

                if concatenation <= value {
                    new_values.push(concatenation);
                }
            }
        }

        values = new_values;
    }

    values.contains(&value)
}
