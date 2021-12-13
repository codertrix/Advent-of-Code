// ---------------------------------------------------
//  Advent of Code 2021
//  Day 10: Syntax Scoring
//
//  Copyright © 2021 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day10.in");

pub fn run() {
    let mut syntax_error_score = 0;
    let mut incomplete_scores = Vec::new();

    for line in INPUT.lines() {
        let mut stack = Vec::new();
        let mut is_corrupt = false;

        for c in line.chars() {
            match c {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '<' => stack.push('>'),
                _ => if stack[stack.len() - 1] != c {
                    match c {
                        ')' => syntax_error_score += 3,
                        ']' => syntax_error_score += 57,
                        '}' => syntax_error_score += 1197,
                        '>' => syntax_error_score += 25137,
                        _ => panic!("Bad input!")
                    };

                    is_corrupt = true;
                    break;
                } else {
                    stack.pop();
                }
            }
        }

        if !is_corrupt {
            incomplete_scores.push(stack
                .iter()
                .rev()
                .map(|c| match c {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => unreachable!()
                } as usize)
                .fold(0, |a, b| a * 5 + b))
        }
    }

    incomplete_scores.sort_unstable();

    println!("Result for puzzle 1: {}", syntax_error_score);
    println!("Result for puzzle 2: {}", incomplete_scores[incomplete_scores.len() / 2]);
}
