use itertools::Itertools;
use utils::read_to_string_in_module;

pub fn run() {
    let input = read_to_string_in_module!("input.txt");

    println!("part1 solution: {}", solve_captcha(&input));
    println!("part2 solution: {}", solve_captcha2(&input));
}

fn solve_captcha(input: &str) -> usize {
    input
        .bytes()
        .circular_tuple_windows()
        .map(|(a, b)| if a == b { (a - b'0') as usize } else { 0 })
        .sum()
}

fn solve_captcha2(input: &str) -> usize {
    let data = input.as_bytes();
    let size = input.len();
    let mut res = 0;
    for i in 0..size {
        if data[i] == data[(i + size / 2) % size] {
            res += (data[i] - b'0') as usize;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_tests() {
        assert_eq!(solve_captcha("1122"), 3);
        assert_eq!(solve_captcha("1111"), 4);
        assert_eq!(solve_captcha("1234"), 0);
        assert_eq!(solve_captcha("91212129"), 9);
    }

    #[test]
    fn part2_tests() {
        assert_eq!(solve_captcha2("1212"), 6);
        assert_eq!(solve_captcha2("1221"), 0);
        assert_eq!(solve_captcha2("123425"), 4);
        assert_eq!(solve_captcha2("123123"), 12);
        assert_eq!(solve_captcha2("12131415"), 4);
    }
}
