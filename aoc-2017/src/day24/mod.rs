use std::collections::{HashMap, HashSet, VecDeque};

pub fn run() {
    let map = read("input.txt");
    println!("part1 solution: {}", find_strongest_bridge_value(&map));
    println!("part2 solution: {}", find_longest_bridge_value(&map));
}

fn find_strongest_bridge_value(map: &HashMap<usize, HashSet<(usize, usize)>>) -> usize {
    let mut strenght = 0;
    find_strongest_bridge_rec(
        map,
        &mut VecDeque::new(),
        &mut strenght,
        0,
        &mut HashSet::new(),
    );
    strenght
}

fn find_strongest_bridge_rec(
    map: &HashMap<usize, HashSet<(usize, usize)>>,
    current: &mut VecDeque<(usize, (usize, usize))>,
    strength: &mut usize,
    current_strength: usize,
    visited: &mut HashSet<(usize, usize)>,
) {
    if current_strength > *strength {
        *strength = current_strength;
    }
    let free = if current.is_empty() {
        0
    } else {
        let (used, prev) = current.back().unwrap();
        if &prev.0 == used {
            prev.1
        } else {
            prev.0
        }
    };
    for item in map.get(&free).unwrap() {
        if !visited.contains(item) {
            visited.insert(*item);
            current.push_back((free, *item));
            let value = item.0 + item.1;
            find_strongest_bridge_rec(map, current, strength, current_strength + value, visited);
            current.pop_back();
            visited.remove(item);
        }
    }
}

fn find_longest_bridge_value(map: &HashMap<usize, HashSet<(usize, usize)>>) -> usize {
    let mut strenght = 0;
    find_longest_bridge_rec(
        map,
        &mut VecDeque::new(),
        &mut strenght,
        0,
        &mut 0,
        &mut HashSet::new(),
    );
    strenght
}

fn find_longest_bridge_rec(
    map: &HashMap<usize, HashSet<(usize, usize)>>,
    current: &mut VecDeque<(usize, (usize, usize))>,
    strength: &mut usize,
    current_strength: usize,
    best_length: &mut usize,
    visited: &mut HashSet<(usize, usize)>,
) {
    if &current.len() > best_length || &current.len() == best_length && current_strength > *strength
    {
        *best_length = current.len();
        *strength = current_strength;
    }
    let free = if current.is_empty() {
        0
    } else {
        let (used, prev) = current.back().unwrap();
        if &prev.0 == used {
            prev.1
        } else {
            prev.0
        }
    };
    for item in map.get(&free).unwrap() {
        if !visited.contains(item) {
            visited.insert(*item);
            current.push_back((free, *item));
            let value = item.0 + item.1;
            find_longest_bridge_rec(
                map,
                current,
                strength,
                current_strength + value,
                best_length,
                visited,
            );
            current.pop_back();
            visited.remove(item);
        }
    }
}

fn read(filename: &str) -> HashMap<usize, HashSet<(usize, usize)>> {
    let mut map = HashMap::new();
    utils::read_to_string_in_module!(filename)
        .lines()
        .for_each(|s| {
            let parts = s
                .split_terminator('/')
                .filter_map(|p| p.parse::<usize>().ok())
                .collect::<Vec<_>>();
            map.entry(parts[0])
                .or_insert_with(HashSet::new)
                .insert((parts[0], parts[1]));
            map.entry(parts[1])
                .or_insert_with(HashSet::new)
                .insert((parts[0], parts[1]));
        });
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let map = read("test-input.txt");
        assert_eq!(find_strongest_bridge_value(&map), 31);
        assert_eq!(find_longest_bridge_value(&map), 19);
    }
}
