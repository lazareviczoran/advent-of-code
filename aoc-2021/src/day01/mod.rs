pub fn run() {
    let input = read("input.txt");
    println!("part 1 solution: {}", count_increases(&input));
    println!("part 2 solution: {}", count_increases_3sum(&input));
}

fn count_increases_3sum(values: &[usize]) -> usize {
    count_increases(
        &values
            .windows(3)
            .map(|vals| vals.iter().sum::<usize>())
            .collect::<Vec<_>>(),
    )
}

fn count_increases(values: &[usize]) -> usize {
    values.windows(2).filter(|vals| vals[0] < vals[1]).count()
}

fn read(filename: &str) -> Vec<usize> {
    utils::read_to_string_in_module!(filename)
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::read;
    use crate::day01::{count_increases, count_increases_3sum};

    #[test]
    fn part1() {
        let input = read("test-input.txt");
        assert_eq!(count_increases(&input), 7)
    }

    #[test]
    fn part2() {
        let input = read("test-input.txt");
        assert_eq!(count_increases_3sum(&input), 5)
    }
}
