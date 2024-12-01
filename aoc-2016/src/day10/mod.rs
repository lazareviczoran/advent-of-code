use regex::Regex;
use std::collections::HashMap;

pub fn run() {
    let mut arena = read("input.txt");
    println!(
        "part1 solution: {:?}",
        arena.find_bot_responsible_for_values(&[61, 17])
    );
    arena.run_til_end();
    println!(
        "part2 solution: {:?}",
        arena.bins.get(&0).unwrap() * arena.bins.get(&1).unwrap() * arena.bins.get(&2).unwrap()
    );
}

#[derive(Debug, Clone)]
struct Arena {
    bots: HashMap<usize, Bot>,
    rules: HashMap<usize, Rule>,
    bins: HashMap<usize, usize>,
}
impl Arena {
    fn new(bots: HashMap<usize, Bot>, rules: HashMap<usize, Rule>) -> Self {
        Self {
            bots,
            rules,
            bins: HashMap::new(),
        }
    }

    fn run_til_end(&mut self) {
        while self.bots.values().any(|b| b.values.len() == 2) {
            self.apply_single_rule();
        }
    }

    fn add_to_bot(&mut self, id: usize, value: usize) {
        let bot = self.bots.entry(id).or_insert(Bot { id, values: vec![] });
        bot.values.push(value);
        bot.values.sort_unstable();
    }

    fn apply_single_rule(&mut self) {
        let bot = self
            .bots
            .values_mut()
            .find(|b| b.values.len() == 2)
            .unwrap();
        let high = bot.values.pop().unwrap();
        let low = bot.values.pop().unwrap();
        let rule = self.rules.get(&bot.id).unwrap().clone();
        match rule.low_to {
            OutputType::Bot(i) => {
                self.add_to_bot(i, low);
            }
            OutputType::Bin(i) => {
                self.bins.insert(i, low);
            }
        }
        match rule.high_to {
            OutputType::Bot(i) => {
                self.add_to_bot(i, high);
            }
            OutputType::Bin(i) => {
                self.bins.insert(i, high);
            }
        }
    }

    fn find_bot_responsible_for_values(&mut self, values: &[usize]) -> usize {
        let mut values = values.to_vec();
        values.sort_unstable();
        loop {
            let bot = self.bots.values().find(|b| b.values.len() == 2).unwrap();
            if bot.values.len() == 2 && bot.values == values {
                return bot.id;
            }
            self.apply_single_rule();
        }
    }
}

#[derive(Debug, Clone)]
struct Bot {
    id: usize,
    values: Vec<usize>,
}

#[derive(Debug, Clone, Copy)]
enum OutputType {
    Bot(usize),
    Bin(usize),
}

#[derive(Debug, Clone)]
struct Rule {
    low_to: OutputType,
    high_to: OutputType,
}

fn read(filename: &str) -> Arena {
    let re = Regex::new(r"\d+").unwrap();
    let rule_re = Regex::new(r"(low|high)\sto\s(bot|output)").unwrap();
    let (bots, rules) = utils::read_to_string_in_module!(filename)
        .split_terminator('\n')
        .fold((HashMap::new(), HashMap::new()), |mut acc, s| {
            let caps = re
                .captures_iter(s)
                .filter_map(|c| c[0].parse::<usize>().ok())
                .collect::<Vec<_>>();
            if s.starts_with("value") {
                let bot = acc.0.entry(caps[1]).or_insert(Bot {
                    id: caps[1],
                    values: vec![],
                });
                if bot.values.is_empty() || bot.values[0] < caps[0] {
                    bot.values.push(caps[0]);
                } else {
                    bot.values.insert(0, caps[0]);
                }
            } else {
                let targets = rule_re
                    .captures_iter(s)
                    .zip(caps.iter().skip(1))
                    .map(|(target_caps, id)| {
                        if &target_caps[2] == "bot" {
                            OutputType::Bot(*id)
                        } else {
                            OutputType::Bin(*id)
                        }
                    })
                    .collect::<Vec<_>>();
                acc.1.insert(
                    caps[0],
                    Rule {
                        low_to: targets[0],
                        high_to: targets[1],
                    },
                );
            }
            acc
        });
    Arena::new(bots, rules)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_tests() {
        let mut arena = read("test-input.txt");
        assert_eq!(arena.find_bot_responsible_for_values(&[5, 2]), 2);
    }
}
