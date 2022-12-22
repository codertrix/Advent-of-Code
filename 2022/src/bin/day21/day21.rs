// ---------------------------------------------------
//  Advent of Code 2022
//  Day 21: Monkey Math
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashMap;

const INPUT: &str = include_str!("day21.in");

pub fn run() {
    let mut number: HashMap<&str, isize> = HashMap::new();
    let mut math = HashMap::new();

    for line in INPUT.lines() {
        let (name, right) = line.split_once(": ").unwrap();

        if let Ok(n) = right.parse() {
            number.insert(name, n);
        } else {
            let formula: Vec<_> = right.split(' ').collect();

            math.insert(name, (formula[0], formula[1].chars().next().unwrap(), formula[2]));
        }
    }

    let (result_one, result_two) = puzzle_one_two(&number, &math);

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

struct Formula<'a> {
    monkey: &'a str,
    left: Option<Box<Formula<'a>>>,
    operator: char,
    right: Option<Box<Formula<'a>>>,
    value: Option<isize>
}

impl<'a> Formula<'a> {
    fn evaluate(&self, humn: isize) -> isize {
        if self.monkey == "humn" {
            return humn;
        }

        if let Some(n) = self.value {
            return n;
        }

        let left_value = self.left.as_ref().unwrap().evaluate(humn);
        let right_value = self.right.as_ref().unwrap().evaluate(humn);

        match self.operator {
            '+' => left_value + right_value,
            '-' => left_value - right_value,
            '*' => left_value * right_value,
            _ => left_value / right_value
        }
    }
}

fn formula_for<'a>(monkey: &'a str, number: &HashMap<&str, isize>, math: &HashMap<&'a str, (&'a str, char, &'a str)>) -> Option<Box<Formula<'a>>> {
    if let Some(value) = number.get(monkey) {
        return Some(Box::new(Formula { monkey, left: None, operator: 'V', right: None, value: Some(*value) }));
    }

    if let Some((left, operator, right)) = math.get(monkey) {
        return Some(Box::new(Formula {
            monkey,
            left: formula_for(left, number, math),
            operator: *operator,
            right: formula_for(right, number, math),
            value: None
        }));
    }

    None
}

fn puzzle_one_two(number: &HashMap<&str, isize>, math: &HashMap<&str, (&str, char, &str)>) -> (isize, isize) {
    let Some((left, operator, right)) = math.get(&"root") else { panic!("No 'root' found!") };
    let formula = Formula {
        monkey: "root",
        left: formula_for(left, number, math),
        operator: *operator,
        right: formula_for(right, number, math),
        value: None
    };
    let Some(humn) = number.get(&"humn") else { panic!("'humn not found!'") };

    let result_one = formula.evaluate(*humn);

    let left_formula = formula.left.unwrap();
    let right_value = formula.right.unwrap().evaluate(0);
    let reverse = left_formula.evaluate(0) > right_value;
    let mut min_to_yell = 0;
    let mut max_to_yell = 5_000_000_000_000;
    let mut result_two;

    loop {
        result_two = min_to_yell + (max_to_yell - min_to_yell) / 2;

        let left_value = left_formula.evaluate(result_two);

        if left_value == right_value {
            break;
        }

        if left_value < right_value {
            if reverse {
                max_to_yell = result_two;
            } else {
                min_to_yell = result_two;
            }
        } else if reverse {
            min_to_yell = result_two;
        } else {
            max_to_yell = result_two;
        }
    }

    (result_one, result_two)
}
