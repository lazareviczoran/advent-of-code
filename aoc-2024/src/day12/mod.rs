const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn run() {
    let map = read_input("input.txt");

    utils::run_solution!(|| calc_price(&map), "part1");
    utils::run_solution!(|| calc_new_price(&map), "part2");
}

fn calc_price(map: &[Vec<char>]) -> usize {
    let mut visited: std::collections::HashSet<(isize, isize)> =
        std::collections::HashSet::default();
    let mut areas: std::collections::HashSet<Region> = std::collections::HashSet::new();
    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (i, row) in map.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            let (i, j) = (i as isize, j as isize);
            if visited.contains(&(i, j)) {
                continue;
            }
            let mut curr_area: std::collections::HashSet<(isize, isize)> =
                std::collections::HashSet::from_iter([(i, j)]);
            let mut q = vec![(i, j)];
            while let Some((x, y)) = q.pop() {
                if visited.contains(&(x, y)) {
                    continue;
                }
                visited.insert((x, y));
                curr_area.insert((x, y));
                for (dx, dy) in dirs {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx < 0
                        || ny < 0
                        || nx >= map.len() as isize
                        || ny >= map[0].len() as isize
                        || map[nx as usize][ny as usize] != c
                    {
                        continue;
                    }
                    q.push((nx, ny));
                }
            }
            areas.insert(Region::new(c, curr_area));
        }
    }
    areas
        .iter()
        .map(|area| area.area() * area.perimeter())
        .sum()
}

fn calc_new_price(map: &[Vec<char>]) -> usize {
    let mut visited: std::collections::HashSet<(isize, isize)> =
        std::collections::HashSet::default();
    let mut areas: std::collections::HashSet<Region> = std::collections::HashSet::new();
    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (i, row) in map.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            let (i, j) = (i as isize, j as isize);
            if visited.contains(&(i, j)) {
                continue;
            }
            let mut curr_area: std::collections::HashSet<(isize, isize)> =
                std::collections::HashSet::from_iter([(i, j)]);
            let mut q = vec![(i, j)];
            while let Some((x, y)) = q.pop() {
                if visited.contains(&(x, y)) {
                    continue;
                }
                visited.insert((x, y));
                curr_area.insert((x, y));
                for (dx, dy) in dirs {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx < 0
                        || ny < 0
                        || nx >= map.len() as isize
                        || ny >= map[0].len() as isize
                        || map[nx as usize][ny as usize] != c
                    {
                        continue;
                    }
                    q.push((nx, ny));
                }
            }
            areas.insert(Region::new(c, curr_area));
        }
    }
    areas
        .iter()
        .map(|area| area.area() * area.perimeter_borders())
        .sum()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Region {
    ch: char,
    points: std::collections::HashSet<(isize, isize)>,
}
impl Region {
    fn new(ch: char, points: std::collections::HashSet<(isize, isize)>) -> Self {
        Region { ch, points }
    }

    fn area(&self) -> usize {
        self.points.len()
    }

    fn perimeter(&self) -> usize {
        self.points
            .iter()
            .map(|&(x, y)| {
                DIRS.iter()
                    .filter(|(dx, dy)| !self.points.contains(&(x + dx, y + dy)))
                    .count()
            })
            .sum::<usize>()
    }

    fn perimeter_borders(&self) -> usize {
        let border_points = self
            .points
            .iter()
            .flat_map(|&(x, y)| {
                let has_up = self.points.contains(&(x - 1, y));
                let has_down = self.points.contains(&(x + 1, y));
                let has_left = self.points.contains(&(x, y - 1));
                let has_right = self.points.contains(&(x, y + 1));
                let has_diagonal_up_left = self.points.contains(&(x - 1, y - 1));
                let has_diagonal_up_right = self.points.contains(&(x - 1, y + 1));
                let has_diagonal_down_right = self.points.contains(&(x + 1, y + 1));
                let has_diagonal_down_left = self.points.contains(&(x + 1, y - 1));
                let mut border_points = std::collections::HashMap::new();
                if !has_up {
                    border_points.insert((x * 3 - 1, 3 * y), '-');
                }
                if !has_down {
                    border_points.insert((x * 3 + 1, 3 * y), '-');
                }
                if !has_left {
                    border_points.insert((x * 3, 3 * y - 1), '|');
                }
                if !has_right {
                    border_points.insert((x * 3, 3 * y + 1), '|');
                }
                if has_down && has_left && !has_diagonal_down_left {
                    border_points.insert((x * 3 + 1, 3 * y - 1), '+');
                }
                if has_down && has_right && !has_diagonal_down_right {
                    border_points.insert((x * 3 + 1, 3 * y + 1), '+');
                }
                if has_up && has_left && !has_diagonal_up_left {
                    border_points.insert((x * 3 - 1, 3 * y - 1), '+');
                }
                if has_up && has_right && !has_diagonal_up_right {
                    border_points.insert((x * 3 - 1, 3 * y + 1), '+');
                }
                if !has_up && !has_left {
                    border_points.insert((x * 3 - 1, 3 * y - 1), '+');
                }
                if !has_up && !has_right {
                    border_points.insert((x * 3 - 1, 3 * y + 1), '+');
                }
                if !has_down && !has_left {
                    border_points.insert((x * 3 + 1, 3 * y - 1), '+');
                }
                if !has_down && !has_right {
                    border_points.insert((x * 3 + 1, 3 * y + 1), '+');
                }
                if has_up && !has_left {
                    border_points.insert((x * 3 - 1, 3 * y - 1), '|');
                }
                if has_up && !has_right {
                    border_points.insert((x * 3 - 1, 3 * y + 1), '|');
                }
                if has_down && !has_left {
                    border_points.insert((x * 3 + 1, 3 * y - 1), '|');
                }
                if has_down && !has_right {
                    border_points.insert((x * 3 + 1, 3 * y + 1), '|');
                }
                if !has_up && has_left {
                    border_points.insert((x * 3 - 1, 3 * y - 1), '-');
                }
                if !has_up && has_right {
                    border_points.insert((x * 3 - 1, 3 * y + 1), '-');
                }
                if !has_down && has_left {
                    border_points.insert((x * 3 + 1, 3 * y - 1), '-');
                }
                if !has_down && has_right {
                    border_points.insert((x * 3 + 1, 3 * y + 1), '-');
                }

                border_points
            })
            .collect::<std::collections::HashMap<_, _>>();
        let mut sides = 0;
        let mut visited = std::collections::HashSet::new();
        while visited.len() < border_points.len() {
            let start = border_points
                .iter()
                .filter(|(&curr, _)| !visited.contains(&curr))
                .fold(
                    (isize::MAX, isize::MAX),
                    |curr_min, (&curr, _)| match curr_min.0.cmp(&curr.0) {
                        std::cmp::Ordering::Less => curr_min,
                        std::cmp::Ordering::Equal => (curr_min.0, curr_min.1.min(curr.1)),
                        std::cmp::Ordering::Greater => curr,
                    },
                );
            let mut queue = vec![(start, DIRS[3])];
            while let Some(((x, y), curr_dir)) = queue.pop() {
                if visited.contains(&(x, y)) {
                    continue;
                }
                visited.insert((x, y));
                let (next_x, next_y) = (x + curr_dir.0, y + curr_dir.1);
                if !matches!(border_points.get(&(next_x, next_y)), Some('+')) {
                    queue.push(((next_x, next_y), curr_dir));
                    continue;
                }
                sides += 1;
                if [DIRS[3], DIRS[2]].contains(&curr_dir) {
                    for dir in [DIRS[0], DIRS[1]] {
                        if matches!(
                            border_points.get(&(next_x + dir.0, next_y + dir.1)),
                            Some('|')
                        ) {
                            queue.push(((next_x, next_y), dir));
                        }
                    }
                }
                if [DIRS[0], DIRS[1]].contains(&curr_dir) {
                    for dir in [DIRS[2], DIRS[3]] {
                        if matches!(
                            border_points.get(&(next_x + dir.0, next_y + dir.1)),
                            Some('-')
                        ) {
                            queue.push(((next_x, next_y), dir));
                        }
                    }
                }
            }
        }
        sides
    }
}
impl std::hash::Hash for Region {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ch.hash(state);
        self.points.iter().for_each(|p| {
            p.hash(state);
        });
    }
}

fn read_input(filename: &str) -> Vec<Vec<char>> {
    utils::read_to_string_in_module!(filename)
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::calc_new_price;
    use super::calc_price;
    use super::read_input;

    #[test]
    fn p1() {
        let map = read_input("test-input.txt");
        assert_eq!(calc_price(&map), 140);
        let map = read_input("test-input2.txt");
        assert_eq!(calc_price(&map), 772);
        let map = read_input("test-input3.txt");
        assert_eq!(calc_price(&map), 1930);
    }

    #[test]
    fn p2() {
        let map = read_input("test-input.txt");
        assert_eq!(calc_new_price(&map), 80);
        let map = read_input("test-input4.txt");
        assert_eq!(calc_new_price(&map), 236);
        let map = read_input("test-input5.txt");
        assert_eq!(calc_new_price(&map), 368);
        let map = read_input("test-input3.txt");
        assert_eq!(calc_new_price(&map), 1206);
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
        let map = read_input("input.txt");
        assert_eq!(calc_price(&map), pt1);
        assert_eq!(calc_new_price(&map), pt2);
    }
}
