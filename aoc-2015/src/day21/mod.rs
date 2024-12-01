use std::collections::HashSet;
use std::hash::Hash;

pub fn run() {
    let (boss, weapons, armors, rings) = read_input();
    utils::run_solution!(
        || find_least_amount_of_gold_in_win(&boss, &weapons, &armors, &rings),
        "part1"
    );

    utils::run_solution!(
        || find_most_amount_of_gold_in_loss(&boss, &weapons, &armors, &rings),
        "part2"
    );
}

fn find_least_amount_of_gold_in_win(
    boss: &Player,
    weapons: &Vec<Item>,
    armors: &Vec<Item>,
    rings: &Vec<Item>,
) -> usize {
    let mut min_cost = usize::MAX;
    let mut queue: Vec<Player> = Vec::new();
    for w in weapons {
        queue.push(Player::new(100, 0, 0, Some(*w), None));
    }
    let mut player_cache: HashSet<Player> = HashSet::new();
    let rings_combinations = get_combinations(rings);

    while !queue.is_empty() {
        let me = queue.remove(0);
        let curr_cost = me.get_current_cost();
        if player_cache.contains(&me) {
            continue;
        }
        player_cache.insert(me.clone());

        let mut game = Game::new(me.clone(), boss.clone());
        if game.am_i_the_winner() && curr_cost < min_cost {
            min_cost = curr_cost;
        }

        // prepare next states

        // without armor
        for combination in &rings_combinations {
            let mut next_me = me.clone();
            next_me.rings = combination.clone();
            queue.push(next_me);
        }

        // with armor
        for a in armors {
            // without rings
            let mut next_me = me.clone();
            next_me.armor_item = Some(*a);
            next_me.rings = vec![];
            queue.push(next_me);

            // with rings
            for combination in &rings_combinations {
                let mut next_me = me.clone();
                next_me.rings = combination.clone();
                queue.push(next_me);
            }
        }
    }

    min_cost
}

fn find_most_amount_of_gold_in_loss(
    boss: &Player,
    weapons: &[Item],
    armors: &Vec<Item>,
    rings: &Vec<Item>,
) -> usize {
    let mut max_cost = 0;
    let mut queue: Vec<Player> = Vec::new();
    for w in weapons.iter().rev() {
        queue.push(Player::new(100, 0, 0, Some(*w), None));
    }
    let mut player_cache: HashSet<Player> = HashSet::new();
    let rings_combinations = get_combinations(rings);

    while !queue.is_empty() {
        let me = queue.remove(0);
        let curr_cost = me.get_current_cost();
        if player_cache.contains(&me) {
            continue;
        }
        player_cache.insert(me.clone());

        let mut game = Game::new(me.clone(), boss.clone());
        if !game.am_i_the_winner() && curr_cost > max_cost {
            max_cost = curr_cost;
        }

        // prepare next states

        // without armor
        for combination in &rings_combinations {
            let mut next_me = me.clone();
            next_me.rings = combination.clone();
            queue.push(next_me);
        }

        // with armor
        for a in armors {
            // without rings
            let mut next_me = me.clone();
            next_me.armor_item = Some(*a);
            next_me.rings = vec![];
            queue.push(next_me);

            // with rings
            for combination in &rings_combinations {
                let mut next_me = me.clone();
                next_me.rings = combination.clone();
                queue.push(next_me);
            }
        }
    }

    max_cost
}

fn get_combinations(items: &Vec<Item>) -> Vec<Vec<Item>> {
    let mut res = Vec::new();
    for i in items {
        res.push(vec![*i]);
    }
    for i in 0..items.len() - 1 {
        for j in i + 1..items.len() {
            res.push(vec![items[i], items[j]]);
        }
    }

    res
}

#[derive(Debug)]
struct Game {
    me: Player,
    boss: Player,
}
impl Game {
    pub fn new(p1: Player, boss: Player) -> Game {
        Game { me: p1, boss }
    }

    pub fn am_i_the_winner(&mut self) -> bool {
        let mut my_damage: i32 = self.me.damage as i32 - self.boss.armor as i32;
        let mut boss_damage: i32 = self.boss.damage as i32 - self.me.armor as i32;
        // my items
        if let Some(w) = self.me.weapon {
            my_damage += w.damage as i32;
        }
        if let Some(a) = self.me.armor_item {
            boss_damage -= a.armor as i32;
        }
        for ring in self.me.rings.iter() {
            my_damage += ring.damage as i32;
            boss_damage -= ring.armor as i32;
        }

        if my_damage <= 0 {
            my_damage = 1;
        }
        if boss_damage <= 0 {
            boss_damage = 1;
        }

        while self.me.hit_points > 0 && self.boss.hit_points > 0 {
            self.boss.hit_points -= my_damage;
            if self.boss.hit_points <= 0 {
                return true;
            }
            self.me.hit_points -= boss_damage;
            if self.boss.hit_points <= 0 {
                return false;
            }
        }
        self.me.hit_points > self.boss.hit_points
    }
}

#[derive(Clone, Debug, Eq)]
struct Player {
    hit_points: i32,
    damage: usize,
    armor: usize,
    weapon: Option<Item>,
    armor_item: Option<Item>,
    rings: Vec<Item>,
}
impl Player {
    pub fn new(
        hit_points: i32,
        damage: usize,
        armor: usize,
        weapon: Option<Item>,
        armor_item: Option<Item>,
    ) -> Player {
        Player {
            hit_points,
            damage,
            armor,
            weapon,
            armor_item,
            rings: vec![],
        }
    }

    pub fn get_current_cost(&self) -> usize {
        let mut cost = 0;
        if let Some(w) = self.weapon {
            cost += w.cost;
        }
        if let Some(a) = self.armor_item {
            cost += a.cost;
        }
        for r in self.rings.iter() {
            cost += r.cost;
        }
        cost
    }
}
impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.armor == other.armor
            && self.armor_item == other.armor_item
            && self.damage == other.damage
            && self.weapon == other.weapon
            && self.rings == other.rings
            && self.hit_points == other.hit_points
    }
}

impl Hash for Player {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hit_points.hash(state);
        self.damage.hash(state);
        self.armor.hash(state);
        self.weapon.hash(state);
        self.armor_item.hash(state);
        self.rings.hash(state);
    }
}

#[derive(Clone, Debug, Copy, Eq)]
struct Item {
    cost: usize,
    damage: usize,
    armor: usize,
}
impl Item {
    pub fn new(cost: usize, damage: usize, armor: usize) -> Item {
        Item {
            cost,
            damage,
            armor,
        }
    }
}
impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.damage == other.damage && self.armor == other.armor
    }
}
impl Hash for Item {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.cost.hash(state);
        self.damage.hash(state);
        self.armor.hash(state);
    }
}

fn read_input() -> (Player, Vec<Item>, Vec<Item>, Vec<Item>) {
    let mut contents = utils::read_to_string_in_module!("input.txt");
    let mut boss_data = Vec::new();
    for s in contents.split_terminator('\n') {
        let parts: Vec<&str> = s.split_terminator(": ").collect();
        boss_data.push(parts[1].parse::<usize>().unwrap());
    }
    let boss = Player::new(boss_data[0] as i32, boss_data[1], boss_data[2], None, None);

    contents = utils::read_to_string_in_module!("weapons.txt");
    let mut weapons = Vec::new();
    for s in contents.split_terminator('\n') {
        let parts: Vec<&str> = s.split_terminator(",").collect();
        weapons.push(Item::new(
            parts[1].parse::<usize>().unwrap(),
            parts[2].parse::<usize>().unwrap(),
            parts[3].parse::<usize>().unwrap(),
        ));
    }

    contents = utils::read_to_string_in_module!("armors.txt");
    let mut armors = Vec::new();
    for s in contents.split_terminator('\n') {
        let parts: Vec<&str> = s.split_terminator(",").collect();
        armors.push(Item::new(
            parts[1].parse::<usize>().unwrap(),
            parts[2].parse::<usize>().unwrap(),
            parts[3].parse::<usize>().unwrap(),
        ));
    }

    contents = utils::read_to_string_in_module!("rings.txt");
    let mut rings = Vec::new();
    for s in contents.split_terminator('\n') {
        let parts: Vec<&str> = s.split_terminator(",").collect();
        rings.push(Item::new(
            parts[1].parse::<usize>().unwrap(),
            parts[2].parse::<usize>().unwrap(),
            parts[3].parse::<usize>().unwrap(),
        ));
    }

    (boss, weapons, armors, rings)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let mut game = Game::new(
            Player::new(8, 5, 5, None, None),
            Player::new(12, 7, 2, None, None),
        );
        assert!(game.am_i_the_winner());
    }

    #[test]
    fn get_combinations_test() {
        let first = Item::new(20, 10, 5);
        let second = Item::new(30, 15, 0);
        let third = Item::new(10, 10, 2);
        let items = vec![first, second, third];
        assert_eq!(
            get_combinations(&items),
            vec![
                vec![first],
                vec![second],
                vec![third],
                vec![first, second],
                vec![first, third],
                vec![second, third]
            ]
        )
    }
}
