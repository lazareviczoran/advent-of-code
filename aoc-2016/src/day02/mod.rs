pub fn run() {
    let buttons = vec![
        vec!['1', '2', '3'],
        vec!['4', '5', '6'],
        vec!['7', '8', '9'],
    ];
    let movements = read("input.txt");
    println!("part1 solution: {}", find_code(&buttons, &movements));

    let buttons = vec![
        vec!['1'],
        vec!['2', '3', '4'],
        vec!['5', '6', '7', '8', '9'],
        vec!['A', 'B', 'C'],
        vec!['D'],
    ];
    println!("part2 solution: {}", find_code2(&buttons, &movements));
}

fn find_code(buttons: &[Vec<char>], movements: &[Vec<Dir>]) -> String {
    let mut curr_pos = find_start_pos(buttons);
    let mut result = String::new();
    for movement in movements {
        execute(&mut curr_pos, buttons, movement);
        let (x, y) = curr_pos;
        result.push(buttons[y][x]);
    }

    result
}

fn find_code2(buttons: &[Vec<char>], movements: &[Vec<Dir>]) -> String {
    let mut curr_pos = find_start_pos(buttons);
    let mut result = String::new();
    for movement in movements {
        execute2(&mut curr_pos, buttons, movement);
        let (x, y) = curr_pos;
        result.push(buttons[y][x]);
    }

    result
}

fn find_start_pos(buttons: &[Vec<char>]) -> (usize, usize) {
    buttons
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, &ch)| if ch == '5' { Some((x, y)) } else { None })
        })
        .unwrap()
}

fn execute(curr_pos: &mut (usize, usize), buttons: &[Vec<char>], moves: &[Dir]) {
    for m in moves {
        match m {
            Dir::Up => {
                if curr_pos.1 > 0 {
                    curr_pos.1 -= 1;
                }
            }
            Dir::Down => {
                if curr_pos.1 < buttons.len() - 1 {
                    curr_pos.1 += 1;
                }
            }
            Dir::Left => {
                if curr_pos.0 > 0 {
                    curr_pos.0 -= 1;
                }
            }
            Dir::Right => {
                if curr_pos.0 < buttons[curr_pos.1].len() - 1 {
                    curr_pos.0 += 1;
                }
            }
        }
    }
}

fn execute2(curr_pos: &mut (usize, usize), buttons: &[Vec<char>], moves: &[Dir]) {
    for m in moves {
        let curr_width = buttons[curr_pos.1].len();
        match m {
            Dir::Up => {
                if curr_pos.1 > 0
                    && (curr_pos.1 > buttons.len() / 2
                        || curr_pos.0 != 0 && curr_pos.0 != curr_width - 1)
                {
                    curr_pos.1 -= 1;
                    if buttons[curr_pos.1].len() > curr_width {
                        curr_pos.0 += 1;
                    } else {
                        curr_pos.0 -= 1;
                    }
                }
            }
            Dir::Down => {
                if curr_pos.1 < buttons.len() - 1
                    && (curr_pos.1 < buttons.len() / 2
                        || curr_pos.0 != 0 && curr_pos.0 != curr_width - 1)
                {
                    curr_pos.1 += 1;
                    if buttons[curr_pos.1].len() > curr_width {
                        curr_pos.0 += 1;
                    } else {
                        curr_pos.0 -= 1;
                    }
                }
            }
            Dir::Left => {
                if curr_pos.0 > 0 {
                    curr_pos.0 -= 1;
                }
            }
            Dir::Right => {
                if curr_pos.0 < buttons[curr_pos.1].len() - 1 {
                    curr_pos.0 += 1;
                }
            }
        }
    }
}

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn read(filename: &str) -> Vec<Vec<Dir>> {
    utils::read_to_string_in_module!(filename)
        .split_terminator('\n')
        .map(|s| {
            s.chars()
                .map(|ch| match ch {
                    'U' => Dir::Up,
                    'D' => Dir::Down,
                    'L' => Dir::Left,
                    'R' => Dir::Right,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let buttons = vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ];
        let moves = read("test-input.txt");
        assert_eq!(find_code(&buttons, &moves), "1985");
    }
    #[test]
    fn part2_test() {
        let buttons = vec![
            vec!['1'],
            vec!['2', '3', '4'],
            vec!['5', '6', '7', '8', '9'],
            vec!['A', 'B', 'C'],
            vec!['D'],
        ];
        let moves = read("test-input.txt");
        assert_eq!(find_code2(&buttons, &moves), "5DB3");
    }
}
