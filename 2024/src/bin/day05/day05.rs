// ---------------------------------------------------
//  Advent of Code 2024
//  Day 5: Print Queue
//
//  Copyright © 2024 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day05.in");

pub fn run() {
    let mut rules = Vec::new();
    let mut updates = Vec::new();
    let mut lines = INPUT.lines();

    for line in lines.by_ref() {
        if let Some((a, b)) = line.split_once('|') {
            let a: usize = a.parse().unwrap();
            let b: usize = b.parse().unwrap();

            rules.push((a, b));
        } else {
            break;
        }
    }

    for line in lines {
        updates.push(
            line.split(',')
                .map(|page| page.parse().unwrap())
                .collect::<Vec<usize>>(),
        );
    }

    let (result_one, result_two) = puzzle_one_two(&rules, &updates);

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(rules: &[(usize, usize)], updates: &[Vec<usize>]) -> (usize, usize) {
    let mut result_one = 0;
    let mut result_two = 0;

    'update: for update in updates {
        let length = update.len();

        for idx in 0..length {
            let not_correct: Vec<_> = rules.iter().filter(|(_, b)| *b == update[idx]).collect();

            for (a, _) in not_correct {
                if update[idx..].iter().skip(1).any(|aa| a == aa) {
                    result_two += fix_update(update, rules);
                    continue 'update;
                }
            }
        }

        result_one += update[length / 2];
    }

    (result_one, result_two)
}

fn fix_update(update: &[usize], rules: &[(usize, usize)]) -> usize {
    let mut update = Vec::from(update);
    let length = update.len();

    for idx1 in 0..length - 1 {
        for idx2 in (idx1..length).skip(1) {
            if rules.contains(&(update[idx2], update[idx1])) {
                update.swap(idx1, idx2);
            }
        }
    }

    update[length / 2]
}
