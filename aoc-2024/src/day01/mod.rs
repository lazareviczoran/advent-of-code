pub fn run() {
    let lists = read_input("input.txt");
    utils::measure_exec_time(|| calc_total_distance(&lists), "part1");
    utils::measure_exec_time(|| calc_similarity_score(&lists), "part2");
}

fn calc_total_distance(lists: &[Vec<isize>; 2]) -> isize {
    lists[0]
        .iter()
        .zip(&lists[1])
        .map(|(x, y)| (x - y).abs())
        .sum()
}

fn calc_similarity_score(lists: &[Vec<isize>; 2]) -> isize {
    let (l1, l2) = (&lists[0], &lists[1]);
    let l2_set = l2
        .iter()
        .cloned()
        .fold(std::collections::HashMap::new(), |mut acc, val| {
            *acc.entry(val).or_insert(0) += 1;
            acc
        });
    l1.iter()
        .map(|&val| val * l2_set.get(&val).unwrap_or(&0))
        .sum()
}

fn read_input(filename: &str) -> [Vec<isize>; 2] {
    let mut lists = std::fs::read_to_string(utils::get_file_path_within_module!(filename))
        .expect("failed to read file")
        .lines()
        .map(|l| {
            let parts = l
                .split_whitespace()
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<isize>().unwrap())
                .collect::<Vec<_>>();
            (parts[0], parts[1])
        })
        .fold([Vec::new(), Vec::new()], |mut acc, (x, y)| {
            acc[0].push(x);
            acc[1].push(y);
            acc
        });
    for list in &mut lists {
        list.sort();
    }
    lists
}

#[cfg(test)]
mod tests {
    use crate::day01::calc_similarity_score;
    use crate::day01::calc_total_distance;
    use crate::day01::read_input;

    #[test]
    fn test() {
        assert_eq!(calc_total_distance(&read_input("test-input.txt")), 11);
        assert_eq!(calc_similarity_score(&read_input("test-input.txt")), 31);
    }

    #[test]
    fn prod() {
        assert_eq!(calc_total_distance(&read_input("input.txt")), 936063);
        assert_eq!(calc_similarity_score(&read_input("input.txt")), 23150395);
    }
}
