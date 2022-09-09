// ---------------------------------------------------
//  Advent of Code 2015
//  Day 7: Some Assembly Required
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashMap;

const INPUT: &str = include_str!("day07.in");

pub fn run() {
    let mut instructions = HashMap::new();

    for line in INPUT.lines() {
        let parts: Vec<&str> = line
            .split(' ')
            .collect();

        match parts.len() {
            3 => instructions.insert(parts[2], Operation::Signal(parts[0])),
            4 => instructions.insert(parts[3], Operation::Not(parts[1])),
            5 => match parts[1] {
                "AND" => instructions.insert(parts[4], Operation::And((parts[0], parts[2]))),
                "OR" => instructions.insert(parts[4], Operation::Or((parts[0], parts[2]))),
                "LSHIFT" => instructions.insert(parts[4], Operation::Lshift((parts[0], parts[2]))),
                "RSHIFT" => instructions.insert(parts[4], Operation::Rshift((parts[0], parts[2]))),
                _ => unreachable!()
            },
            _ => unreachable!()
        };
    }

    let result_one = puzzle_one(&instructions);

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", puzzle_two(&instructions, result_one));
}

enum Operation<'a> {
    Signal(&'a str),
    Not(&'a str),
    And((&'a str, &'a str)),
    Or((&'a str, &'a str)),
    Lshift((&'a str, &'a str)),
    Rshift((&'a str, &'a str)),
}

fn resolve<'a>(wire: &'a str, instructions: &HashMap<&'a str, Operation<'a>>, values: &mut HashMap<&'a str, u16>) -> u16 {
    if let Some(value) = values.get(wire) {
        return *value;
    }

    if let Some(operation) = instructions.get(wire) {
        let signal = match operation {
            Operation::Signal(s) => if let Ok(value) = s.parse::<u16>() { value } else { resolve(s, instructions, values) },
            Operation::Not(s) => if let Ok(value) = s.parse::<u16>() { !value } else { !resolve(s, instructions, values) },
            Operation::And((a, b)) => {
                let aa = if let Ok(value) = a.parse() { value } else { resolve(a, instructions, values) };
                let bb = if let Ok(value) = b.parse() { value } else { resolve(b, instructions, values) };
                aa & bb
            },
            Operation::Or((a, b)) => {
                let aa = if let Ok(value) = a.parse() { value } else { resolve(a, instructions, values) };
                let bb = if let Ok(value) = b.parse() { value } else { resolve(b, instructions, values) };
                aa | bb
            },
            Operation::Lshift((a, b)) => {
                let aa = if let Ok(value) = a.parse() { value } else { resolve(a, instructions, values) };
                let bb = if let Ok(value) = b.parse() { value } else { resolve(b, instructions, values) };
                aa << bb
            },
            Operation::Rshift((a, b)) => {
                let aa = if let Ok(value) = a.parse() { value } else { resolve(a, instructions, values) };
                let bb = if let Ok(value) = b.parse() { value } else { resolve(b, instructions, values) };
                aa >> bb
            }
        };

        values.insert(wire, signal);
        return signal;
    }

    panic!("Bad input!");
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one<'a>(instructions: &HashMap<&'a str, Operation<'a>>) -> u16 {
    resolve("a", instructions, &mut HashMap::new())
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two<'a>(instructions: &HashMap<&'a str, Operation<'a>>, signal: u16) -> u16 {
    let mut signals = HashMap::new();

    signals.insert("b", signal);
    resolve("a", instructions, &mut signals)
}
