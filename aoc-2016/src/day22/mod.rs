use std::collections::{HashSet, VecDeque};

pub fn run() {
    let nodes = read("input.txt");
    println!("part1 solution: {:?}", find_viable_pairs_count(&nodes));

    println!("part2 solution: {:?}", find_shortest_path_lenght(&nodes));
}

type Node = (String, usize, usize, usize, usize);
type Grid = Vec<Vec<(usize, usize)>>;
type Position = (i64, i64);

fn find_shortest_path_lenght(nodes: &[Node]) -> usize {
    let grid = nodes_to_grid(nodes);
    let p2_state = reach_target1(grid);

    let target = (0, 0);
    let mut q = VecDeque::from(vec![(
        p2_state.0,
        p2_state.1,
        p2_state.2,
        p2_state.3,
        HashSet::new(),
    )]);
    while !q.is_empty() {
        let (curr_grid, (x, y), data_pos, steps, mut visited) = q.pop_front().unwrap();
        if (x, y) == target {
            return steps + 1; // additional swap to move data to target
        } else if (x, y) == (data_pos.0 - 1, data_pos.1) {
            let next_grid = swap_disk_content(&curr_grid, (x, y), (x + 1, y));
            q.push_front((next_grid, (x + 1, y), (x, y), steps + 1, HashSet::new()));
            continue;
        }
        visited.insert((x, y));
        for diff in [(1, 0), (0, -1), (0, 1), (-1, 0)].iter() {
            let (next_x, next_y) = (x + diff.0, y + diff.1);
            if (0..curr_grid[0].len() as i64).contains(&next_x)
                && (0..curr_grid.len() as i64).contains(&next_y)
                && !visited.contains(&(next_x, next_y))
                && curr_grid[next_y as usize][next_x as usize].0
                    <= curr_grid[y as usize][x as usize].1
                && (next_x, next_y) != data_pos
            {
                let next_grid = swap_disk_content(&curr_grid, (x, y), (next_x, next_y));
                if cmp_diff((x, y), (next_x, next_y), target) <= 1
                    && is_in_neighbourhood((next_x, next_y), data_pos)
                {
                    q.push_front((
                        next_grid,
                        (next_x, next_y),
                        data_pos,
                        steps + 1,
                        visited.clone(),
                    ));
                } else {
                    q.push_back((
                        next_grid,
                        (next_x, next_y),
                        data_pos,
                        steps + 1,
                        visited.clone(),
                    ));
                }
            }
        }
    }
    unreachable!()
}

fn reach_target1(grid: Grid) -> (Grid, Position, Position, usize) {
    let start_pos = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, col)| {
                if col.0 == 0 {
                    Some((x as i64, y as i64))
                } else {
                    None
                }
            })
        })
        .unwrap();
    let target = (grid[0].len() as i64 - 1, 0);
    let mut visited = HashSet::new();
    let mut q = VecDeque::from(vec![(grid, start_pos, start_pos, 0)]);
    while !q.is_empty() {
        let (curr_grid, (x, y), prev_pos, steps) = q.pop_front().unwrap();
        if (x, y) == target {
            return (curr_grid, (x, y), prev_pos, steps);
        }
        visited.insert((x, y));
        for diff in [(1, 0), (0, -1), (0, 1), (-1, 0)].iter() {
            let (next_x, next_y) = (x + diff.0, y + diff.1);
            if (0..curr_grid[0].len() as i64).contains(&next_x)
                && (0..curr_grid.len() as i64).contains(&next_y)
                && !visited.contains(&(next_x, next_y))
                && curr_grid[next_y as usize][next_x as usize].0
                    <= curr_grid[y as usize][x as usize].1
            {
                let next_grid = swap_disk_content(&curr_grid, (x, y), (next_x, next_y));
                if cmp_diff((x, y), (next_x, next_y), target) < 0 {
                    q.push_front((next_grid, (next_x, next_y), (x, y), steps + 1));
                } else {
                    q.push_back((next_grid, (next_x, next_y), (x, y), steps + 1));
                }
            }
        }
    }
    unreachable!()
}

fn swap_disk_content(grid: &[Vec<(usize, usize)>], pos1: Position, pos2: Position) -> Grid {
    let (from_x, from_y) = pos1;
    let (to_x, to_y) = pos2;
    let mut next_grid = grid.to_vec();
    next_grid[to_y as usize][to_x as usize].0 = grid[from_y as usize][from_x as usize].0;
    next_grid[from_y as usize][from_x as usize].0 = grid[to_y as usize][to_x as usize].0;
    next_grid
}

fn cmp_diff(prev: Position, curr: Position, target: Position) -> i64 {
    let prev_dist = (prev.0 - target.0).abs() + (prev.1 - target.1).abs();
    let curr_dist = (curr.0 - target.0).abs() + (curr.1 - target.1).abs();
    curr_dist - prev_dist
}

fn is_in_neighbourhood(curr: Position, target: Position) -> bool {
    let x_diff = (curr.0 - target.0).abs();
    let y_diff = (curr.1 - target.1).abs();
    x_diff <= 1 && y_diff <= 1
}

#[allow(dead_code)]
fn print_grid(grid: &[Vec<(usize, usize)>]) {
    println!(
        "{}",
        grid.iter()
            .map(|row| {
                row.iter().fold(String::new(), |mut acc, n| {
                    acc.push_str(&format!("{:3}/{:3}  ", n.0, n.1));
                    acc
                })
            })
            .fold(String::new(), |mut acc, r| {
                acc.push_str(&r);
                acc.push('\n');
                acc
            })
    );
}

fn nodes_to_grid(nodes: &[Node]) -> Grid {
    nodes
        .iter()
        .fold(Vec::new(), |mut acc, (path, size, used, _, _)| {
            let coords = path
                .strip_prefix("/dev/grid/node-x")
                .unwrap()
                .split_terminator("-y")
                .filter_map(|v| v.parse().ok())
                .collect::<Vec<usize>>();
            if acc.len() <= coords[1] {
                acc.push(vec![(*used, *size)]);
            } else {
                acc[coords[1]].push((*used, *size));
            }
            acc
        })
}

fn find_viable_pairs_count(nodes: &[Node]) -> usize {
    let mut pairs = HashSet::new();
    let mut usages = nodes
        .iter()
        .enumerate()
        .map(|(i, n)| (i, n.2))
        .collect::<Vec<_>>();
    let mut available_spaces = nodes
        .iter()
        .enumerate()
        .map(|(i, n)| (i, n.3))
        .collect::<Vec<_>>();
    usages.sort_unstable_by_key(|s| s.1);
    available_spaces.sort_unstable_by_key(|s| s.1);

    for &(i, usage) in &usages {
        for &(j, available_space) in available_spaces.iter().rev() {
            if usage > available_space {
                break;
            }
            if usage > 0 && i != j && usage <= available_space {
                pairs.insert((i.min(j), j.max(i)));
            }
        }
    }

    pairs.len()
}

fn read(filename: &str) -> Vec<Node> {
    utils::read_to_string_in_module!(filename)
        .lines()
        .skip(2)
        .map(|s| {
            let parts = s
                .split_terminator(' ')
                .filter(|p| !p.is_empty())
                .collect::<Vec<_>>();
            let size = parts[1].strip_suffix('T').unwrap().parse().unwrap();
            let used = parts[2].strip_suffix('T').unwrap().parse().unwrap();
            let available = parts[3].strip_suffix('T').unwrap().parse().unwrap();
            let percentage = parts[4].strip_suffix('%').unwrap().parse().unwrap();
            (parts[0].into(), size, used, available, percentage)
        })
        .collect()
}
