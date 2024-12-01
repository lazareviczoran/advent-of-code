pub fn run() {
    utils::run_solution!(|| find_lowest_answer("iwrupvqb", "00000"), "part1");

    utils::run_solution!(|| find_lowest_answer("iwrupvqb", "000000"), "part2");
}

fn find_lowest_answer(secret: &str, starts_with_condition: &str) -> usize {
    let mut hash;
    let mut answer = 1;
    loop {
        let mut new_attempt = String::new();
        new_attempt.push_str(secret);
        new_attempt.push_str(answer.to_string().as_str());
        let digest = md5::compute(new_attempt.as_bytes());
        hash = format!("{:x}", digest);
        if hash.starts_with(starts_with_condition) {
            return answer;
        }
        answer += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_input1() {
        assert_eq!(find_lowest_answer("abcdef", "00000"), 609043);
    }

    #[test]
    fn part1_input2() {
        assert_eq!(find_lowest_answer("pqrstuv", "00000"), 1048970);
    }
}
