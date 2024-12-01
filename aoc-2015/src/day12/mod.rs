use regex::Regex;
use serde_json::Value;

pub fn run() {
    let contents = utils::read_to_string_in_module!("input.txt");

    utils::run_solution!(|| calculate_total(&contents), "part1");

    utils::run_solution!(|| calculate_total_json(&contents), "part1 v2");

    utils::run_solution!(|| calculate_total_without_red(&contents), "part2");
}

fn calculate_total(input: &str) -> i32 {
    let re = Regex::new(r"(-?\d+)").unwrap();
    let mut total = 0;
    for cap in re.captures_iter(input) {
        total += &cap[1].parse::<i32>().unwrap();
    }
    total
}

fn calculate_total_json(input: &str) -> i64 {
    let json_value: Value = serde_json::from_str(input).unwrap();
    calculate_total2(&json_value)
}

fn calculate_total2(json_value: &Value) -> i64 {
    if json_value.is_number() {
        return json_value.as_i64().unwrap();
    }
    let mut total = 0;
    if json_value.is_array() {
        for item in json_value.as_array().unwrap() {
            total += calculate_total2(&item);
        }
    } else if json_value.is_object() {
        for (_key, val) in json_value.as_object().unwrap() {
            total += calculate_total2(val);
        }
    }
    total
}

fn calculate_total_without_red(input: &str) -> i64 {
    let json_value: Value = serde_json::from_str(input).unwrap();
    calculate_total_without_red_rec(&json_value)
}

fn calculate_total_without_red_rec(json_value: &Value) -> i64 {
    if json_value.is_number() {
        return json_value.as_i64().unwrap();
    }
    let mut total = 0;
    if json_value.is_array() {
        for item in json_value.as_array().unwrap() {
            total += calculate_total_without_red_rec(&item);
        }
    } else if json_value.is_object() {
        let map = json_value.as_object().unwrap();
        for val in map.values() {
            if val.is_string() && val.as_str().unwrap() == "red" {
                return 0;
            }
        }
        for (_key, val) in map {
            total += calculate_total_without_red_rec(val);
        }
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_input1() {
        assert_eq!(calculate_total(r"[1,2,3]"), 6);
        assert_eq!(calculate_total(r#"{"a":2,"b":4}"#), 6);
        assert_eq!(calculate_total(r"[[[3]]]"), 3);
        assert_eq!(calculate_total(r#"{"a":{"b":4},"c":-1}"#), 3);
        assert_eq!(calculate_total(r#"{"a":[-1,1]}"#), 0);
        assert_eq!(calculate_total(r#"[-1,{"a":1}]"#), 0);
        assert_eq!(calculate_total(r"[]"), 0);
        assert_eq!(calculate_total(r#"{}"#), 0);
    }

    #[test]
    fn part1_input2() {
        assert_eq!(calculate_total_json(r"[1,2,3]"), 6);
        assert_eq!(calculate_total_json(r#"{"a":2,"b":4}"#), 6);
        assert_eq!(calculate_total_json(r"[[[3]]]"), 3);
        assert_eq!(calculate_total_json(r#"{"a":{"b":4},"c":-1}"#), 3);
        assert_eq!(calculate_total_json(r#"{"a":[-1,1]}"#), 0);
        assert_eq!(calculate_total_json(r#"[-1,{"a":1}]"#), 0);
        assert_eq!(calculate_total_json(r"[]"), 0);
        assert_eq!(calculate_total_json(r#"{}"#), 0);
    }

    #[test]
    fn part2_input1() {
        assert_eq!(calculate_total_without_red(r"[1,2,3]"), 6);
        assert_eq!(calculate_total_without_red(r#"[1,{"c":"red","b":2},3]"#), 4);
        assert_eq!(
            calculate_total_without_red(r#"{"d":"red","e":[1,2,3,4],"f":5}"#),
            0
        );
        assert_eq!(calculate_total_without_red(r#"[1,"red",5]"#), 6);
    }
}
