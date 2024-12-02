pub fn run() {
    let lists = read_input("input.txt");
    utils::run_solution!(|| count_safe_reports(&lists, false), "part1");
    utils::run_solution!(|| count_safe_reports(&lists, true), "part2");
}

fn count_safe_reports(reports: &[Vec<isize>], allows_err: bool) -> usize {
    let check_fn = |diff: isize, sign, matching| {
        let is_ok = diff.signum() == sign && (1..=3).contains(&diff.abs());
        match matching {
            true => is_ok,
            false => !is_ok,
        }
    };
    let valid = |window: &[isize], sign| check_fn(window[0] - window[1], sign, true);
    let invalid = |window: &[isize], sign| check_fn(window[0] - window[1], sign, false);

    reports
        .iter()
        .filter(|&report| {
            let mut report = report.clone();
            let signs = report
                .windows(2)
                .filter(|window| (window[0] - window[1]).signum() != 0)
                .map(|window| (window[0] - window[1]).signum())
                .fold(std::collections::HashMap::new(), |mut map, k| {
                    *map.entry(k).or_insert(0) += 1;
                    map
                });
            let sign = *signs.iter().max_by_key(|&(_, v)| v).unwrap().0;

            let unsafe_position = report.windows(2).position(|w| invalid(w, sign));
            unsafe_position
                .map(|i| {
                    let removed = report.remove(i);
                    let is_safe = report.windows(2).all(|w| valid(w, sign));
                    report.remove(i);
                    report.insert(i, removed);
                    allows_err && (is_safe || report.windows(2).all(|w| valid(w, sign)))
                })
                .unwrap_or(true)
        })
        .count()
}

fn read_input(filename: &str) -> Vec<Vec<isize>> {
    utils::read_to_string_in_module!(filename)
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<isize>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::count_safe_reports;
    use super::read_input;

    #[test]
    fn test() {
        assert_eq!(count_safe_reports(&read_input("test-input.txt"), false), 2);
        assert_eq!(count_safe_reports(&read_input("test-input.txt"), true), 4);
    }

    #[test]
    fn prod() {
        assert_eq!(count_safe_reports(&read_input("input.txt"), false), 510);
        assert_eq!(count_safe_reports(&read_input("input.txt"), true), 553);
    }
}
