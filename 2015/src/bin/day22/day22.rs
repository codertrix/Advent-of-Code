// ---------------------------------------------------
//  Advent of Code 2015
//  Day 22: Wizard Simulator 20XX
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

// My input:
//   Hit Points: 71
//   Damage: 10

pub fn run() {
    println!("Result for puzzle 1: {}", puzzle_one_two(false));
    println!("Result for puzzle 2: {}", puzzle_one_two(true));
}


// --------------
//  Puzzle 1 + 2
// --------------

#[derive(Clone)]
struct Game {
    player_hit_points: isize,
    player_armor: isize,
    mana_points: isize,
    mana_spent: usize,
    shield_timer: usize,
    poison_timer: usize,
    recharge_timer: usize,
    boss_hit_points: isize,
    boss_damage: isize
}

impl Game {
    fn apply_effects(&mut self, spent_manas: &mut usize) -> bool {
        if self.shield_timer > 0 {
            self.shield_timer -= 1;

            if self.shield_timer == 0 {
                self.player_armor = 0;
            }
        }

        if self.poison_timer > 0 {
            self.poison_timer -= 1;
            self.boss_hit_points -= 3;

            if self.boss_hit_points < 1 {
                *spent_manas = self.mana_spent.min(*spent_manas);
                return true;
            }
        }

        if self.recharge_timer > 0 {
            self.recharge_timer -= 1;
            self.mana_points += 101;
        }

        false
    }
}

fn next_turns(mut game: Game, spent_manas: &mut usize, is_hard: bool) {
    if game.mana_spent >= *spent_manas {
        return;
    }

    if game.apply_effects(spent_manas) {
        return;
    }

    game.player_hit_points -= 1.max(game.boss_damage - game.player_armor);

    if game.player_hit_points < 1 {
        return;
    }

    if is_hard {
        game.player_hit_points -= 1;

        if game.player_hit_points < 1 {
            return;
        }
    }

    if game.apply_effects(spent_manas) {
        return;
    }

    // Magic Missile
    if game.mana_points < 53 {
        return;
    }

    let mut next_game = game.clone();

    next_game.mana_points -= 53;
    next_game.mana_spent += 53;
    next_game.boss_hit_points -= 4;

    if next_game.boss_hit_points < 1 {
        *spent_manas = next_game.mana_spent.min(*spent_manas);
    }

    next_turns(next_game, spent_manas, is_hard);

    // Drain
    if game.mana_points < 73 {
        return;
    }

    let mut next_game = game.clone();

    next_game.mana_points -= 73;
    next_game.mana_spent += 73;
    next_game.player_hit_points += 2;
    next_game.boss_hit_points -= 2;

    if next_game.boss_hit_points < 1 {
        *spent_manas = next_game.mana_spent.min(*spent_manas);
    }

    next_turns(next_game, spent_manas, is_hard);

    // Shield
    if game.shield_timer == 0 {
        if game.mana_points < 113 {
            return;
        }

        let mut next_game = game.clone();

        next_game.mana_points -= 113;
        next_game.mana_spent += 113;
        next_game.shield_timer = 6;
        next_game.player_armor = 7;

        next_turns(next_game, spent_manas, is_hard);
    }

    // Poison
    if game.poison_timer == 0 {
        if game.mana_points < 173 {
            return;
        }

        let mut next_game = game.clone();

        next_game.mana_points -= 173;
        next_game.mana_spent += 173;
        next_game.poison_timer = 6;

        next_turns(next_game, spent_manas, is_hard);
    }

    // Recharge
    if game.recharge_timer == 0 {
        if game.mana_points < 229 {
            return;
        }

        let mut next_game = game.clone();

        next_game.mana_points -= 229;
        next_game.mana_spent += 229;
        next_game.recharge_timer = 5;

        next_turns(next_game, spent_manas, is_hard);
    }
}

fn puzzle_one_two(is_hard: bool) -> usize {
    let mut spent_manas = usize::MAX;
    let game = Game {
        player_hit_points: 50 + 10, // compensate the boss first turn
        player_armor: 0,
        mana_points: 500,
        mana_spent: 0,
        shield_timer: 0,
        poison_timer: 0,
        recharge_timer: 0,
        boss_hit_points: 71,
        boss_damage: 10
    };

    next_turns(game, &mut spent_manas, is_hard);

    spent_manas
}
