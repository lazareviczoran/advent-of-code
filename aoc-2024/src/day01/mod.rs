pub fn run() {
    let lists = read_input("input.txt");
    utils::run_solution!(|| calc_total_distance(&lists), "part1");
    utils::run_solution!(|| calc_similarity_score(&lists), "part2");
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
    let l2_set: rustc_hash::FxHashMap<_, _> =
        l2.iter().cloned().fold(Default::default(), |mut acc, val| {
            *acc.entry(val).or_insert(0) += 1;
            acc
        });
    l1.iter()
        .map(|&val| val * l2_set.get(&val).unwrap_or(&0))
        .sum()
}

fn read_input(filename: &str) -> [Vec<isize>; 2] {
    let mut lists = utils::read_to_string_in_module!(filename)
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
    #[cfg(feature = "include-main-input")]
    fn prod() {
        use itertools::Itertools;
        let (pt1, pt2) = utils::read_to_string_in_module!("results.txt")
            .lines()
            .map(|line| line.parse::<isize>().unwrap())
            .collect_tuple()
            .unwrap();
        assert_eq!(calc_total_distance(&read_input("input.txt")), pt1);
        assert_eq!(calc_similarity_score(&read_input("input.txt")), pt2);
    }
}
