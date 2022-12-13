// ---------------------------------------------------
//  Advent of Code 2022
//  Day 13: Distress Signal
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::cmp::Ordering;

const INPUT: &str = include_str!("day13.in");

pub fn run() {
    let (result_one, result_two) = puzzle_one_two();

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}

#[derive(Clone, Eq, PartialEq)]
enum Msg {
    Value(u32),
    List(Vec<Msg>)
}

impl PartialOrd for Msg {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Msg {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Msg::Value(a), Msg::Value(b)) => a.cmp(b),
            (Msg::Value(a), Msg::List(_)) => Msg::List(vec![Msg::Value(*a)]).cmp(other),
            (Msg::List(_), Msg::Value(b)) => self.cmp(&Msg::List(vec![Msg::Value(*b)])),
            (Msg::List(a), Msg::List(b)) => {
                for idx in 0..a.len() {
                    let Some(_) = a.get(idx) else { return Ordering::Less; };
                    let Some(_) = b.get(idx) else { return Ordering::Greater; };

                    match a.cmp(b) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => continue,
                        Ordering::Greater => return Ordering::Greater
                    };
                }

                a.len().cmp(&b.len())
            }
        }
    }
}

fn decode_message(input: &Vec<char>, idx: &mut usize) -> Msg {
    let mut message = Vec::new();
    let mut value = None;

    while *idx < input.len() - 1 {
        *idx += 1;

        match input[*idx] {
            '[' => message.push(decode_message(input, idx)),
            ']' => {
                if let Some(a) = value {
                    message.push(Msg::Value(a));
                }
                return Msg::List(message)
            }
            ',' => {
                if let Some(a) = value {
                    message.push(Msg::Value(a));
                }
                value = None;
            }
            _ => {
                let digit = input[*idx].to_digit(10).unwrap();

                value = if let Some(a) = value {
                    Some(a * 10 + digit)
                } else {
                    Some(digit)
                }
            }
        }
    }

    Msg::List(message)
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two() -> (usize, usize) {
    let mut result_one = 0;
    let mut pair = 1;
    let mut lines = INPUT.lines();
    let mut messages = Vec::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            pair += 1;
            continue;
        }

        let left = decode_message(&line.chars().collect(), &mut 0);
        let Some(line) = lines.next() else { panic!() };
        let right = decode_message(&line.chars().collect(), &mut 0);

        if left < right {
            result_one += pair;
        }

        messages.push(left);
        messages.push(right);
    }

    let divider_one = Msg::List(vec![Msg::List(vec![Msg::Value(2)])]);
    let divider_two = Msg::List(vec![Msg::List(vec![Msg::Value(6)])]);

    messages.push(divider_one.clone());
    messages.push(divider_two.clone());
    messages.sort_unstable();

    (result_one, messages
        .iter()
        .enumerate()
        .filter_map(|(idx, m)| if *m == divider_one || *m == divider_two {
            Some(idx + 1)
        } else {
            None
        })
        .product())
}
