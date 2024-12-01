use regex::Regex;
use utils::read_to_string_in_module;

pub fn run() {
    let passwords = read("input.txt");
    println!("part1 solution: {}", count_valid(&passwords));
    println!("part2 solution: {}", count_valid2(&passwords));
}

fn count_valid(passwords: &[(usize, usize, char, Vec<char>)]) -> usize {
    passwords
        .iter()
        .filter(|(min, max, ch, pwd)| {
            let count = pwd.iter().filter(|&c| ch == c).count();
            count >= *min && count <= *max
        })
        .count()
}

fn count_valid2(passwords: &[(usize, usize, char, Vec<char>)]) -> usize {
    passwords
        .iter()
        .filter(|(min, max, ch, pwd)| {
            let n = pwd.len();
            let pos1 = *min <= n && pwd[*min - 1] == *ch;
            let pos2 = *max <= n && pwd[*max - 1] == *ch;
            pos1 ^ pos2
        })
        .count()
}

fn read(filename: &str) -> Vec<(usize, usize, char, Vec<char>)> {
    let re = Regex::new(r"(\d+)-(\d+)\s([a-z]):\s(.*)").unwrap();
    read_to_string_in_module!(filename)
        .split_terminator('\n')
        .map(|s| {
            let caps = re.captures(s).unwrap();
            (
                caps[1].parse().unwrap(),
                caps[2].parse().unwrap(),
                caps[3].chars().next().unwrap(),
                caps[4].chars().collect(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let passwords = read("test-input.txt");
        assert_eq!(count_valid(&passwords), 2);
    }

    #[test]
    fn part2_test() {
        let passwords = read("test-input.txt");
        assert_eq!(count_valid2(&passwords), 1);
    }
}
