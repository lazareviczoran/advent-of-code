use std::collections::HashSet;

use utils::structs::Point;

pub fn run() {
    let mut map = read_input("input.txt");

    utils::run_solution!(|| count_visited(&map), "part1");
    utils::run_solution!(|| count_obstacles(&mut map), "part2");
}

fn count_visited(map: &[Vec<char>]) -> usize {
    traverse(map)
        .expect("no cycle")
        .into_iter()
        .map(|(pos, _)| pos)
        .collect::<HashSet<_>>()
        .len()
}

fn count_obstacles(map: &mut [Vec<char>]) -> usize {
    let path = traverse(map).expect("no cycle");
    let mut visited: HashSet<Point<2, _>> = HashSet::new();
    path.iter()
        .skip(1)
        .filter_map(|(pos, _)| {
            if visited.contains(pos) {
                return None;
            }
            visited.insert(*pos);
            let old = map[pos.get('x')? as usize][pos.get('y')? as usize];
            let is_starting_pos = &path[0].0 == pos;
            let a = map[path[0].0.get('x')? as usize][path[0].0.get('y')? as usize];
            let b = map[path[1].0.get('x')? as usize][path[1].0.get('y')? as usize];
            if is_starting_pos {
                map[path[0].0.get('x')? as usize][path[0].0.get('y')? as usize] = b;
                map[path[1].0.get('x')? as usize][path[1].0.get('y')? as usize] = a;
            }
            map[pos.get('x')? as usize][pos.get('y')? as usize] = '#';
            let is_looping = traverse(map).map(|_| None).unwrap_or(Some(()));
            if is_starting_pos {
                map[path[0].0.get('x')? as usize][path[0].0.get('y')? as usize] = a;
                map[path[1].0.get('x')? as usize][path[1].0.get('y')? as usize] = b;
            }
            map[pos.get('x')? as usize][pos.get('y')? as usize] = old;
            is_looping
        })
        .count()
}

fn traverse(map: &[Vec<char>]) -> Option<Vec<(Point<2, isize>, char)>> {
    let start_pos = map
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find(|(_, &ch)| matches!(ch, '<' | '>' | 'v' | '^'))
                .map(|(j, _)| Point::new([i as isize, j as isize]))
        })
        .expect("start position not found");

    let dirs: [Direction; 4] = [
        Direction::new(-1, 0, '^'),
        Direction::new(0, 1, '>'),
        Direction::new(1, 0, 'v'),
        Direction::new(0, -1, '<'),
    ];
    let is_in_bounds = |p: Point<2, _>| {
        Some(
            p.get('x')? >= 0
                && p.get('x')? < map.len() as isize
                && p.get('y')? >= 0
                && p.get('y')? < map[0].len() as isize,
        )
    };
    let mut curr_pos = start_pos;
    let mut iter = dirs.iter().cycle();
    let mut curr_dir = iter.find(|dir| {
        dir.ch == map[curr_pos.get('x').unwrap() as usize][curr_pos.get('y').unwrap() as usize]
    })?;
    let mut path = Vec::new();

    let mut visited: HashSet<(Point<2, _>, char)> = HashSet::new();
    while is_in_bounds(curr_pos)? {
        if !visited.insert((curr_pos, curr_dir.ch)) {
            return None;
        }
        if !matches!(path.last(), Some(&(point, _)) if point == curr_pos) {
            path.push((curr_pos, curr_dir.ch));
        }
        let next = curr_dir.next(curr_pos)?;
        if is_in_bounds(next)? && map[next.get('x')? as usize][next.get('y')? as usize] == '#' {
            curr_dir = iter.next()?;
        } else {
            curr_pos = next;
        }
    }

    Some(path)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Direction {
    dir: Point<2, isize>,
    ch: char,
}
impl Direction {
    fn new(x: isize, y: isize, ch: char) -> Self {
        Self {
            dir: Point::new([x, y]),
            ch,
        }
    }

    fn next(&self, point: Point<2, isize>) -> Option<Point<2, isize>> {
        Some(Point::new([
            point.get('x')? + self.dir.get('x')?,
            point.get('y')? + self.dir.get('y')?,
        ]))
    }
}

fn read_input(filename: &str) -> Vec<Vec<char>> {
    utils::read_to_string_in_module!(filename)
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::count_obstacles;
    use super::count_visited;
    use super::read_input;

    #[test]
    fn tests() {
        let mut map = read_input("test-input.txt");
        assert_eq!(count_visited(&map), 41);
        assert_eq!(count_obstacles(&mut map), 6);
    }

    #[test]
    fn prod() {
        let mut map = read_input("input.txt");
        assert_eq!(count_visited(&map), 4580);
        assert_eq!(count_obstacles(&mut map), 1480);
    }
}
