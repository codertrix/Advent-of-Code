// ---------------------------------------------------
//  Advent of Code 2021
//  Day 24: Arithmetic Logic Unit
//
//  Copyright © 2021 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day24.in");

pub fn run() {
    let mut steps = Vec::new();
    let instructions: Vec<&str> = INPUT
        .lines()
        .collect();

    for n in 0..14 {
        let push_or_pop = instructions[n * 18 + 5]
            .split_at(6).1
            .parse::<isize>()
            .unwrap();

        let offset = instructions[n * 18 + 15]
            .split_at(6).1
            .parse::<isize>()
            .unwrap();

        steps.push((push_or_pop, offset));
    }

    println!("Result for puzzle 1: {}", model_number(&steps, [9, 8, 7, 6, 5, 4 ,3, 2 ,1]));
    println!("Result for puzzle 2: {}", model_number(&steps, [1, 2, 3, 4, 5, 6, 7, 8, 9]));
}


// --------------
//  Puzzle 1 + 2
// --------------

fn model_number(steps: &[(isize, isize)], digits: [isize; 9]) -> isize {
    let mut stack: Vec<(usize, isize)> = Vec::new();
    let mut model_number = [0; 14];

    for (idx, (push_or_pop, offset)) in steps.iter().enumerate() {
        if *push_or_pop <= 0 {
            if let Some((push_idx, offset)) = stack.pop() {
                for n1 in digits {
                    let n2 = n1 + offset + *push_or_pop;
                    if (1..=9).contains(&n2) {
                        model_number[push_idx] = n1;
                        model_number[idx] = n2;
                        break;
                    }
                }
            }
        } else {
            stack.push((idx, *offset));
        }
    }

    model_number
        .iter()
        .fold(0, |a, b| a * 10 + b)
}
