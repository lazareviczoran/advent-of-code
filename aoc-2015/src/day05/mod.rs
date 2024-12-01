pub fn run() {
    let contents = utils::read_to_string_in_module!("input.txt");

    let strings: Vec<&str> = contents.split_terminator('\n').collect();

    utils::run_solution!(|| strings.iter().filter(|e| is_nice(e)).count(), "part1");

    utils::run_solution!(|| strings.iter().filter(|e| is_nice_v2(e)).count(), "part2");
}

fn is_nice(string: &str) -> bool {
    let mut vowel_count = 0;
    let mut has_repeated_adjs = false;
    let mut has_disallowed_adjs = false;
    let mut chars = string.chars().peekable();
    while let Some(ch) = chars.next() {
        if "aeiou".contains(ch) {
            vowel_count += 1;
        }
        if let Some(next) = chars.peek() {
            if !has_repeated_adjs && ch == *next {
                has_repeated_adjs = true;
            }
            let mut adjs_string = String::new();
            adjs_string.push(ch);
            adjs_string.push(*next);
            if "ab,cd,pq,xy".contains(&adjs_string.as_str()) {
                has_disallowed_adjs = true;
            }
        }
    }

    vowel_count >= 3 && has_repeated_adjs && !has_disallowed_adjs
}

fn is_nice_v2(string: &str) -> bool {
    let mut has_repeated_pair = false;
    let mut has_mirrored_seq = false;
    let mut chars = string.chars();
    let mut prev = None;
    while let Some(ch) = chars.next() {
        let mut peekable = chars.clone().peekable();
        if let Some(next_ch) = peekable.peek() {
            if !has_repeated_pair {
                let (_, remaining) = chars.as_str().split_at(1);
                let mut pair = String::new();
                pair.push(ch);
                pair.push(*next_ch);
                if remaining.contains(&pair) {
                    has_repeated_pair = true;
                }
            }
            if !has_mirrored_seq && prev.is_some() {
                if prev.unwrap() == *next_ch {
                    has_mirrored_seq = true;
                }
            }
        }
        prev = Some(ch);
    }

    has_mirrored_seq && has_repeated_pair
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_input1() {
        assert!(is_nice("ugknbfddgicrmopn"));
    }

    #[test]
    fn part1_input2() {
        assert!(is_nice("aaa"));
    }

    #[test]
    fn part1_input3() {
        assert!(!is_nice("jchzalrnumimnmhp"));
    }

    #[test]
    fn part1_input4() {
        assert!(!is_nice("haegwjzuvuyypxyu"));
    }

    #[test]
    fn part1_input5() {
        assert!(!is_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn part2_input1() {
        assert!(is_nice_v2("qjhvhtzxzqqjkmpb"));
    }

    #[test]
    fn part2_input2() {
        assert!(is_nice_v2("xxyxx"));
    }

    #[test]
    fn part2_input3() {
        assert!(!is_nice_v2("uurcxstgmygtbstg"));
    }

    #[test]
    fn part2_input4() {
        assert!(!is_nice_v2("ieodomkazucvgmuy"));
    }
}
