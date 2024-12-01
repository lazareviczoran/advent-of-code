pub fn run() {
    let mut input = "1113122113";
    let mut res = String::new();
    for _ in 0..40 {
        res = convert_num(&input);
        input = res.as_str();
    }
    utils::run_solution!(|| res.len(), "part1");

    input = "1113122113";
    for _ in 0..50 {
        res = convert_num(&input);
        input = res.as_str();
    }
    utils::run_solution!(|| res.len(), "part2");
}

fn convert_num(number: &str) -> String {
    let mut res = String::new();
    let mut chars = number.chars().peekable();
    let mut repeats = 1;
    while let Some(ch) = chars.next() {
        if let Some(next_ch) = chars.peek() {
            if ch != *next_ch {
                res.push_str(&repeats.to_string());
                res.push(ch);
                repeats = 1;
            } else {
                repeats += 1;
            }
        } else {
            res.push_str(&repeats.to_string());
            res.push(ch);
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_input1() {
        assert_eq!(convert_num("1"), "11");
        assert_eq!(convert_num("11"), "21");
        assert_eq!(convert_num("21"), "1211");
        assert_eq!(convert_num("1211"), "111221");
        assert_eq!(convert_num("111221"), "312211");
    }
}
