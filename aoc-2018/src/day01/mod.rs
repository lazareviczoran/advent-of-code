use regex::Regex;
use std::collections::HashSet;
use utils::read_to_string_in_module;

pub fn run() {
    let values = read_input("input.txt");
    println!(
        "Day 1: Chronal Calibration part1 solution\n {}",
        values.iter().sum::<i32>()
    );

    println!(
        "Day 1: Chronal Calibration part2 solution\n {}",
        find_first_repeated_frequency(&values)
    );
}

fn find_first_repeated_frequency(values: &Vec<i32>) -> i32 {
    let mut curr_freq = 0;
    let mut frequencies = HashSet::new();
    frequencies.insert(curr_freq);
    loop {
        for val in values {
            curr_freq += val;
            if frequencies.contains(&curr_freq) {
                return curr_freq;
            } else {
                frequencies.insert(curr_freq);
            }
        }
    }
}

fn read_input(filename: &str) -> Vec<i32> {
    let re = Regex::new(r"(-?\d+)").unwrap();
    read_to_string_in_module!(filename)
        .split_terminator('\n')
        .map(|s| {
            let caps = re.captures(s).unwrap();
            caps[1].parse::<i32>().unwrap()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part2_tests() {
        assert_eq!(find_first_repeated_frequency(&vec![1, -1]), 0);
        assert_eq!(find_first_repeated_frequency(&vec![3, 3, 4, -2, -4]), 10);
        assert_eq!(find_first_repeated_frequency(&vec![-6, 3, 8, 5, -6]), 5);
        assert_eq!(find_first_repeated_frequency(&vec![7, 7, -2, -7, -4]), 14);
    }
}
