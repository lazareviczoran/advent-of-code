pub fn run() {
    let map = read("input.txt");
    let (message, steps) = find_message(&map);
    println!("part1 solution: {}", message);
    println!("part2 solution: {}", steps);
}

fn find_message(map: &[Vec<char>]) -> (String, usize) {
    let start_pos = (map[0].iter().position(|ch| ch == &'|').unwrap(), 0);
    let mut q = vec![(start_pos, (0, 1), 1, String::new())];
    while let Some(((x, y), dir, steps, mut message)) = q.pop() {
        if map[y][x].is_alphabetic() {
            message.push(map[y][x]);
        }
        let (next_x, next_y) = (x as i64 + dir.0, y as i64 + dir.1);
        if (0..map[y].len() as i64).contains(&next_x)
            && (0..map.len() as i64).contains(&next_y)
            && map[next_y as usize][next_x as usize] != ' '
        {
            q.push(((next_x as usize, next_y as usize), dir, steps + 1, message));
        } else {
            match dir {
                (0, _) => {
                    [(1, 0), (-1, 0)].iter().for_each(|&(x_diff, y_diff)| {
                        let (next_x, next_y) = (x as i64 + x_diff, y as i64 + y_diff);
                        if (0..map[y].len() as i64).contains(&next_x)
                            && (0..map.len() as i64).contains(&next_y)
                            && map[next_y as usize][next_x as usize] != ' '
                        {
                            q.push((
                                (next_x as usize, next_y as usize),
                                (x_diff, y_diff),
                                steps + 1,
                                message.clone(),
                            ));
                        }
                    });
                }
                (_, 0) => {
                    [(0, 1), (0, -1)].iter().for_each(|&(x_diff, y_diff)| {
                        let (next_x, next_y) = (x as i64 + x_diff, y as i64 + y_diff);
                        if (0..map[y].len() as i64).contains(&next_x)
                            && (0..map.len() as i64).contains(&next_y)
                            && map[next_y as usize][next_x as usize] != ' '
                        {
                            q.push((
                                (next_x as usize, next_y as usize),
                                (x_diff, y_diff),
                                steps + 1,
                                message.clone(),
                            ));
                        }
                    });
                }
                _ => unreachable!(),
            }
            if q.is_empty() {
                return (message, steps);
            }
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
    fn tests() {
        let (message, steps) = find_message(&read("test-input.txt"));
        assert_eq!(message, "ABCDEF");
        assert_eq!(steps, 38);
    }
}
