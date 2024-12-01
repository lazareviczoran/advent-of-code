use utils::read_to_string_in_module;

pub fn run() {
    let map = read("input.txt");
    println!("part1 solution: {}", count_trees(&map, &(3, 1)));
    println!("part2 solution: {}", count_trees_multi(&map));
}

fn count_trees(map: &[Vec<char>], step: &(usize, usize)) -> usize {
    let len_x = map[0].len();
    let mut x = 0;
    let mut res = 0;
    for row in map.iter().skip(step.1).step_by(step.1) {
        x = (x + step.0) % len_x;
        if row[x] == '#' {
            res += 1;
        }
    }
    res
}

fn count_trees_multi(map: &[Vec<char>]) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .fold(1, |acc, step| acc * count_trees(map, step))
}

fn read(filename: &str) -> Vec<Vec<char>> {
    read_to_string_in_module!(filename)
        .split_terminator('\n')
        .map(|s| s.chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let map = read("test-input1.txt");
        assert_eq!(count_trees(&map, &(3, 1)), 7);
    }

    #[test]
    fn part2_test() {
        let map = read("test-input1.txt");
        assert_eq!(count_trees_multi(&map), 336);
    }
}
