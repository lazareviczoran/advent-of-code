use std::collections::{HashMap, HashSet, VecDeque};

pub fn run() {
    let fav_num = 1364;
    let mut map = HashMap::new();
    println!(
        "part1 solution: {}",
        find_shortest_path_length(fav_num, (1, 1), (31, 39), &mut map)
    );

    println!(
        "part2 solution: {}",
        find_distinct_location_in_n_steps(fav_num, (1, 1), &mut map, 50)
    );
}

fn find_shortest_path_length(
    fav_num: usize,
    from: (usize, usize),
    to: (usize, usize),
    map: &mut HashMap<(usize, usize), bool>,
) -> usize {
    let mut q = VecDeque::from(vec![(from, 0, HashSet::new())]);
    loop {
        let ((curr_x, curr_y), steps, mut visited) = q.pop_front().unwrap();
        if visited.contains(&(curr_x, curr_y)) {
            continue;
        }
        if (curr_x, curr_y) == to {
            return steps;
        }
        visited.insert((curr_x, curr_y));
        if curr_x > 0 && is_open(map, curr_x - 1, curr_y, fav_num) {
            q.push_back(((curr_x - 1, curr_y), steps + 1, visited.clone()));
        }
        if curr_y > 0 && is_open(map, curr_x, curr_y - 1, fav_num) {
            q.push_back(((curr_x, curr_y - 1), steps + 1, visited.clone()));
        }
        if is_open(map, curr_x + 1, curr_y, fav_num) {
            q.push_back(((curr_x + 1, curr_y), steps + 1, visited.clone()));
        }
        if is_open(map, curr_x, curr_y + 1, fav_num) {
            q.push_back(((curr_x, curr_y + 1), steps + 1, visited));
        }
    }
}

fn find_distinct_location_in_n_steps(
    fav_num: usize,
    from: (usize, usize),
    map: &mut HashMap<(usize, usize), bool>,
    steps_limit: usize,
) -> usize {
    let mut visited = HashSet::new();
    let mut q = VecDeque::from(vec![(from, 0)]);
    while !q.is_empty() {
        let ((curr_x, curr_y), steps) = q.pop_front().unwrap();
        if steps > steps_limit {
            continue;
        }
        visited.insert((curr_x, curr_y));
        if curr_x > 0
            && is_open(map, curr_x - 1, curr_y, fav_num)
            && !visited.contains(&(curr_x - 1, curr_y))
        {
            q.push_back(((curr_x - 1, curr_y), steps + 1));
        }
        if curr_y > 0
            && is_open(map, curr_x, curr_y - 1, fav_num)
            && !visited.contains(&(curr_x, curr_y - 1))
        {
            q.push_back(((curr_x, curr_y - 1), steps + 1));
        }
        if is_open(map, curr_x + 1, curr_y, fav_num) && !visited.contains(&(curr_x + 1, curr_y)) {
            q.push_back(((curr_x + 1, curr_y), steps + 1));
        }
        if is_open(map, curr_x, curr_y + 1, fav_num) && !visited.contains(&(curr_x, curr_y + 1)) {
            q.push_back(((curr_x, curr_y + 1), steps + 1));
        }
    }
    visited.len()
}

fn is_open(map: &mut HashMap<(usize, usize), bool>, x: usize, y: usize, fav_num: usize) -> bool {
    if let Some(is_open_field) = map.get(&(x, y)) {
        return *is_open_field;
    }
    let val = x * x + 3 * x + 2 * x * y + y + y * y + fav_num;
    let is_open_field = val.count_ones() % 2 == 0;
    map.insert((x, y), is_open_field);
    is_open_field
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(
            find_shortest_path_length(10, (1, 1), (7, 4), &mut HashMap::new()),
            11
        );
    }
}
