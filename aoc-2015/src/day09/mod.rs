use regex::Regex;
use std::collections::HashMap;

pub fn run() {
    let contents = utils::read_to_string_in_module!("input.txt");
    let distances_map = load_distances_map(&contents);

    utils::run_solution!(
        || find_route_distance(&distances_map, TspType::Shortest),
        "part1"
    );

    utils::run_solution!(
        || find_route_distance(&distances_map, TspType::Longest),
        "part2"
    );
}

fn find_route_distance(
    distances_map: &HashMap<String, HashMap<String, usize>>,
    tsp_type: TspType,
) -> usize {
    let mut all_places: HashMap<String, usize> = HashMap::new();
    for key in distances_map.keys() {
        all_places.insert(key.to_string(), all_places.len());
    }
    let mut distances_from_each_place = HashMap::new();
    for (place, i) in all_places.clone() {
        let mask = 0;
        let mut dp = HashMap::new();
        distances_from_each_place.insert(
            place.clone(),
            tsp(
                set_bit(mask, i as u8),
                place,
                &all_places,
                &distances_map,
                &mut dp,
                &tsp_type,
            ),
        );
    }

    let mut res;
    match tsp_type {
        TspType::Shortest => {
            res = usize::max_value();
            for (_, dist) in distances_from_each_place.iter() {
                if *dist < res {
                    res = *dist;
                }
            }
        }
        TspType::Longest => {
            res = 0;
            for (_, dist) in distances_from_each_place.iter() {
                if *dist > res {
                    res = *dist;
                }
            }
        }
    }
    res
}

enum TspType {
    Shortest,
    Longest,
}

fn tsp(
    mask: usize,
    curr_place: String,
    all_places: &HashMap<String, usize>,
    distances: &HashMap<String, HashMap<String, usize>>,
    dp: &mut HashMap<(usize, String), usize>,
    tsp_type: &TspType,
) -> usize {
    if let Some(value) = dp.get(&(mask, curr_place.clone())) {
        return *value;
    }
    let mut ans;
    match tsp_type {
        TspType::Shortest => ans = usize::max_value(),
        TspType::Longest => ans = 0,
    }

    // for each unvisited and unlocked
    let mut neighbours = all_places.clone();
    neighbours.retain(|k, order| {
        let distance_between = distances.get(&curr_place).unwrap().get(k);
        !has_bit(mask, *order as u8) && distance_between.is_some()
    });
    if neighbours.len() == 0 {
        return 0;
    }
    for (v, order) in neighbours.iter() {
        let city = *order as u8;
        if mask & (1 << city) == 0 {
            let best_dist = tsp(
                mask | (1 << city),
                v.clone(),
                &all_places,
                &distances,
                dp,
                &tsp_type,
            );
            let new_ans = distances.get(&curr_place).unwrap().get(v).unwrap() + best_dist;
            match tsp_type {
                TspType::Shortest => {
                    if ans > new_ans {
                        ans = new_ans;
                    }
                }
                TspType::Longest => {
                    if ans < new_ans {
                        ans = new_ans;
                    }
                }
            }
        }
    }
    dp.insert((mask, curr_place), ans);
    ans
}

#[allow(unused)]
fn unset_bit(keys: usize, i: u8) -> usize {
    keys & !(1 << i)
}

fn set_bit(keys: usize, i: u8) -> usize {
    keys | (1 << i)
}

fn has_bit(keys: usize, i: u8) -> bool {
    keys & (1 << i) == (1 << i)
}

fn load_distances_map(content: &String) -> HashMap<String, HashMap<String, usize>> {
    let mut distances_map: HashMap<String, HashMap<String, usize>> = HashMap::new();
    let re = Regex::new(r"(.+)\sto\s(.+)\s=\s(\d+)").unwrap();

    for d in content.split_terminator('\n') {
        let captures = re.captures(&d).unwrap();
        let from = captures[1].to_string();
        let to = captures[2].to_string();
        let dist = captures[3].parse::<usize>().unwrap();
        if let Some(val) = distances_map.get_mut(&from) {
            val.insert(to.clone(), dist);
        } else {
            let mut new_map = HashMap::new();
            new_map.insert(to.clone(), dist);
            distances_map.insert(from.clone(), new_map);
        }
        if let Some(val) = distances_map.get_mut(&to) {
            val.insert(from, dist);
        } else {
            let mut new_map = HashMap::new();
            new_map.insert(from, dist);
            distances_map.insert(to, new_map);
        }
    }

    distances_map
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_input1() {
        let content = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141";
        let map = load_distances_map(&content.to_string());
        assert_eq!(find_route_distance(&map, TspType::Shortest), 605);
    }

    #[test]
    fn part2_input1() {
        let content = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141";
        let map = load_distances_map(&content.to_string());
        assert_eq!(find_route_distance(&map, TspType::Longest), 982);
    }
}
