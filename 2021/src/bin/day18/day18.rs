// ---------------------------------------------------
//  Advent of Code 2021
//  Day 18: Snailfish
//
//  Copyright © 2021 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day18.in");

pub fn run() {
    println!("Result for puzzle 1: {}", puzzle_one());
    println!("Result for puzzle 2: {}", puzzle_two());
}

fn add_numbers(a: &str, b: &str) -> String {
    let mut sum = format!("[{},{}]", a, b);
    let mut is_exploded = true;
    let mut is_splited = true;

    while is_exploded || is_splited {
        if let Some(number) = explode_number(&sum) {
            sum = number;
            is_exploded = true;
            continue;
        } else {
            is_exploded = false;
        }

        if let Some(number) = split_number(&sum) {
            sum = number;
            is_splited = true;
        } else {
            is_splited = false;
        }
    }

    sum
}

fn explode_number(number: &str) -> Option<String> {
    let mut start_idx = 0;
    let mut end_idx = 0;
    let mut num_idx = 0;
    let mut count = 0;
    let mut prev_c = ' ';

    for (idx, c) in number.chars().enumerate() {
        match c {
            '[' => {
                count += 1;

                if count >= 5 {
                    start_idx = idx;
                }
            }
            ']' => {
                count -= 1;

                if count >= 4 {
                    end_idx = idx + 1;
                    break;
                }
            }
            _ => {
                if count < 5 && c.is_ascii_digit() && prev_c.is_ascii_punctuation() {
                    num_idx = idx;
                }
            }
        }

        prev_c = c;
    }

    if start_idx == 0 {
        return None;
    }

    let mut exploded_number = String::from(&number[..num_idx]);

    let (left, right) = number[start_idx + 1..end_idx - 1]
        .split_once(',')
        .unwrap();

    if num_idx > 0 {
        let num_end_idx = num_idx + number[num_idx..]
            .chars()
            .position(|c| !c.is_ascii_digit())
            .unwrap();
        let num: u32 = number[num_idx..num_end_idx]
            .parse()
            .unwrap();
        let left: u32 = left
            .parse()
            .unwrap();

        exploded_number.push_str(format!("{}", num + left).as_str());
        exploded_number.push_str(&number[num_end_idx..start_idx]);
    } else {
        exploded_number.push_str(&number[..start_idx]);
    }

    exploded_number.push('0');

    for (idx, c) in number[end_idx..].chars().enumerate() {
        match c {
            '[' => count += 1,
            ']' => count -= 1,
            _ => ()
        }

        if count == 0 {
            exploded_number.push_str(&number[end_idx..]);
            break;
        } else if c.is_ascii_digit() {
            num_idx = end_idx + idx;
            let num_end_idx = num_idx + number[num_idx..]
                .chars()
                .position(|c| !c.is_ascii_digit())
                .unwrap();
            let num: u32 = number[num_idx..num_end_idx]
                .parse()
                .unwrap();
            let right: u32 = right
                .parse()
                .unwrap();

            exploded_number.push_str(&number[end_idx..num_idx]);
            exploded_number.push_str(format!("{}", num + right).as_str());
            exploded_number.push_str(&number[num_end_idx..]);
            break;
        }
    }

    Some(exploded_number)
}

fn split_number(number: &str) -> Option<String> {
    let chars: Vec<char> = number
        .chars()
        .collect();

    for (idx, c) in chars.windows(2).enumerate() {
        if c[0].is_ascii_digit() && c[1].is_ascii_digit() {
            let mut splited_number = String::from(&number[..idx]);

            let num: u32 = number[idx..idx + 2]
                .parse()
                .unwrap();

            splited_number.push('[');
            splited_number.push_str(format!("{}", num / 2).as_str());
            splited_number.push(',');
            splited_number.push_str(format!("{}", (num + 1) / 2).as_str());
            splited_number.push(']');
            splited_number.push_str(&number[idx + 2..]);

            return Some(splited_number);
        }
    }

    None
}

fn magnitude_for(number: &str) -> u32 {
    let mut stack = Vec::new();

    for c in number.chars() {
        match c {
            '[' => stack.push(3),
            ',' => {
                let n1 = stack.pop().unwrap();
                let n2 = stack.pop().unwrap();
                stack.push(n1 * n2)
            }
            ']' => {
                let n1 = stack.pop().unwrap();
                let n2 = stack.pop().unwrap();
                stack.push(n1 * 2 + n2);
            }
            _ => stack.push(c.to_digit(10).unwrap()),
        }
    }

    stack[0]
}

// ----------
//  Puzzle 1
// ----------

fn puzzle_one() -> u32 {
    let mut numbers = INPUT.lines();
    let mut result = numbers
        .next()
        .unwrap()
        .to_string();

    for number in numbers {
        result = add_numbers(&result, number);
    }

    magnitude_for(&result)
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two() -> u32 {
    let numbers: Vec<&str> = INPUT
        .lines()
        .collect();
    let mut largest_magnitude = 0;

    for idx1 in 0..numbers.len() {
        for idx2 in 1..numbers.len() {
            if idx1 == idx2 {
                continue;
            }

            let result = add_numbers(numbers[idx1], numbers[idx2]);
            let m = magnitude_for(&result);

            largest_magnitude = largest_magnitude.max(m);

            let result = add_numbers(numbers[idx2], numbers[idx1]);
            let m = magnitude_for(&result);

            largest_magnitude = largest_magnitude.max(m);
        }
    }

    largest_magnitude
}
