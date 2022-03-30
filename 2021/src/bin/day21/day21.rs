// ---------------------------------------------------
//  Advent of Code 2021
//  Day 21: Dirac Dice
//
//  Copyright © 2021 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashMap;

// Input:
const START_PLAYER_ONE: usize  = 2;
const START_PLAYER_TWO: usize  = 8;
const PLAYER_ONE: usize = 0;

pub fn run() {
    println!("Result for puzzle 1: {}", puzzle_one());
    println!("Result for puzzle 2: {}", puzzle_two());
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct State {
    player: usize,
    positions: [usize; 2],
    scores: [usize; 2],
}


// ----------
//  Puzzle 1
// ----------

const WIN_SCORE_ONE: usize = 1000;

fn puzzle_one() -> usize {
    let mut die = 1;
    let mut state = State {
        player: PLAYER_ONE,
        positions: [START_PLAYER_ONE - 1, START_PLAYER_TWO - 1],
        scores: [0, 0]
    };

    loop {
        state.positions[state.player] = (state.positions[state.player] + die * 3 + 3) % 10;
        state.scores[state.player] += state.positions[state.player] + 1;
        die += 3;

        let next_player = (state.player + 1) % 2;

        if state.scores[state.player] >= WIN_SCORE_ONE {
            return state.scores[next_player] * (die - 1);
        }

        state.player = next_player;
    }
}


// ----------
//  Puzzle 2
// ----------

const WIN_SCORE_TWO: usize = 21;

fn play_game(state: State, memo: &mut HashMap<State, [usize; 2]>) -> [usize; 2] {
    if state.scores[0] >= WIN_SCORE_TWO { return [1, 0]; }
    if state.scores[1] >= WIN_SCORE_TWO { return [0, 1]; }
    if let Some(wins) = memo.get(&state) { return *wins; }

    let mut wins_sum = [0, 0];

    for roll_1 in 1..=3 {
        for roll_2 in 1..=3 {
            for roll_3 in 1..=3 {
                let mut new_state = state.clone();

                new_state.player = (state.player + 1 ) % 2;
                new_state.positions[state.player] = (state.positions[state.player] + roll_1 + roll_2 + roll_3) % 10;
                new_state.scores[state.player] += new_state.positions[state.player] + 1;

                let wins = play_game(new_state, memo);

                wins_sum[0] += wins[0];
                wins_sum[1] += wins[1];
            }
        }
    }

    memo.insert(state, wins_sum);

    wins_sum
}

fn puzzle_two() -> usize {
    let mut memo = HashMap::new();
    let state = State {
        player: PLAYER_ONE,
        positions: [START_PLAYER_ONE - 1, START_PLAYER_TWO - 1],
        scores: [0, 0]
    };

    let wins = play_game(state, &mut memo);

    wins[0].max(wins[1])
}
