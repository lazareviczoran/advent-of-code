pub fn run() {
    let spreadsheet = read("input.txt");
    println!("part1 solution: {}", checksum(&spreadsheet));
    println!("part2 solution: {}", checksum2(&spreadsheet));
}

fn checksum(spreadsheet: &[Vec<usize>]) -> usize {
    spreadsheet
        .iter()
        .map(|row| {
            let (min_val, max_val) = row.iter().fold((usize::MAX, 0), |(min_val, max_val), c| {
                (min_val.min(*c), max_val.max(*c))
            });
            max_val - min_val
        })
        .sum()
}

fn checksum2(spreadsheet: &[Vec<usize>]) -> usize {
    spreadsheet
        .iter()
        .map(|row| {
            for (i, &a) in row.iter().enumerate().take(row.len() - 1) {
                for &b in row.iter().skip(i + 1) {
                    if a % b == 0 {
                        return a / b;
                    } else if b % a == 0 {
                        return b / a;
                    }
                }
            }
            0
        })
        .sum()
}

fn read(filename: &str) -> Vec<Vec<usize>> {
    utils::read_to_string_in_module!(filename)
        .lines()
        .map(|s| {
            s.split_terminator(' ')
                .filter(|p| !p.is_empty())
                .filter_map(|p| p.parse().ok())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let spreadsheet = read("test-input.txt");
        assert_eq!(checksum(&spreadsheet), 18);
    }

    #[test]
    fn part2_test() {
        let spreadsheet = read("test-input2.txt");
        assert_eq!(checksum2(&spreadsheet), 9);
    }
}
