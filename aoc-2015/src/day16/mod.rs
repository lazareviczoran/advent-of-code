use regex::Regex;
use std::collections::HashMap;

pub fn run() {
    let aunts = read_input("input.txt");
    let match_values = vec![
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ];

    utils::run_solution!(|| find_correct_aunt(&aunts, &match_values), "part1");

    utils::run_solution!(|| find_correct_aunt_v2(&aunts, &match_values), "part2");
}

fn find_correct_aunt(
    aunts: &HashMap<i32, HashMap<String, i32>>,
    match_values: &Vec<(&str, i32)>,
) -> i32 {
    for (aunt, attrs) in aunts.iter() {
        let mut is_match = true;
        for (match_attr, match_attr_val) in match_values.iter() {
            if let Some(val) = attrs.get(*match_attr) {
                is_match = is_match && *val == *match_attr_val;
                if !is_match {
                    break;
                }
            }
        }
        if is_match {
            return *aunt;
        }
    }
    // no matching aunt found
    return -1;
}

fn find_correct_aunt_v2(
    aunts: &HashMap<i32, HashMap<String, i32>>,
    match_values: &Vec<(&str, i32)>,
) -> i32 {
    for (aunt, attrs) in aunts.iter() {
        let mut is_match = true;
        for (match_attr, match_attr_val) in match_values.iter() {
            if let Some(val) = attrs.get(*match_attr) {
                match *match_attr {
                    "cats" | "trees" => is_match = is_match && *val > *match_attr_val,
                    "pomeranians" | "goldfish" => is_match = is_match && *val < *match_attr_val,
                    _ => is_match = is_match && *val == *match_attr_val,
                }
                if !is_match {
                    break;
                }
            }
        }
        if is_match {
            return *aunt;
        }
    }
    // no matching aunt found
    return -1;
}

fn read_input(filename: &str) -> HashMap<i32, HashMap<String, i32>> {
    let mut aunts = HashMap::new();
    let contents = utils::read_to_string_in_module!(filename);
    let re = Regex::new(r"(\d+?):(.*)").unwrap();
    let attr_re = Regex::new(r"\s(.+?):\s(\d+)").unwrap();
    for string in contents.split_terminator('\n') {
        let captures = re.captures(&string).unwrap();
        let aunt = &captures[1].parse::<i32>().unwrap();
        let mut aunt_attrs = HashMap::new();
        for cap in attr_re.captures_iter(&captures[2]) {
            aunt_attrs.insert(cap[1].to_string(), cap[2].parse::<i32>().unwrap());
        }
        aunts.insert(*aunt, aunt_attrs);
    }

    aunts
}
