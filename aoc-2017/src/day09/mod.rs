use std::iter::Peekable;
use std::str::Chars;

use utils::read_to_string_in_module;

pub fn run() {
    let input = read_to_string_in_module!("input.txt");
    let (score, garbage_count) = calculate_score(&input);
    println!("part1 solution: {}", score);
    println!("part2 solution: {}", garbage_count);
}

fn calculate_score(stream: &str) -> (usize, usize) {
    let mut chars = stream.chars().peekable();
    calculate_score_rec(&mut chars, 0)
}

fn calculate_score_rec(iter: &mut Peekable<Chars>, level: usize) -> (usize, usize) {
    let mut score = level;
    let mut garbage = 0;
    while let Some(ch) = iter.next() {
        match ch {
            '{' => {
                let (partial_score, partial_garbage) = calculate_score_rec(iter, level + 1);
                score += partial_score;
                garbage += partial_garbage;
            }
            '}' => return (score, garbage),
            ',' => {}
            '<' => {
                while let Some(skipped) = iter.next() {
                    match skipped {
                        '!' => {
                            iter.next();
                        }
                        '>' => break,
                        _ => {
                            garbage += 1;
                        }
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    (score, garbage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        // {}, score of 1.
        assert_eq!(calculate_score("{}").0, 1);
        // {{{}}}, score of 1 + 2 + 3 = 6.
        assert_eq!(calculate_score("{{{}}}").0, 6);
        // {{},{}}, score of 1 + 2 + 2 = 5.
        assert_eq!(calculate_score("{{},{}}").0, 5);
        // {{{},{},{{}}}}, score of 1 + 2 + 3 + 3 + 3 + 4 = 16.
        assert_eq!(calculate_score("{{{},{},{{}}}}").0, 16);
        // {<a>,<a>,<a>,<a>}, score of 1.
        assert_eq!(calculate_score("{<a>,<a>,<a>,<a>}").0, 1);
        // {{<ab>},{<ab>},{<ab>},{<ab>}}, score of 1 + 2 + 2 + 2 + 2 = 9.
        assert_eq!(calculate_score("{{<ab>},{<ab>},{<ab>},{<ab>}}").0, 9);
        // {{<!!>},{<!!>},{<!!>},{<!!>}}, score of 1 + 2 + 2 + 2 + 2 = 9.
        assert_eq!(calculate_score("{{<!!>},{<!!>},{<!!>},{<!!>}}").0, 9);
        // {{<a!>},{<a!>},{<a!>},{<ab>}}, score of 1 + 2 = 3.
        assert_eq!(calculate_score("{{<a!>},{<a!>},{<a!>},{<ab>}}").0, 3);
    }
}
