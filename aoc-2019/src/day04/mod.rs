use std::collections::HashMap;

pub fn run() {
    let (passwords1, passwords2) = get_potential_password_count();
    println!("Secure Container part1 Solution: {}", passwords1);
    println!("Secure Container part2 Solution: {}", passwords2);
}

fn get_potential_password_count() -> (i32, i32) {
    let mut count = 0;
    let mut count2 = 0;
    for pass in 137683..=596253 {
        let (is_valid, is_valid_with_extra_rule) = is_valid_password(pass);
        if is_valid {
            count += 1;
        }
        if is_valid_with_extra_rule {
            count2 += 1;
        }
    }
    (count, count2)
}

fn is_valid_password(password: i32) -> (bool, bool) {
    if !(137683..596253).contains(&password) {
        return (false, false);
    }

    let mut prev_char: char = ' ';
    let str_pass = password.to_string();
    let mut has_repeated_digits = false;
    let mut repeadet_digits: HashMap<char, i32> = HashMap::new();
    for digit in str_pass.chars() {
        if prev_char != ' ' {
            match prev_char.cmp(&digit) {
                std::cmp::Ordering::Less => {}
                std::cmp::Ordering::Equal => {
                    has_repeated_digits = true;
                    *repeadet_digits.entry(digit).or_default() += 1;
                }
                std::cmp::Ordering::Greater => return (false, false),
            }
        }
        prev_char = digit;
    }

    return (
        has_repeated_digits,
        repeadet_digits.values().any(|&v| v == 1),
    );
}

#[cfg(test)]
mod test {
    use super::is_valid_password;

    #[test]
    fn first_sample_input() {
        assert_eq!(is_valid_password(222222), (true, false));
    }

    #[test]
    fn second_sample_input() {
        assert_eq!(is_valid_password(223450), (false, false));
    }

    #[test]
    fn third_sample_input() {
        assert_eq!(is_valid_password(234789), (false, false));
    }

    #[test]
    fn fourth_sample_input() {
        assert_eq!(is_valid_password(223344), (true, true));
    }

    #[test]
    fn fifth_sample_input() {
        assert_eq!(is_valid_password(234555), (true, false));
    }

    #[test]
    fn sixth_sample_input() {
        assert_eq!(is_valid_password(222233), (true, true));
    }
}
