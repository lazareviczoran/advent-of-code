pub fn run() {
    let contents = utils::read_to_string_in_module!("input.txt");

    let strings = contents
        .split_terminator('\n')
        .map(|s| s.to_string())
        .collect();
    utils::run_solution!(|| calculate_total_decoded_diff(&strings), "part1");
    utils::run_solution!(|| calculate_total_encoded_diff(&strings), "part2");
}

fn calculate_total_decoded_diff(strings: &Vec<String>) -> usize {
    let mut total_unescaped_length = 0;
    let mut total_escaped_length = 0;
    for s in strings {
        total_unescaped_length += s.len();
        total_escaped_length += get_decoded_string_length(s);
    }

    total_unescaped_length - total_escaped_length
}

fn calculate_total_encoded_diff(strings: &Vec<String>) -> usize {
    let mut total_unescaped_length = 0;
    let mut total_escaped_length = 0;
    for s in strings {
        total_escaped_length += s.len();
        total_unescaped_length += get_encoded_string_length(s);
    }

    total_unescaped_length - total_escaped_length
}

fn get_encoded_string_length(string: &str) -> usize {
    let mut res = String::from("\"");
    for ch in string.chars() {
        if ch == '\\' || ch == '"' {
            res.push('\\');
        }
        res.push(ch);
    }
    res.push('"');
    res.len()
}

fn get_decoded_string_length(string: &str) -> usize {
    let mut count = 0;
    let mut res = String::new();
    let stripped_string = &string[1..string.len() - 1];
    let mut chars = stripped_string.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(next_ch) = chars.peek() {
                match next_ch {
                    '\\' | '"' => {
                        res.push(*next_ch);
                        count += 1;
                        chars.next();
                    }
                    'x' => {
                        chars.next();
                        let mut hex = String::new();
                        hex.push(chars.next().unwrap());
                        hex.push(chars.next().unwrap());

                        let value = i64::from_str_radix(&hex, 16).unwrap();
                        res.push(value as u8 as char);
                        count += 1;
                    }
                    _ => {
                        res.push(ch);
                        count += 1;
                    }
                }
            }
        } else {
            res.push(ch);
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_input1() {
        let strings = vec![
            format!("{}", r#""""#),
            format!("{}", r#""abc""#),
            format!("{}", r#""aaa\"aaa""#),
            format!("{}", r#""\x27""#),
        ];
        assert_eq!(calculate_total_decoded_diff(&strings), 12);
    }

    #[test]
    fn part2_input1() {
        let strings = vec![
            format!("{}", r#""""#),
            format!("{}", r#""abc""#),
            format!("{}", r#""aaa\"aaa""#),
            format!("{}", r#""\x27""#),
        ];
        assert_eq!(calculate_total_encoded_diff(&strings), 19);
    }
}
