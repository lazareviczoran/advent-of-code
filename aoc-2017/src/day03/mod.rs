use std::collections::HashMap;

pub fn run() {
    let input = 325489;
    println!("part1 solution: {}", find_distance_to(input));
    println!("part2 solution: {}", find_first_larger_than(input));
}

fn find_distance_to(input: usize) -> usize {
    let directions = [(1, 0), (0, -1), (-1, 0), (0, 1)];
    let mut pos = (0i32, 0i32);
    let mut value = 1;
    let mut steps = 1;
    let mut curr_step = 0;
    let mut dir_idx = 0;
    let mut change_step = 0;
    while value != input {
        pos = (pos.0 + directions[dir_idx].0, pos.1 + directions[dir_idx].1);
        value += 1;

        curr_step += 1;
        change_step += 1;

        if curr_step == steps {
            dir_idx = (dir_idx + 1) % 4;
            curr_step = 0;
        }

        if change_step == steps * 2 {
            change_step = 0;
            steps += 1;
        }
    }
    (pos.0.abs() + pos.1.abs()) as usize
}

fn find_first_larger_than(input: usize) -> usize {
    let directions = [(1, 0), (0, -1), (-1, 0), (0, 1)];
    let neighbours = [
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    let mut pos = (0i32, 0i32);
    let mut steps = 1;
    let mut curr_step = 0;
    let mut dir_idx = 0;
    let mut change_step = 0;
    let mut fields = HashMap::new();
    fields.insert(pos, 1);
    loop {
        pos = (pos.0 + directions[dir_idx].0, pos.1 + directions[dir_idx].1);
        let value = neighbours
            .iter()
            .map(|&diff| *fields.get(&(pos.0 + diff.0, pos.1 + diff.1)).unwrap_or(&0))
            .sum();
        if value > input {
            return value;
        }
        fields.insert(pos, value);
        curr_step += 1;
        change_step += 1;

        if curr_step == steps {
            dir_idx = (dir_idx + 1) % 4;
            curr_step = 0;
        }

        if change_step == steps * 2 {
            change_step = 0;
            steps += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_tests() {
        assert_eq!(find_distance_to(1), 0);
        assert_eq!(find_distance_to(12), 3);
        assert_eq!(find_distance_to(23), 2);
        assert_eq!(find_distance_to(1024), 31);
    }
}
