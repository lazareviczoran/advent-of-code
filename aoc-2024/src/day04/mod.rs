pub fn run() {
    let input = read_input("input.txt");
    utils::run_solution!(|| count_xmas(&input), "part1");
    utils::run_solution!(|| count_x_mas(&input), "part2");
}

fn count_xmas(input: &[Vec<char>]) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(x, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c == 'X')
                .map(|(y, _)| count_matches((x, y), input))
                .sum::<usize>()
        })
        .sum()
}

fn count_matches((x, y): (usize, usize), map: &[Vec<char>]) -> usize {
    [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ]
    .iter()
    .filter(|&(dir_x, dir_y)| {
        let (x, y) = (x as isize, y as isize);
        let x_range = match dir_x.cmp(&0) {
            std::cmp::Ordering::Less => ((x - 3).max(0)..=x).rev().take(4).collect(),
            std::cmp::Ordering::Equal => vec![x; 4],
            std::cmp::Ordering::Greater => (x..=(x + 3).min((map.len() - 1) as isize))
                .take(4)
                .collect(),
        };
        let y_range = match dir_y.cmp(&0) {
            std::cmp::Ordering::Less => ((y - 3).max(0)..=y).rev().take(4).collect(),
            std::cmp::Ordering::Equal => vec![y; 4],
            std::cmp::Ordering::Greater => (y..=(y + 3).min((map[0].len() - 1) as isize))
                .take(4)
                .collect(),
        };

        let word = x_range
            .iter()
            .zip(y_range.iter())
            .fold(String::new(), |mut acc, (&x, &y)| {
                acc.push(map[x as usize][y as usize]);
                acc
            });

        "XMAS" == word
    })
    .count()
}

fn count_x_mas(map: &[Vec<char>]) -> usize {
    let patterns = [
        [
            ['M', '.', 'S'], // keep
            ['.', 'A', '.'], // keep
            ['M', '.', 'S'], // keep
        ],
        [
            ['M', '.', 'M'], // keep
            ['.', 'A', '.'], // keep
            ['S', '.', 'S'], // keep
        ],
        [
            ['S', '.', 'M'], // keep
            ['.', 'A', '.'], // keep
            ['S', '.', 'M'], // keep
        ],
        [
            ['S', '.', 'S'], // keep
            ['.', 'A', '.'], // keep
            ['M', '.', 'M'], // keep
        ],
    ];
    (1..map.len() - 1)
        .map(|x| {
            (1..map[0].len() - 1)
                .filter(|&y| map[x][y] == 'A')
                .map(|y| {
                    patterns
                        .iter()
                        .filter(|pattern| {
                            map[x - 1][y - 1] == pattern[0][0]
                                && map[x - 1][y + 1] == pattern[0][2]
                                && map[x][y] == pattern[1][1]
                                && map[x + 1][y - 1] == pattern[2][0]
                                && map[x + 1][y + 1] == pattern[2][2]
                        })
                        .count()
                })
                .sum::<usize>()
        })
        .sum()
}

fn read_input(filename: &str) -> Vec<Vec<char>> {
    utils::read_to_string_in_module!(filename)
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day04::count_x_mas;

    use super::count_xmas;
    use super::read_input;

    #[test]
    fn p1() {
        assert_eq!(count_xmas(&read_input("test-input.txt")), 18);
        assert_eq!(count_xmas(&read_input("test-input2.txt")), 4);
    }

    #[test]
    fn p2() {
        assert_eq!(count_x_mas(&read_input("test-input.txt")), 9);
    }

    #[test]
    #[cfg(feature = "include-main-input")]
    fn prod() {
        use itertools::Itertools;
        let (pt1, pt2) = utils::read_to_string_in_module!("results.txt")
            .lines()
            .map(|line| line.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        assert_eq!(count_xmas(&read_input("input.txt")), pt1);
        assert_eq!(count_x_mas(&read_input("input.txt")), pt2);
    }
}
