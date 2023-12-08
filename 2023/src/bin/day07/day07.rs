// ---------------------------------------------------
//  Advent of Code 2023
//  Day 7: Camel Cards
//
//  Copyright © 2023 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::cmp::Ordering;
use std::collections::HashMap;

const INPUT: &str = include_str!("day07.in");

pub fn run() {
    let mut weak_list: HashMap<char, usize> = "23456789TJQKA"
        .chars()
        .enumerate()
        .map(|(idx, c)| (c, idx + 2))
        .collect();
    let mut hands = Vec::new();

    for line in INPUT.lines() {
        let (cards, bid) = line.split_once(' ').unwrap();
        let mut cards_map: HashMap<char, u8> = HashMap::new();

        for c in cards.chars() {
            cards_map
                .entry(c)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        let mut counts: Vec<_> = cards_map.values().copied().collect();
        counts.sort_unstable_by(|a, b| b.cmp(a));

        let bid = bid.parse().unwrap();
        let type_one = hand_type(&counts);

        if let Some(j_count) = cards_map.remove(&'J') {
            if j_count < 5 {
                counts = cards_map.values().copied().collect();
                counts.sort_unstable_by(|a, b| b.cmp(a));
                counts[0] += j_count;
            }
        }

        let type_two = hand_type(&counts);

        hands.push(Hand {
            cards,
            bid,
            type_one,
            type_two,
        });
    }

    println!("Result for puzzle 1: {}", puzzle_one(&mut hands, &weak_list));
    weak_list.insert('J', 1);
    println!("Result for puzzle 2: {}", puzzle_two(&mut hands, &weak_list));
}

#[derive(Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

struct Hand {
    cards: &'static str,
    bid: usize,
    type_one: HandType,
    type_two: HandType,
}

fn hand_type(counts: &[u8]) -> HandType {
    match counts.len() {
        1 => HandType::FiveOfAKind,
        2 => {
            if counts[0] == 4 {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            }
        }
        3 => {
            if counts[0] == 3 {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPair
            }
        }
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => panic!("Something is wrong with the input!"),
    }
}


// ----------
//  Puzzle 1
// ----------

fn puzzle_one(hands: &mut [Hand], weak_list: &HashMap<char, usize>) -> usize {
    hands.sort_unstable_by(|a, b| {
        if a.type_one != b.type_one {
            return a.type_one.cmp(&b.type_one);
        }

        for (ca, cb) in a.cards.chars().zip(b.cards.chars()) {
            let wa = weak_list.get(&ca).unwrap();
            let wb = weak_list.get(&cb).unwrap();

            if wa != wb {
                return wa.cmp(wb);
            }
        }

        Ordering::Equal
    });

    hands
        .iter()
        .enumerate()
        .fold(0, |x, (idx, hand)| x + hand.bid * (idx + 1))
}


// ----------
//  Puzzle 2
// ----------

fn puzzle_two(hands: &mut [Hand], weak_list: &HashMap<char, usize>) -> usize {
    hands.sort_unstable_by(|a, b| {
        if a.type_two != b.type_two {
            return a.type_two.cmp(&b.type_two);
        }

        for (ca, cb) in a.cards.chars().zip(b.cards.chars()) {
            let wa = weak_list.get(&ca).unwrap();
            let wb = weak_list.get(&cb).unwrap();

            if wa != wb {
                return wa.cmp(wb);
            }
        }

        Ordering::Equal
    });

    hands
        .iter()
        .enumerate()
        .fold(0, |x, (idx, hand)| x + hand.bid * (idx + 1))
}
