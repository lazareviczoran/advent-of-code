use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

pub fn run() {
    let map = read("input.txt");
    println!(
        "part1 solution: {:?}",
        find_shortest_path_length(&map, false)
    );
    println!(
        "part2 solution: {:?}",
        find_shortest_path_length(&map, true)
    );
}

fn find_shortest_path_length(map: &[Vec<char>], return_to_start: bool) -> usize {
    let nodes = map
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (y, row)| {
            row.iter().enumerate().for_each(|(x, &ch)| {
                if ch.is_ascii_digit() {
                    acc.insert(ch, (x as i64, y as i64));
                }
            });
            acc
        });

    let distances = calculate_distances(&nodes, map);
    let remaining = map.iter().fold(0, |mut acc, row| {
        for &ch in row {
            if ch != '#' && ch != '.' {
                acc |= 1 << (ch as u8 - b'0');
            }
        }
        acc
    });
    let mut q = VecDeque::from(vec![('0', 0, HashSet::new(), remaining)]);
    let mut best = usize::MAX;
    while let Some((curr_char, steps, mut visited, mut remaining)) = q.pop_back() {
        if visited.contains(&curr_char) {
            continue;
        }
        if remaining & (1 << (curr_char as u8 - b'0')) == (1 << (curr_char as u8 - b'0')) {
            remaining &= !(1 << (curr_char as u8 - b'0'));
            if remaining == 0 {
                let steps = if return_to_start {
                    steps
                        + distances
                            .get(&curr_char)
                            .unwrap()
                            .iter()
                            .find(|s| s.to == '0')
                            .unwrap()
                            .steps
                } else {
                    steps
                };
                if steps < best {
                    best = steps;
                }
                continue;
            }
        }
        visited.insert(curr_char);

        distances.get(&curr_char).unwrap().iter().for_each(|dist| {
            q.push_back((dist.to, steps + dist.steps, visited.clone(), remaining));
        });
    }
    best
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Distance {
    to: char,
    steps: usize,
}
impl Distance {
    fn new(to: char, steps: usize) -> Self {
        Self { to, steps }
    }
}
impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
impl Ord for Distance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.steps.cmp(&other.steps).then(self.to.cmp(&other.to))
    }
}

fn calculate_distances(
    nodes: &HashMap<char, (i64, i64)>,
    map: &[Vec<char>],
) -> HashMap<char, BTreeSet<Distance>> {
    let mut res = HashMap::new();
    let mut keys = nodes.keys().cloned().collect::<Vec<_>>();
    keys.sort_unstable();
    for (skip, &i) in keys.iter().enumerate() {
        for &j in keys.iter().skip(skip + 1) {
            let n1 = *nodes.get(&i).unwrap();
            let n2 = *nodes.get(&j).unwrap();
            let dist = find_shortest_path_between(n1, n2, map);
            res.entry(i)
                .or_insert_with(BTreeSet::new)
                .insert(Distance::new(j, dist));
            res.entry(j)
                .or_insert_with(BTreeSet::new)
                .insert(Distance::new(i, dist));
        }
    }

    res
}

fn find_shortest_path_between(n1: (i64, i64), n2: (i64, i64), map: &[Vec<char>]) -> usize {
    let mut q = VecDeque::from(vec![(n1, 0)]);
    let mut visited = HashSet::new();
    while let Some(((x, y), steps)) = q.pop_front() {
        if (x, y) == n2 {
            return steps;
        }
        if (0..map[0].len() as i64).contains(&x)
            && (0..map.len() as i64).contains(&y)
            && !visited.contains(&(x, y))
            && map[y as usize][x as usize] != '#'
        {
            visited.insert((x, y));
            [(1, 0), (0, 1), (-1, 0), (0, -1)].iter().for_each(|diff| {
                let (new_x, new_y) = (x + diff.0, y + diff.1);
                q.push_back(((new_x, new_y), steps + 1));
            });
        }
    }
    unreachable!()
}

fn read(filename: &str) -> Vec<Vec<char>> {
    utils::read_to_string_in_module!(filename)
        .lines()
        .map(|s| s.chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_tests() {
        let map = read("test-input.txt");
        assert_eq!(find_shortest_path_length(&map, false), 14);
    }
}
