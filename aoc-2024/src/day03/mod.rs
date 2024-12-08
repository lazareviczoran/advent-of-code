pub fn run() {
    let input = read_input("input.txt");
    utils::run_solution!(|| compute(&input, false), "part1");
    utils::run_solution!(|| compute(&input, true), "part2");
}

fn compute(input: &str, consider_do: bool) -> usize {
    let regex_str = if consider_do {
        r"do\(\)|don\'t\(\)|mul\((\d+),(\d+)\)"
    } else {
        r"mul\((\d+),(\d+)\)"
    };
    let re = regex::Regex::new(regex_str).unwrap();
    re.captures_iter(input)
        .fold((true, 0), |(mut should_include, mut values), next| {
            match (next.get(0), next.get(1), next.get(2)) {
                (Some(m), None, None) if m.as_str() == "do()" => should_include = true,
                (Some(m), None, None) if m.as_str() == "don't()" => should_include = false,
                (Some(m), Some(a), Some(b)) if m.as_str().starts_with("mul(") && should_include => {
                    values +=
                        a.as_str().parse::<usize>().unwrap() * b.as_str().parse::<usize>().unwrap()
                }
                _ => {}
            }
            (should_include, values)
        })
        .1
}

fn read_input(filename: &str) -> String {
    utils::read_to_string_in_module!(filename)
}

#[cfg(test)]
mod tests {
    use super::compute;
    use super::read_input;

    #[test]
    fn test() {
        assert_eq!(compute(&read_input("test-input.txt"), false), 161);
        assert_eq!(compute(&read_input("test-input2.txt"), true), 48);
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
        assert_eq!(compute(&read_input("input.txt"), false), pt1);
        assert_eq!(compute(&read_input("input.txt"), true), pt2);
    }
}
