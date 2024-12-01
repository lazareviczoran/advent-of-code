pub fn run() {
    let input = read("input.txt");
    println!("part1 solution: {:?}", count_steps(input.clone(), false));
    println!("part2 solution: {:?}", count_steps(input, true));
}

fn count_steps(mut instructions: Vec<i64>, part2: bool) -> usize {
    let mut steps = 0;
    let mut curr_idx = 0;
    while curr_idx < instructions.len() as i64 {
        let temp = instructions[curr_idx as usize];
        if part2 && temp >= 3 {
            instructions[curr_idx as usize] -= 1;
        } else {
            instructions[curr_idx as usize] += 1;
        }
        curr_idx += temp;
        steps += 1;
    }

    steps
}

fn read(filename: &str) -> Vec<i64> {
    utils::read_to_string_in_module!(filename)
        .lines()
        .filter_map(|s| s.parse().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(count_steps(vec![0, 3, 0, 1, -3], false), 5);
    }

    #[test]
    fn part2_test() {
        assert_eq!(count_steps(vec![0, 3, 0, 1, -3], true), 10);
    }
}
