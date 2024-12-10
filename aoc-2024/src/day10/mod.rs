use std::collections::VecDeque;

use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;

pub fn run() {
    let map = read_input("input.txt");

    utils::run_solution!(|| sum_trailhead_scores(&map), "part1");
    utils::run_solution!(|| sum_trailhead_ratings(&map), "part2");
}

fn sum_trailhead_scores(map: &[Vec<u8>]) -> usize {
    find_starting_points(map)
        .iter()
        .map(|&(i, j)| bfs(map, (i, j)).len())
        .sum()
}

fn sum_trailhead_ratings(map: &[Vec<u8>]) -> usize {
    find_starting_points(map)
        .iter()
        .map(|&(i, j)| bfs(map, (i, j)).values().sum::<usize>())
        .sum()
}

fn find_starting_points(map: &[Vec<u8>]) -> Vec<(usize, usize)> {
    map.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &ch)| ch == 0)
                .map(|(j, _)| (i, j))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn bfs(map: &[Vec<u8>], (i, j): (usize, usize)) -> FxHashMap<(usize, usize), usize> {
    let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut destinations = FxHashMap::default();
    let mut queue = VecDeque::from([(i, j, FxHashSet::default())]);
    while let Some((x, y, mut visited)) = queue.pop_front() {
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));
        if map[x][y] == 9 {
            *destinations.entry((x, y)).or_insert(0) += 1;
            continue;
        }
        for &(dx, dy) in &dirs {
            let (next_x, next_y) = (x as isize + dx, y as isize + dy);
            if next_x >= 0
                && next_x < map.len() as isize
                && next_y >= 0
                && next_y < map[0].len() as isize
                && map[next_x as usize][next_y as usize] == map[x][y] + 1
            {
                queue.push_back((next_x as usize, next_y as usize, visited.clone()));
            }
        }
    }
    destinations
}

fn read_input(filename: &str) -> Vec<Vec<u8>> {
    utils::read_to_string_in_module!(filename)
        .lines()
        .map(|line| line.chars().map(|ch| (ch as u8 - b'0')).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day10::sum_trailhead_ratings;

    use super::read_input;
    use super::sum_trailhead_scores;

    #[test]
    fn p1() {
        let map = read_input("test-input.txt");
        assert_eq!(sum_trailhead_scores(&map), 36);
        let map = read_input("test-input2.txt");
        assert_eq!(sum_trailhead_scores(&map), 2);
        let map = read_input("test-input3.txt");
        assert_eq!(sum_trailhead_scores(&map), 4);
        let map = read_input("test-input4.txt");
        assert_eq!(sum_trailhead_scores(&map), 3);
        let map = read_input("test-input5.txt");
        assert_eq!(sum_trailhead_scores(&map), 1);
    }

    #[test]
    fn p2() {
        let map = read_input("test-input.txt");
        assert_eq!(sum_trailhead_ratings(&map), 81);
    }

    #[test]
    #[cfg(feature = "include-main-input")]
    fn prod() {
        use itertools::Itertools;

        use super::sum_trailhead_ratings;
        let (pt1, pt2) = utils::read_to_string_in_module!("results.txt")
            .lines()
            .map(|line| line.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        let map = read_input("input.txt");
        assert_eq!(sum_trailhead_scores(&map), pt1);
        assert_eq!(sum_trailhead_ratings(&map), pt2);
    }
}
