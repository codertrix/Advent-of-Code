// ---------------------------------------------------
//  Advent of Code 2015
//  Day 21: RPG Simulator 20XX
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

// My input:
//   Hit Points: 109
//   Damage: 8
//   Armor: 2

struct Weapon(usize, isize);
struct Armor(usize, isize);
struct Ring(usize, isize, isize);

const WEAPONS: [Weapon; 5] = [Weapon(8, 4), Weapon(10, 5), Weapon(25, 6), Weapon(40, 7), Weapon(74, 8)];
const ARMORS: [Armor; 6] = [Armor(0, 0), Armor(13, 1), Armor(31, 2), Armor(53, 3), Armor(75, 4), Armor(102, 5)];
const RINGS: [Ring; 7] = [Ring(0, 0, 0), Ring(25, 1, 0), Ring(50, 2, 0), Ring(100, 3, 0), Ring(20, 0, 1), Ring(40, 0, 2), Ring(80, 0, 3)];

pub fn run() {
    let (result_one, result_two) = puzzle_one_two();

    println!("Result for puzzle 1: {}", result_one);
    println!("Result for puzzle 2: {}", result_two);
}


// --------------
//  Puzzle 1 + 2
// --------------

struct Player {
    hit_points: isize,
    damage: isize,
    armor: isize
}

fn player_wins(mut player: Player, mut boss: Player) -> bool {
    loop {
        boss.hit_points -= 1.max(player.damage - boss.armor);

        if boss.hit_points < 1 {
            return true;
        }

        player.hit_points -= 1.max(boss.damage - player.armor);

        if player.hit_points < 1 {
            return false;
        }
    }
}

fn puzzle_one_two() -> (usize, usize) {
    let mut gold_one = usize::MAX;
    let mut gold_two = 0;

    for Weapon(w_cost, w_damage) in WEAPONS {
        for Armor(a_cost, a_armor) in ARMORS {
            for (idx, Ring(r1_cost, r1_damage, r1_armor)) in RINGS.iter().enumerate() {
                for Ring(r2_cost, r2_damage, r2_armor) in &RINGS[idx..] {
                    if r1_cost == r2_cost && *r1_cost > 0 {
                        continue;
                    }

                    let player = Player { hit_points: 100, damage: w_damage + r1_damage + r2_damage, armor: a_armor + r1_armor + r2_armor };
                    let boss = Player { hit_points: 109, damage: 8, armor: 2 };
                    let gold_spent = w_cost + a_cost + r1_cost + r2_cost;

                    if player_wins(player, boss) {
                        gold_one = gold_one.min(gold_spent);
                    } else {
                        gold_two = gold_two.max(gold_spent);
                    }
                }
            }
        }
    }

    (gold_one, gold_two)
}
