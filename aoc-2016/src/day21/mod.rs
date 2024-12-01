use std::collections::VecDeque;

pub fn run() {
    let instructions = read("input.txt");
    println!(
        "part1 solution: {}",
        apply_instructions("abcdefgh", &instructions)
    );
    println!(
        "part2 solution: {}",
        revert_instructions("fbgdceah", &instructions)
    );
}

fn apply_instructions(content: &str, instructions: &[Instruction]) -> String {
    let mut content = content.chars().collect::<VecDeque<_>>();
    for instr in instructions {
        match instr {
            Instruction::SwapPos { p1, p2 } => content.swap(*p1, *p2),
            Instruction::SwapLetter { l1, l2 } => content.iter_mut().for_each(|ch| {
                if ch == l1 {
                    *ch = *l2;
                } else if ch == l2 {
                    *ch = *l1;
                }
            }),
            Instruction::Rotate { dir, steps } => match dir {
                Dir::Left => content.rotate_left(*steps),
                Dir::Rigth => content.rotate_right(*steps),
            },
            Instruction::RotateLetter { letter } => {
                let idx = content.iter().position(|ch| ch == letter).unwrap();
                content.rotate_right(idx + 1);
                if idx >= 4 {
                    content.rotate_right(1);
                }
            }
            Instruction::Reverse { from, through } => {
                let mid = (through - from + 1) / 2;
                for i in 0..mid {
                    content.swap(from + i, through - i);
                }
            }
            Instruction::Move { from, to } => {
                let value = content.remove(*from).unwrap();
                content.insert(*to, value);
            }
        }
    }
    content.into_iter().collect()
}

fn revert_instructions(content: &str, instructions: &[Instruction]) -> String {
    let mut content = content.chars().collect::<VecDeque<_>>();
    for instr in instructions.iter().rev() {
        println!("content {:?}", content);
        println!("instr {:?}", instr);
        match instr {
            Instruction::SwapPos { p1, p2 } => content.swap(*p1, *p2),
            Instruction::SwapLetter { l1, l2 } => content.iter_mut().for_each(|ch| {
                if ch == l1 {
                    *ch = *l2;
                } else if ch == l2 {
                    *ch = *l1;
                }
            }),
            Instruction::Rotate { dir, steps } => match dir {
                Dir::Left => content.rotate_right(*steps),
                Dir::Rigth => content.rotate_left(*steps),
            },
            Instruction::RotateLetter { letter } => {
                let idx = content.iter().position(|ch| ch == letter).unwrap();

                if idx % 2 == 0 {
                    match idx {
                        0 => content.rotate_left(1),
                        2 => content.rotate_right(2),
                        4 => content.rotate_right(1),
                        _ => {}
                    }
                } else {
                    match idx {
                        1 => content.rotate_left(1),
                        3 => content.rotate_left(2),
                        5 => content.rotate_left(3),
                        _ => content.rotate_left(4),
                    }
                }
            }
            Instruction::Reverse { from, through } => {
                let mid = (through - from + 1) / 2;
                for i in 0..mid {
                    content.swap(from + i, through - i);
                }
            }
            Instruction::Move { from, to } => {
                let value = content.remove(*to).unwrap();
                content.insert(*from, value);
            }
        }
    }
    content.into_iter().collect()
}

#[derive(Debug)]
enum Instruction {
    SwapPos { p1: usize, p2: usize },
    SwapLetter { l1: char, l2: char },
    Rotate { dir: Dir, steps: usize },
    RotateLetter { letter: char },
    Reverse { from: usize, through: usize },
    Move { from: usize, to: usize },
}

#[derive(Debug)]
enum Dir {
    Left,
    Rigth,
}

fn read(filename: &str) -> Vec<Instruction> {
    utils::read_to_string_in_module!(filename)
        .split_terminator('\n')
        .map(|s| {
            if s.starts_with("swap position ") {
                let parts = s
                    .strip_prefix("swap position ")
                    .unwrap()
                    .split_terminator(" with position ")
                    .filter_map(|c| c.parse().ok())
                    .collect::<Vec<usize>>();
                Instruction::SwapPos {
                    p1: parts[0],
                    p2: parts[1],
                }
            } else if s.starts_with("swap letter ") {
                let parts = s
                    .strip_prefix("swap letter ")
                    .unwrap()
                    .split_terminator(" with letter ")
                    .filter_map(|c| c.chars().next())
                    .collect::<Vec<char>>();
                Instruction::SwapLetter {
                    l1: parts[0],
                    l2: parts[1],
                }
            } else if s.starts_with("rotate based ") {
                let letter = s
                    .strip_prefix("rotate based on position of letter ")
                    .unwrap()
                    .chars()
                    .next()
                    .unwrap();
                Instruction::RotateLetter { letter }
                //
            } else if s.starts_with("rotate ") {
                let parts = s
                    .strip_prefix("rotate ")
                    .unwrap()
                    .split_terminator(' ')
                    .collect::<Vec<_>>();
                let dir = match parts[0] {
                    "left" => Dir::Left,
                    "right" => Dir::Rigth,
                    _ => unreachable!(),
                };
                Instruction::Rotate {
                    dir,
                    steps: parts[1].parse().unwrap(),
                }
            } else if s.starts_with("reverse positions ") {
                let parts = s
                    .strip_prefix("reverse positions ")
                    .unwrap()
                    .split_terminator(" through ")
                    .filter_map(|c| c.parse().ok())
                    .collect::<Vec<usize>>();
                Instruction::Reverse {
                    from: parts[0],
                    through: parts[1],
                }
            } else if s.starts_with("move ") {
                let parts = s
                    .strip_prefix("move position ")
                    .unwrap()
                    .split_terminator(" to position ")
                    .filter_map(|c| c.parse().ok())
                    .collect::<Vec<usize>>();
                Instruction::Move {
                    from: parts[0],
                    to: parts[1],
                }
            } else {
                unreachable!()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let instructions = read("test-input.txt");
        assert_eq!(apply_instructions("abcde", &instructions), "decab");
    }

    #[test]
    fn part2_test() {
        let instructions = read("test-input.txt");
        assert_eq!(revert_instructions("decab", &instructions), "abcde");
    }

    #[test]
    fn part2_test2() {
        let instructions = read("input.txt");
        assert_eq!(revert_instructions("gbhcefad", &instructions), "abcdefgh");
    }
}
