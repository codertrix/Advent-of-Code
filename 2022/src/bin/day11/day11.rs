// ---------------------------------------------------
//  Advent of Code 2022
//  Day 11: Monkey in the Middle
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day11.in");

pub fn run() {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut monkey = Monkey::new();

    for line in INPUT.lines() {
        if let Some((prefix, postfix)) = line.trim().split_once(": ") {
            match prefix {
                "Starting items" => {
                    postfix
                        .split(", ")
                        .for_each(|item| {
                            monkey.items.push(item.parse().unwrap());
                        });
                }
                "Operation" => {
                    let parts: Vec<_> = postfix.split(' ').collect();

                    if let Ok(value) = parts[parts.len() - 1].parse() {
                        monkey.operation = if parts[parts.len() - 2] == "+" { Operation::Add(value) } else { Operation::Mul(value) };
                    } else {
                        monkey.operation = Operation::Old;
                    };
                }
                "Test" => {
                    let parts: Vec<_> = postfix.split(' ').collect();

                    monkey.divisor = parts[parts.len() - 1].parse().unwrap();
                }
                "If true" => {
                    let parts: Vec<_> = postfix.split(' ').collect();

                    monkey.if_true = parts[parts.len() - 1].parse().unwrap();
                }
                "If false" => {
                    let parts: Vec<_> = postfix.split(' ').collect();

                    monkey.if_false = parts[parts.len() - 1].parse().unwrap();
                    monkeys.push(monkey.clone());
                    monkey = Monkey::new();
                }
                _ => {}
            }
        };
    }

    let least_common_multiple: usize = monkeys
        .iter()
        .map(|monkey| monkey.divisor)
        .product();

    println!("Result for puzzle 1: {}", puzzle_one_two(monkeys.clone(), least_common_multiple, 20));
    println!("Result for puzzle 2: {}", puzzle_one_two(monkeys, least_common_multiple, 10000));
}

#[derive(Clone)]
enum Operation {
    Add(usize),
    Mul(usize),
    Old
}

#[derive(Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    divisor: usize,
    if_true: usize,
    if_false: usize,
    inspections: usize
}

impl Monkey {
    fn new() -> Self {
        Self {
            items: Vec::new(),
            operation: Operation::Add(0),
            divisor: 1,
            if_true: 0,
            if_false: 0,
            inspections: 0
        }
    }
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(mut monkeys: Vec<Monkey>, least_common_multiple: usize, rounds: usize) -> usize {
    for _ in 0..rounds {
        for idx in 0..monkeys.len() {
            while let Some(mut worry_level) = monkeys[idx].items.pop() {
                match monkeys[idx].operation {
                    Operation::Add(value) => worry_level += value,
                    Operation::Mul(value) => worry_level *= value,
                    Operation::Old => worry_level *= worry_level
                }

                if rounds == 20 {
                    worry_level /= 3;
                }

                if worry_level % monkeys[idx].divisor == 0 {
                    let monkey_idx = monkeys[idx].if_true;
                    monkeys[monkey_idx].items.push(worry_level % least_common_multiple);
                } else {
                    let monkey_idx = monkeys[idx].if_false;
                    monkeys[monkey_idx].items.push(worry_level % least_common_multiple);
                }

                monkeys[idx].inspections += 1;
            }
        }
    }

    let mut inspections: Vec<_> = monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .collect();

    inspections.sort_unstable_by(|a, b| b.cmp(a));

    inspections[0] * inspections[1]
}
