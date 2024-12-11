use rustc_hash::FxHashMap;

pub fn run() {
    let stones = read_input("input.txt");

    utils::run_solution!(|| count_stones(&stones, 25), "part1");
    utils::run_solution!(|| count_stones(&stones, 75), "part2");
}

fn count_stones(stones: &[usize], n: usize) -> usize {
    let mut memo = FxHashMap::default();
    stones.iter().map(|&s| find(s, n, &mut memo)).sum()
}

fn find(item: usize, n: usize, memo: &mut FxHashMap<(usize, usize), usize>) -> usize {
    if let Some(&res) = memo.get(&(item, n)) {
        return res;
    }
    if n == 0 {
        return 1;
    }
    let res = match item {
        0 => find(1, n - 1, memo),
        s if (s.ilog10() + 1) % 2 == 0 => {
            let k = 10_i32.pow((s.ilog10() + 1) / 2) as usize;
            find(s / k, n - 1, memo) + find(s % k, n - 1, memo)
        }
        _ => find(item * 2024, n - 1, memo),
    };
    memo.insert((item, n), res);
    res
}

fn read_input(filename: &str) -> Vec<usize> {
    utils::read_to_string_in_module!(filename)
        .split_whitespace()
        .map(|val| val.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::count_stones;
    use super::read_input;

    #[test]
    fn p1() {
        let stones = read_input("test-input.txt");
        assert_eq!(count_stones(&stones, 1), 7);
        let stones = read_input("test-input2.txt");
        assert_eq!(count_stones(&stones, 6), 22);
    }

    #[test]
    #[cfg(feature = "include-main-input")]
    fn prod() {
        use itertools::Itertools;

        let (pt1, pt2) = utils::read_to_string_in_module!("results.txt")
            .lines()
            .map(|line| line.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        let stones = read_input("input.txt");
        assert_eq!(count_stones(&stones, 25), pt1);
        assert_eq!(count_stones(&stones, 75), pt2);
    }
}
