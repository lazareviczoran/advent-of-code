use std::collections::HashSet;

use utils::read_to_string_in_module;

pub fn run() {
    let directions = read("input.txt");
    utils::run_solution!(|| find_distance(&directions), "part1");

    utils::run_solution!(|| find_distance_to_hq(&directions), "part2");
}

enum Direction {
    Left(usize),
    Right(usize),
}

enum Orient {
    East,
    West,
    North,
    South,
}

fn find_distance(steps: &[Direction]) -> usize {
    let mut curr_pos = (0i32, 0i32);
    let mut orient = Orient::North;
    for step in steps {
        match step {
            Direction::Left(n) => {
                orient = change_orient(orient, true);
                move_by(&mut curr_pos, &orient, *n);
            }
            Direction::Right(n) => {
                orient = change_orient(orient, false);
                move_by(&mut curr_pos, &orient, *n)
            }
        }
    }

    (curr_pos.0.abs() + curr_pos.1.abs()) as usize
}

fn find_distance_to_hq(steps: &[Direction]) -> usize {
    let mut curr_pos = (0i32, 0i32);
    let mut visited = HashSet::new();
    let mut orient = Orient::North;
    for step in steps {
        let step_size = match step {
            Direction::Left(n) => {
                orient = change_orient(orient, true);
                n
            }
            Direction::Right(n) => {
                orient = change_orient(orient, false);
                n
            }
        };
        for _ in 0..*step_size {
            if visited.contains(&curr_pos) {
                break;
            }
            visited.insert(curr_pos);
            move_by(&mut curr_pos, &orient, 1);
        }
    }

    (curr_pos.0.abs() + curr_pos.1.abs()) as usize
}

fn change_orient(or: Orient, left: bool) -> Orient {
    use Orient::*;
    if left {
        match or {
            North => West,
            South => East,
            East => North,
            West => South,
        }
    } else {
        match or {
            North => East,
            South => West,
            East => South,
            West => North,
        }
    }
}

fn move_by(pos: &mut (i32, i32), orient: &Orient, step_size: usize) {
    use Orient::*;
    match orient {
        North => pos.1 -= step_size as i32,
        South => pos.1 += step_size as i32,
        East => pos.0 += step_size as i32,
        West => pos.0 -= step_size as i32,
    }
}

fn read(filename: &str) -> Vec<Direction> {
    read_to_string_in_module!(filename)
        .split_terminator(',')
        .filter_map(|s| {
            let mut chars = s.trim().chars();
            let direction = chars.next()?;
            let step_size = chars.collect::<String>().parse::<usize>().ok()?;
            match direction {
                'R' => Some(Direction::Right(step_size)),
                'L' => Some(Direction::Left(step_size)),
                _ => unreachable!(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_tests() {
        assert_eq!(find_distance(&[Direction::Right(2), Direction::Left(3)]), 5);
        assert_eq!(
            find_distance(&[
                Direction::Right(2),
                Direction::Right(2),
                Direction::Right(2),
            ]),
            2
        );
        assert_eq!(
            find_distance(&[
                Direction::Right(5),
                Direction::Left(5),
                Direction::Right(5),
                Direction::Right(3),
            ]),
            12
        );
    }

    #[test]
    fn part2_tests() {
        assert_eq!(
            find_distance_to_hq(&[
                Direction::Right(8),
                Direction::Right(4),
                Direction::Right(4),
                Direction::Right(8),
            ]),
            4
        );
    }
}
