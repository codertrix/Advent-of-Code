// ---------------------------------------------------
//  Advent of Code 2021
//  Day 4: Giant Squid
//
//  Copyright © 2021 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

const INPUT: &str = include_str!("day04.in");

#[derive(Clone)]
struct Board {
    numbers: [[usize; 5]; 5],
    won: bool,
}

impl Board {
    fn check_number(&mut self, num: usize) -> bool {
        if self.won { return false }

        for (r_idx, row) in self.numbers.iter().enumerate() {
            if let Some(c_idx) = row.iter().position(|&n| n == num) {
                self.numbers[r_idx][c_idx] = 0;

                self.won = self.numbers[r_idx]
                    .iter()
                    .sum::<usize>() == 0;
                self.won |= self.numbers
                    .iter()
                    .fold(0, |a, b| a + b[c_idx]) == 0;

                return self.won;
            }
        }

        false
    }

    fn score_factor(&self) -> usize {
        self.numbers
            .iter()
            .fold(0, |a, b| a + b
                  .iter()
                  .sum::<usize>())
    }
}

pub fn run() {
    let mut lines = INPUT.lines();
    let numbers: Vec<usize> = lines
        .next()
        .unwrap()
        .split(',')
        .flat_map(|x| x.parse())
        .collect();
    let mut boards = Vec::new();
    let mut board_numbers = [[0; 5]; 5];
    let mut row = 0;

    lines
        .skip(1)
        .for_each(|line|
            if line.is_empty() {
                board_numbers = [[0; 5]; 5];
                row = 0;
            } else {
                line
                    .split_whitespace()
                    .enumerate()
                    .for_each(|(idx, num)| board_numbers[row][idx] = num.parse().unwrap());
                row += 1;
                if row == 5 {
                    boards.push(Board { numbers: board_numbers, won: false });
                }
            });

    println!("Result for puzzle 1: {}", play(&numbers, &mut boards.clone(), 1));
    println!("Result for puzzle 2: {}", play(&numbers, &mut boards.clone(), boards.len()));
}


// --------------
//  Puzzle 1 + 2
// --------------

fn play(numbers: &[usize], boards: &mut Vec<Board>, rounds: usize) -> usize {
    let mut play_count = 0;

    for &number in numbers {
        for board in boards.iter_mut() {
            if board.check_number(number) {
                play_count += 1;
                if play_count < rounds { continue; }
                return board.score_factor() * number;
            };
        }
    }

    panic!("No winner?");
}
