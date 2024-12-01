use regex::Regex;
use std::cmp::min;

pub fn run() {
    let ingredients = read_input("input.txt");
    utils::run_solution!(|| calculate_best_ingredient_score(&ingredients), "part1");

    utils::run_solution!(
        || calculate_best_ingredient_score_with_calories(&ingredients),
        "part2"
    );
}

fn calculate_best_ingredient_score_with_calories(ingredients: &[Ingredient]) -> i32 {
    let n = ingredients.len();
    let partitions = get_unordered_partitions(100, n as i32);
    let permutations = get_permutations(&(0..n).collect::<Vec<_>>(), n);
    let mut max_score = 0;
    for p in partitions {
        for perm in &permutations {
            let score = get_single_partition_score(&p, ingredients, perm, Some(500));
            if score > max_score {
                max_score = score;
            }
        }
    }
    max_score
}

fn calculate_best_ingredient_score(ingredients: &[Ingredient]) -> i32 {
    let n = ingredients.len();
    let partitions = get_unordered_partitions(100, n as i32);
    let permutations = get_permutations(&(0..n).collect::<Vec<_>>(), n);
    let mut max_score = 0;
    for p in partitions {
        for perm in &permutations {
            let score = get_single_partition_score(&p, ingredients, perm, None);
            if score > max_score {
                max_score = score;
            }
        }
    }
    max_score
}

fn get_single_partition_score(
    p: &[i32],
    ingredients: &[Ingredient],
    perm: &[usize],
    calories_target: Option<i32>,
) -> i32 {
    let n = ingredients.len();
    let mut values = [0; 4];
    let mut total_calories = 0;
    for i in 0..n {
        values[0] += p[i] * ingredients[perm[i]].capacity;
        values[1] += p[i] * ingredients[perm[i]].durability;
        values[2] += p[i] * ingredients[perm[i]].flavor;
        values[3] += p[i] * ingredients[perm[i]].texture;
        total_calories += p[i] * ingredients[perm[i]].calories;
    }
    if let Some(target) = calories_target {
        if total_calories != target {
            return 0;
        }
    }
    values
        .iter()
        .fold(1, |acc, x| if x > &0 { acc * x } else { 0 })
}

fn get_unordered_partitions(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    for i in 1..=k {
        let mut temp = get_partitions_k(n, i, n);
        for p in temp.iter_mut() {
            for _ in 0..k - i {
                p.push(0);
            }
        }
        res.append(&mut temp);
    }
    res
}

fn get_partitions_k(n: i32, k: i32, pre: i32) -> Vec<Vec<i32>> {
    if k == 0 {
        if n == 0 {
            return vec![vec![]];
        }
        vec![]
    } else {
        (1..=min(n, pre))
            .flat_map(|i| {
                get_partitions_k(n - i, k - 1, i)
                    .into_iter()
                    .map(move |mut vec| {
                        vec.push(i);
                        vec
                    })
            })
            .collect()
    }
}

fn get_permutations(current_perm: &[usize], n: usize) -> Vec<Vec<usize>> {
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

fn swap(perm: &[usize], from: usize, to: usize) -> Vec<usize> {
    let mut new_perm = perm.to_vec();
    new_perm.swap(from, to);
    new_perm
}

fn read_input(filename: &str) -> Vec<Ingredient> {
    let mut items = Vec::new();
    let contents = utils::read_to_string_in_module!(filename);
    let re = Regex::new(r"(-?\d+?)").unwrap();
    for string in contents.split_terminator('\n') {
        let parts: Vec<&str> = string.split_terminator(':').collect();
        let mut values = Vec::new();
        for cap in re.captures_iter(parts[1]) {
            values.push(cap[1].parse::<i32>().unwrap());
        }
        items.push(Ingredient::new(
            values[0], values[1], values[2], values[3], values[4],
        ));
    }

    items
}

#[derive(Clone, Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}
impl Ingredient {
    pub fn new(
        capacity: i32,
        durability: i32,
        flavor: i32,
        texture: i32,
        calories: i32,
    ) -> Ingredient {
        Ingredient {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_input1() {
        let ingredients = read_input("test-input.txt");
        assert_eq!(calculate_best_ingredient_score(&ingredients), 62842880);
    }

    #[test]
    fn part2_input1() {
        let ingredients = read_input("test-input.txt");
        assert_eq!(
            calculate_best_ingredient_score_with_calories(&ingredients),
            57600000
        );
    }
}
