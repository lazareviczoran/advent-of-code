use std::collections::{HashMap, HashSet};

pub fn run() {
    let programs = read("input.txt");
    println!("part1 solution: {}", count_group_containing(&programs, 0));
    println!("part2 solution: {}", count_groups(&programs));
}

fn count_group_containing(programs: &HashMap<usize, HashSet<usize>>, target: usize) -> usize {
    find_group_containing_rec(programs, target, HashSet::new()).len()
}

fn count_groups(programs: &HashMap<usize, HashSet<usize>>) -> usize {
    let mut remaining = programs.keys().cloned().collect::<HashSet<_>>();
    let mut groups = Vec::new();
    while !remaining.is_empty() {
        let target = remaining.iter().next().unwrap();
        let group = find_group_containing_rec(programs, *target, HashSet::new());
        remaining = remaining.difference(&group).cloned().collect();
        groups.push(group);
    }
    groups.len()
}

fn find_group_containing_rec(
    programs: &HashMap<usize, HashSet<usize>>,
    target: usize,
    visited: HashSet<usize>,
) -> HashSet<usize> {
    let mut visited = visited;
    if visited.contains(&target) {
        return visited;
    }
    visited.insert(target);
    let visited_clone = visited.clone();
    for child in programs.get(&target).unwrap().iter() {
        visited = visited
            .union(&find_group_containing_rec(
                programs,
                *child,
                visited_clone.clone(),
            ))
            .cloned()
            .collect();
    }
    visited
}

fn read(filename: &str) -> HashMap<usize, HashSet<usize>> {
    utils::read_to_string_in_module!(filename)
        .lines()
        .map(|s| {
            let parts = s.split_terminator(" <-> ").collect::<Vec<_>>();
            let id = parts[0].parse().unwrap();
            (
                id,
                parts[1]
                    .split_terminator(", ")
                    .filter_map(|a| a.parse().ok())
                    .collect(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let programs = read("test-input.txt");
        assert_eq!(count_group_containing(&programs, 0), 6);
        assert_eq!(count_groups(&programs), 2);
    }
}
