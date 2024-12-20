use std::collections::HashSet;

pub fn run() {
    let map = read("input.txt");
    println!("part1 solution: {}", sum_risk_level(&map));
    println!("part2 solution: {}", multiply_top3_basins(&map));
}

fn find_low_points(map: &[Vec<usize>]) -> Vec<(usize, usize)> {
    let mut low_points = vec![];
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let mut positions = vec![];
            if x > 0 {
                positions.push((y, x - 1));
            }
            if x < map[0].len() - 1 {
                positions.push((y, x + 1));
            }
            if y > 0 {
                positions.push((y - 1, x));
            }
            if y < map.len() - 1 {
                positions.push((y + 1, x));
            }
            if positions.iter().all(|&(y1, x1)| map[y1][x1] > map[y][x]) {
                low_points.push((y, x));
            }
        }
    }
    low_points
}

fn sum_risk_level(map: &[Vec<usize>]) -> usize {
    find_low_points(map)
        .iter()
        .map(|&(y, x)| map[y][x] + 1)
        .sum()
}

fn multiply_top3_basins(map: &[Vec<usize>]) -> usize {
    let mut basins = find_low_points(map)
        .iter()
        .map(|pos| find_basin_size(map, pos))
        .collect::<Vec<_>>();
    basins.sort_unstable();
    basins.iter().rev().take(3).product()
}

fn find_basin_size(map: &[Vec<usize>], pos: &(usize, usize)) -> usize {
    let mut q = vec![*pos];
    let mut visited = HashSet::new();
    while !q.is_empty() {
        let (curr_y, curr_x) = q.remove(0);
        if map[curr_y][curr_x] == 9 {
            continue;
        }
        visited.insert((curr_y, curr_x));
        if curr_x > 0 && map[curr_y][curr_x - 1] > map[curr_y][curr_x] {
            q.push((curr_y, curr_x - 1));
        }
        if curr_x < map[0].len() - 1 && map[curr_y][curr_x + 1] > map[curr_y][curr_x] {
            q.push((curr_y, curr_x + 1));
        }
        if curr_y > 0 && map[curr_y - 1][curr_x] > map[curr_y][curr_x] {
            q.push((curr_y - 1, curr_x));
        }
        if curr_y < map.len() - 1 && map[curr_y + 1][curr_x] > map[curr_y][curr_x] {
            q.push((curr_y + 1, curr_x));
        }
    }
    visited.len()
}

fn read(filename: &str) -> Vec<Vec<usize>> {
    utils::read_to_string_in_module!(filename)
        .lines()
        .map(|l| l.chars().map(|ch| (ch as u8 - b'0') as usize).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{multiply_top3_basins, read, sum_risk_level};

    #[test]
    fn part1_test() {
        let map = read("test-input.txt");
        assert_eq!(sum_risk_level(&map), 15);
    }

    #[test]
    fn part2_test() {
        let map = read("test-input.txt");
        assert_eq!(multiply_top3_basins(&map), 1134);
    }
}
