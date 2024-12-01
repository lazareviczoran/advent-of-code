use std::collections::{HashMap, VecDeque};

pub fn run() {
    let mut items = VecDeque::from(('a'..='p').collect::<Vec<_>>());
    let moves = read("input.txt");
    println!(
        "part1 solution: {}",
        simulate_dance(&mut items.clone(), &moves)
    );
    println!(
        "part2 solution: {}",
        simulate_n_times(&mut items, &moves, 1_000_000_000)
    );
}

fn simulate_n_times(items: &mut VecDeque<char>, moves: &[Moves], n: usize) -> String {
    let mut prev_results = HashMap::new();
    let mut i = 0;
    let mut result = String::new();
    while i < n {
        i += 1;
        result = simulate_dance(items, moves);
        if prev_results.contains_key(&result) {
            break;
        }
        prev_results.insert(result.clone(), i);
    }
    if i < n {
        let first_occurence = *prev_results.get(&result).unwrap();
        let cycle_size = i - first_occurence;
        while i + cycle_size < n {
            i += cycle_size;
        }
        let remaining = n - i;
        for _ in 0..remaining {
            result = simulate_dance(items, moves);
        }
    }
    result
}

fn simulate_dance(items: &mut VecDeque<char>, moves: &[Moves]) -> String {
    for m in moves {
        match m {
            Moves::Spin(n) => items.rotate_right(*n),
            Moves::Exchange(a, b) => items.swap(*a, *b),
            Moves::Partner(a, b) => items.iter_mut().for_each(|ch| {
                if ch == a {
                    *ch = *b
                } else if ch == b {
                    *ch = *a
                }
            }),
        }
    }
    items.iter().collect()
}

enum Moves {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

fn read(filename: &str) -> Vec<Moves> {
    utils::read_to_string_in_module!(filename)
        .split_terminator(',')
        .filter_map(|s| match s {
            c if c.starts_with('s') => {
                Some(Moves::Spin(c.strip_prefix('s').unwrap().parse().unwrap()))
            }
            c if c.starts_with('x') => {
                let content = c
                    .strip_prefix('x')
                    .unwrap()
                    .split_terminator('/')
                    .filter_map(|p| p.parse().ok())
                    .collect::<Vec<_>>();
                Some(Moves::Exchange(content[0], content[1]))
            }
            c if c.starts_with('p') => {
                let content = c
                    .strip_prefix('p')
                    .unwrap()
                    .split_terminator('/')
                    .filter_map(|p| p.chars().next())
                    .collect::<Vec<_>>();
                Some(Moves::Partner(content[0], content[1]))
            }
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let mut items = VecDeque::from(('a'..='e').collect::<Vec<_>>());
        let moves = read("test-input.txt");
        assert_eq!(simulate_dance(&mut items.clone(), &moves), "baedc");
        assert_eq!(simulate_n_times(&mut items, &moves, 2), "ceadb");
    }
}
