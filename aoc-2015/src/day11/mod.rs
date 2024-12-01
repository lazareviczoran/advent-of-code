use std::collections::HashSet;

pub fn run() {
    utils::run_solution!(|| get_next_valid_password("hxbxwxba"), "part1");
    utils::run_solution!(|| get_next_valid_password("hxbxxyzz"), "part2");
}

fn get_next_valid_password(current: &str) -> String {
    let mut pass = generate_next_password(current);
    while !is_valid_password(&pass) {
        pass = generate_next_password(&pass);
    }
    pass
}

fn generate_next_password(current: &str) -> String {
    let mut res = String::new();
    for ch in current.chars().rev() {
        if ch != 'z' {
            res.insert(0, (ch as u8 + 1) as char);
            res.insert_str(0, current.get(0..current.len() - res.len()).unwrap());
            return res;
        } else {
            res.insert(0, 'a');
        }
    }
    res
}

fn is_valid_password(password: &str) -> bool {
    let mut has_increasing_streak_3 = false;
    let mut increasing_streak_lenght = 1;
    let mut pairs = HashSet::new();
    let mut chars = password.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == 'i' || ch == 'o' || ch == 'l' {
            return false;
        }
        if let Some(next_ch) = chars.peek() {
            if ch == *next_ch && !pairs.contains(&ch) {
                pairs.insert(ch);
            }
            if !has_increasing_streak_3 && *next_ch == (ch as u8 + 1) as char {
                increasing_streak_lenght += 1;
                if increasing_streak_lenght > 2 {
                    has_increasing_streak_3 = true;
                }
            } else {
                increasing_streak_lenght = 1;
            }
        }
    }

    has_increasing_streak_3 && pairs.len() > 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_generate_next_pass() {
        assert_eq!(generate_next_password("fsdfsaa"), "fsdfsab");
        assert_eq!(generate_next_password("fsdfsaz"), "fsdfsba");
        assert_eq!(generate_next_password("fsdfzzz"), "fsdgaaa");
    }

    #[test]
    fn part1_is_valid() {
        assert!(!is_valid_password("hijklmmn"));
        assert!(!is_valid_password("abbceffg"));
        assert!(!is_valid_password("abbcegjk"));
        assert!(is_valid_password("abcdffaa"));
        assert!(is_valid_password("ghjaabcc"));
    }

    #[test]
    fn part1_get_next_valid_password() {
        assert_eq!(get_next_valid_password("abcdefgh"), "abcdffaa");
        assert_eq!(get_next_valid_password("ghijklmn"), "ghjaabcc");
    }
}
