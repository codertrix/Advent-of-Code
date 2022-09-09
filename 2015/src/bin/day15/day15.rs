// ---------------------------------------------------
//  Advent of Code 2015
//  Day 15: Science for Hungry People
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day15.in");

pub fn run() {
    let ingredients: Vec<_> = INPUT
        .lines()
        .map(|line| {
            let parts: Vec<_> = line
                .split([' ', ','])
                .collect();

            Ingredient {
                capacity: parts[2].parse().unwrap_or(0),
                durability: parts[5].parse().unwrap_or(0),
                flavor: parts[8].parse().unwrap_or(0),
                texture: parts[11].parse().unwrap_or(0),
                calories: parts[14].parse().unwrap_or(0)
            }
        })
        .collect();

    let (result_one, result_two) = puzzle_one_two(&ingredients);

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}

struct Ingredient {
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two(ingredients: &[Ingredient]) -> (isize, isize) {
    let mut score_one = 0;
    let mut score_two = 0;

    for a in 0..=100 {
        for b in 0..=(100 - a) {
            for c in 0..=(100 - a - b) {
                let d = 100 - a - b - c;
                let teespoons = [a, b, c, d];
                let mut properties = (0, 0, 0, 0, 0);

                for idx in 0..4 {
                    properties.0 += ingredients[idx].capacity * teespoons[idx];
                    properties.1 += ingredients[idx].durability * teespoons[idx];
                    properties.2 += ingredients[idx].flavor * teespoons[idx];
                    properties.3 += ingredients[idx].texture * teespoons[idx];
                    properties.4 += ingredients[idx].calories * teespoons[idx];
                }

                let total_score = properties.0.max(0) * properties.1.max(0) * properties.2.max(0) * properties.3.max(0);

                score_one = score_one.max(total_score);

                if properties.4 == 500 {
                    score_two = score_two.max(total_score);
                }
            }
        }
    }

    (score_one, score_two)
}
