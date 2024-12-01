use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn run() {
    let (mut relations, mut people) = read_input("input.txt");

    utils::run_solution!(
        || find_optimal_seating_arrangement(&people, &relations),
        "part1"
    );

    let me = String::from("Me");
    for person in people.iter() {
        relations.insert((me.to_string(), person.to_string()), 0);
        relations.insert((person.to_string(), me.to_string()), 0);
    }
    people.insert(me.to_string());

    utils::run_solution!(
        || find_optimal_seating_arrangement(&people, &relations),
        "part2"
    );
}

fn find_optimal_seating_arrangement(
    people: &HashSet<String>,
    relations: &HashMap<(String, String), i32>,
) -> i32 {
    let mut people_vec = Vec::new();
    for person in people {
        people_vec.push(person.to_string());
    }
    let mut permutations = get_permutations(
        // no need to generate all permutations, because we can have repeated order
        // therefore the first person will be fixed
        &people_vec.get(1..people_vec.len()).unwrap().to_vec(),
        people_vec.len() - 1,
    );

    let mut max_happiness = i32::min_value();
    for mut perm in permutations.iter_mut() {
        let mut every_person = vec![people_vec[0].clone()];
        every_person.append(&mut perm);
        let val = calculate_total_happiness(&every_person, relations);
        if val > max_happiness {
            max_happiness = val;
        }
    }

    max_happiness
}

fn calculate_total_happiness(
    people: &Vec<String>,
    relations: &HashMap<(String, String), i32>,
) -> i32 {
    let mut total = 0;
    let num_of_people = people.len();
    for i in 0..num_of_people {
        total += relations
            .get(&(people[i].clone(), people[(i + 1) % num_of_people].clone()))
            .unwrap()
            + relations
                .get(&(
                    people[i].clone(),
                    people[(i + num_of_people - 1) % num_of_people].clone(),
                ))
                .unwrap();
    }
    total
}

fn get_permutations(current_perm: &Vec<String>, n: usize) -> Vec<Vec<String>> {
    let mut results = Vec::new();
    if n == 1 {
        results.push(current_perm.to_vec());
        return results;
    }
    let mut new_perm = current_perm.to_vec();
    for i in 0..n - 1 {
        results.append(&mut get_permutations(&new_perm, n - 1));
        new_perm = results.last().unwrap().to_vec();

        if n % 2 == 0 {
            new_perm = swap(&new_perm, i, n - 1);
        } else {
            new_perm = swap(&new_perm, 0, n - 1);
        }
    }

    results.append(&mut get_permutations(&new_perm, n - 1));
    results
}

fn swap(perm: &Vec<String>, from: usize, to: usize) -> Vec<String> {
    let mut new_perm = perm.to_vec();
    let temp = new_perm[from].clone();
    new_perm[from] = new_perm[to].clone();
    new_perm[to] = temp;
    new_perm
}

fn read_input(filename: &str) -> (HashMap<(String, String), i32>, HashSet<String>) {
    let contents = utils::read_to_string_in_module!(filename);
    let re = Regex::new(r"(.+)\s.+\s(gain|lose)\s(\d+)\s.+\s(.+)\.").unwrap();
    let mut relations = HashMap::new();
    let mut people = HashSet::new();
    for string in contents.split_terminator('\n') {
        let cap = re.captures(&string).unwrap();
        let val = cap[3].parse::<i32>().unwrap();
        let real_val;
        if &cap[2] == "gain" {
            real_val = val;
        } else {
            real_val = -val;
        }
        people.insert(cap[1].to_string());
        relations.insert((cap[1].to_string(), cap[4].to_string()), real_val);
    }
    (relations, people)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_input1() {
        let (relations, people) = read_input("test-input.txt");

        assert_eq!(find_optimal_seating_arrangement(&people, &relations), 330);
    }
}
