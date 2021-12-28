// ---------------------------------------------------
//  Advent of Code 2021
//  Day 14: Extended Polymerization
//
//  Copyright © 2021 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashMap;

const INPUT: &str = include_str!("day14.in");

pub fn run() {
    let (template, rules) = INPUT
        .split_once("\n\n")
        .unwrap();

    let rules: HashMap<(char, char), char> = rules
        .lines()
        .map(|line| {
            let (pair, element) = line
                .split_once(" -> ")
                .unwrap();
            let pair: Vec<char> = pair
                .chars()
                .collect();
            let element = element
                .chars()
                .next()
                .unwrap();
            ((pair[0], pair[1]), element)
        })
        .collect();

    let mut pairs: HashMap<(char, char), usize> = HashMap::new();
    let mut result_one = 0;

    for pair in template.chars().collect::<Vec<char>>().windows(2) {
        *pairs.entry((pair[0], pair[1])).or_default() += 1;
    }

    for step in 1..=40 {
        let mut new_pairs = HashMap::new();

        for (pair, count) in &pairs {
            if let Some(c) = rules.get(pair) {
                *new_pairs.entry((pair.0, *c)).or_default() += *count;
                *new_pairs.entry((*c, pair.1)).or_default() += *count;
            }
        }

        pairs = new_pairs;

        if step == 10 {
            result_one = result_for(template, &pairs);
        }
    }

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_for(template, &pairs));
}

fn result_for(template: &str, pairs: &HashMap<(char, char), usize>) -> usize {
    let mut elements: HashMap<char, usize> = HashMap::new();
    let mut template = template.chars();

    elements.insert(template.next().unwrap(), 1);
    elements.insert(template.last().unwrap(), 1);

    for (pair, count) in pairs {
        *elements.entry(pair.0).or_default() += *count;
        *elements.entry(pair.1).or_default() += *count;
    }

    (elements.values().max().unwrap() - elements.values().min().unwrap()) / 2
}
