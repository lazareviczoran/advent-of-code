use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn run() {
    let (transformations, initial_config) = read_input("input.txt");

    utils::run_solution!(
        || find_distinct_molecules(
            &transformations
                .clone()
                .into_iter()
                .flat_map(|(_, v)| v)
                .collect(),
            &initial_config
        )
        .len(),
        "part1"
    );

    utils::run_solution!(
        || find_fewest_steps_to_medicine(&transformations, &initial_config),
        "part2"
    );
}

fn find_fewest_steps_to_medicine(
    transformations: &HashMap<String, Vec<Transformation>>,
    initial_config: &str,
) -> usize {
    let target_str = String::from("e");
    let mut reverse_transformations = HashMap::new();
    for v in transformations.values() {
        for el in v.iter() {
            let t = Transformation::new(el.to.clone(), el.from.clone());
            reverse_transformations.insert(t.from.clone(), t);
        }
    }
    let mut froms: Vec<String> = reverse_transformations.keys().cloned().collect();
    froms.sort_by_key(|a| a.len());
    let mut min_steps_count = usize::MAX;
    let mut config = initial_config.to_string();
    let mut nth_perm = 0;
    let mut steps_count = 0;
    let mut permutation = froms.clone();
    while config != target_str {
        if config.contains(&target_str) {
            if nth_perm >= factorial(froms.len() as u64) {
                panic!("no possible way to get to target pos");
            }
            config = initial_config.to_string();
            permutation = get_nth_permutation(&froms, nth_perm);
            steps_count = 0;
            nth_perm += 1;
            continue;
        }
        for from in &permutation {
            if config.contains(from) {
                let next_tr = reverse_transformations.get(from).unwrap();
                config = config.replacen(&next_tr.from, &next_tr.to, 1);
                steps_count += 1;
                break;
            }
        }
        if config == target_str && steps_count < min_steps_count {
            min_steps_count = steps_count;
        }
    }
    min_steps_count
}

fn get_nth_permutation(items: &[String], n: u64) -> Vec<String> {
    let mut m = n;
    let mut s = items.to_vec();
    let mut p = Vec::new();
    while !s.is_empty() {
        let f = factorial(s.len() as u64 - 1);
        let i = m / f;
        let x = s[i as usize].clone();
        m %= f;
        p.push(x);
        s.remove(i as usize);
    }
    p
}

fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

fn find_distinct_molecules(
    transformations: &Vec<Transformation>,
    initial_config: &str,
) -> Vec<String> {
    let mut res = HashSet::new();
    for t in transformations {
        let matches: Vec<(usize, &str)> = initial_config.match_indices(&t.from).collect();
        for (i, _) in matches {
            res.insert(get_new_config(initial_config, t, i));
        }
    }

    res.into_iter().collect()
}

fn get_new_config(initial_config: &str, t: &Transformation, i: usize) -> String {
    let mut config = String::new();
    config.push_str(&initial_config[0..i]);
    config.push_str(&t.to);
    config.push_str(&initial_config[i + t.from.len()..]);
    config
}

fn read_input(filename: &str) -> (HashMap<String, Vec<Transformation>>, String) {
    let transf_re = Regex::new(r"(.+?)\s=>\s(.+)").unwrap();
    let contents = utils::read_to_string_in_module!(filename);
    let mut transformations: HashMap<String, Vec<Transformation>> = HashMap::new();
    let mut initial_config = String::new();
    for x in contents.split_terminator('\n') {
        if x != "\n" {
            if let Some(captures) = transf_re.captures(x) {
                let new_transformation =
                    Transformation::new(captures[1].to_string(), captures[2].to_string());
                if let Some(vector) = transformations.get_mut(&captures[1]) {
                    vector.push(new_transformation);
                } else {
                    transformations.insert(captures[1].to_string(), vec![new_transformation]);
                }
            } else {
                initial_config = x.to_string();
            }
        }
    }
    (transformations, initial_config)
}

#[derive(Debug, Clone)]
struct Transformation {
    from: String,
    to: String,
}
impl Transformation {
    pub fn new(from: String, to: String) -> Transformation {
        Transformation { from, to }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_input1() {
        let (transformations, initial_config) = read_input("test-input.txt");
        let mut configs = find_distinct_molecules(
            &transformations.into_iter().flat_map(|(_, v)| v).collect(),
            &initial_config,
        );
        assert_eq!(configs.len(), 4);
        configs.sort();
        let mut expected = ["HOOH", "HOHO", "OHOH", "HHHH"];
        expected.sort();
        assert_eq!(configs, expected);
    }

    #[test]
    fn part1_input2() {
        let (transformations, initial_config) = read_input("test-input2.txt");
        assert_eq!(
            find_distinct_molecules(
                &transformations.into_iter().flat_map(|(_, v)| v).collect(),
                &initial_config
            )
            .len(),
            7
        );
    }

    #[test]
    fn part1_input3() {
        let (transformations, initial_config) = read_input("test-input3.txt");
        assert_eq!(
            find_distinct_molecules(
                &transformations.into_iter().flat_map(|(_, v)| v).collect(),
                &initial_config
            ),
            ["OO2O"]
        );
    }

    #[test]
    fn part2_input1() {
        let (transformations, initial_config) = read_input("test-input4.txt");
        assert_eq!(
            find_fewest_steps_to_medicine(&transformations, &initial_config),
            3
        );
    }

    #[test]
    fn part2_input2() {
        let (transformations, initial_config) = read_input("test-input5.txt");
        assert_eq!(
            find_fewest_steps_to_medicine(&transformations, &initial_config),
            6
        );
    }
}
