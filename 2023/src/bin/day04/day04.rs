// ---------------------------------------------------
//  Advent of Code 2023
//  Day 4: Scratchcards
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day04.in");

pub fn run() {
    let (result_one, result_two) = puzzle_one_two();

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

fn puzzle_one_two() -> (usize, usize) {
    let mut result = 0;
    let number_of_cards = INPUT.lines().count();
    let mut scratchcards = vec![1usize; number_of_cards + 1];

    scratchcards[0] = 0;

    for line in INPUT.lines() {
        let (card, numbers) = line.split_once(':').unwrap();
        let (_, card) = card.split_once(' ').unwrap();
        let card = card.trim().parse::<usize>().unwrap();
        let mut numbers = numbers.split(' ');
        let mut winning_numbers = Vec::new();

        for word in numbers.by_ref() {
            if word == "|" {
                break;
            }

            if let Ok(number) = word.parse::<u8>() {
                winning_numbers.push(number);
            }
        }

        let mut n = 0;

        for word in numbers {
            if let Ok(number) = word.parse::<u8>() {
                if winning_numbers.contains(&number) {
                    n += 1;
                }
            }
        }

        if n > 0 {
            result += 1 << (n - 1);

            for idx in 1..=n {
                if idx + card <= number_of_cards {
                    scratchcards[idx + card] += scratchcards[card];
                }
            }
        }
    }

    (result, scratchcards.into_iter().sum())
}
