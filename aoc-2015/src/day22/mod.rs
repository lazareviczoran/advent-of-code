use std::collections::HashMap;
use std::hash::Hash;

pub fn run() {
    let boss = read_input();
    let me = Player::new(50, 0, 500);
    let spells = vec![
        Spell::new(53, 4, 0, 0, 0, 0),
        Spell::new(73, 2, 0, 2, 0, 0),
        Spell::new(113, 0, 7, 0, 0, 6),
        Spell::new(173, 3, 0, 0, 0, 6),
        Spell::new(229, 0, 0, 0, 101, 5),
    ];

    utils::run_solution!(
        || find_least_amount_of_mana_in_win(&me, &boss, &spells, false),
        "part1"
    );

    utils::run_solution!(
        || find_least_amount_of_mana_in_win(&me, &boss, &spells, true),
        "part2"
    );
}

fn find_least_amount_of_mana_in_win(
    initial_me: &Player,
    boss: &Player,
    spells: &Vec<Spell>,
    hard_difficulty: bool,
) -> usize {
    let mut queue: Vec<(Game, usize)> = Vec::new();
    queue.push((Game::new(initial_me.clone(), boss.clone()), 0));
    let mut min_cost = usize::MAX;

    while let Some((mut game, curr_cost)) = queue.pop() {
        game.is_my_turn = !game.is_my_turn;
        if game.is_my_turn && hard_difficulty {
            game.me.hit_points -= 1;
            if game.me.hit_points <= 0 {
                continue;
            }
        }

        let mut spell_damage = 0i32;
        let mut spell_armor = 0;
        // apply effects
        for (s, _) in game.me.active_spells_durations.iter() {
            spell_damage += s.damage as i32;
            spell_armor += s.armor as i32;
            game.me.hit_points += s.heals as i32;
            game.me.mana += s.mana;
        }

        game.boss.hit_points -= spell_damage;
        if game.boss.hit_points <= 0 {
            if min_cost > curr_cost {
                min_cost = curr_cost;
            }
            continue;
        }

        game.me.active_spells_durations = game
            .me
            .active_spells_durations
            .into_iter()
            .map(|(k, v)| (k, v - 1))
            .filter(|&(_, v)| v > 0)
            .collect();

        if game.is_my_turn {
            // players turn
            // prepare next states
            for s in spells {
                // try using spells that aren't used currently
                let active_spell = game.me.active_spells_durations.get(s);
                if game.me.mana >= s.cost && active_spell.is_none() {
                    let mut next_game = game.clone();
                    next_game.me.mana -= s.cost;
                    next_game.me.used_spells.push(*s);
                    let new_cost = curr_cost + s.cost;
                    if s.duration == 0 {
                        next_game.me.hit_points += s.heals as i32;
                        next_game.boss.hit_points -= s.damage as i32;

                        if next_game.boss.hit_points <= 0 {
                            if min_cost > new_cost {
                                min_cost = new_cost;
                            }
                        } else {
                            queue.push((next_game, new_cost));
                        }
                    } else {
                        next_game.me.active_spells_durations.insert(*s, s.duration);
                        queue.push((next_game, new_cost));
                    }
                }
            }
        } else {
            // bosses turn
            let mut boss_damage = game.boss.damage as i32 - spell_armor;
            if boss_damage <= 0 {
                boss_damage = 1;
            }
            game.me.hit_points -= boss_damage;
            if game.me.hit_points > 0 {
                queue.push((game.clone(), curr_cost));
            }
        }
    }
    min_cost
}

#[derive(Debug, Clone)]
struct Game {
    me: Player,
    boss: Player,
    is_my_turn: bool,
}
impl Game {
    pub fn new(p1: Player, boss: Player) -> Game {
        Game {
            me: p1,
            boss,
            is_my_turn: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Player {
    hit_points: i32,
    damage: usize,
    mana: usize,
    used_spells: Vec<Spell>,
    active_spells_durations: HashMap<Spell, usize>,
}
impl Player {
    pub fn new(hit_points: i32, damage: usize, mana: usize) -> Player {
        Player {
            hit_points,
            damage,
            mana,
            used_spells: Vec::new(),
            active_spells_durations: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq)]
struct Spell {
    cost: usize,
    damage: usize,
    armor: usize,
    heals: usize,
    mana: usize,
    duration: usize,
}
impl Spell {
    pub fn new(
        cost: usize,
        damage: usize,
        armor: usize,
        heals: usize,
        mana: usize,
        duration: usize,
    ) -> Spell {
        Spell {
            cost,
            damage,
            armor,
            heals,
            mana,
            duration,
        }
    }
}

fn read_input() -> Player {
    let contents = utils::read_to_string_in_module!("input.txt");
    let mut boss_data = Vec::new();
    for s in contents.split_terminator('\n') {
        let parts: Vec<&str> = s.split_terminator(": ").collect();
        boss_data.push(parts[1].parse::<usize>().unwrap());
    }
    Player::new(boss_data[0] as i32, boss_data[1], 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test1() {
        let spells = vec![
            Spell::new(53, 4, 0, 0, 0, 0),
            Spell::new(73, 2, 0, 2, 0, 0),
            Spell::new(113, 0, 7, 0, 0, 6),
            Spell::new(173, 3, 0, 0, 0, 6),
            Spell::new(229, 0, 0, 0, 101, 5),
        ];

        let me = Player::new(10, 0, 250);
        let boss = Player::new(13, 8, 0);
        assert_eq!(
            find_least_amount_of_mana_in_win(&me, &boss, &spells, false),
            226
        );
    }

    #[test]
    fn part1_test2() {
        let spells = vec![
            Spell::new(53, 4, 0, 0, 0, 0),
            Spell::new(73, 2, 0, 2, 0, 0),
            Spell::new(113, 0, 7, 0, 0, 6),
            Spell::new(173, 3, 0, 0, 0, 6),
            Spell::new(229, 0, 0, 0, 101, 5),
        ];

        let me = Player::new(10, 0, 250);
        let boss = Player::new(14, 8, 0);
        assert_eq!(
            find_least_amount_of_mana_in_win(&me, &boss, &spells, false),
            641
        );
    }
}
