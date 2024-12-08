use itertools::Itertools;
use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;
use utils::structs::Point;

pub fn run() {
    let mut map = read_input("input.txt");

    utils::run_solution!(|| map.count_unique_antinodes_positions(false), "part1");
    utils::run_solution!(|| { map.count_unique_antinodes_positions(true) }, "part2");
}

#[derive(Debug)]
struct Map {
    items: Vec<Vec<char>>,
    frequencies: FxHashMap<char, FxHashSet<Point<2, isize>>>,
    antinodes: FxHashMap<Point<2, isize>, FxHashSet<char>>,
}
impl Map {
    fn new(items: Vec<Vec<char>>) -> Self {
        let frequencies: FxHashMap<char, FxHashSet<Point<2, isize>>> = items
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, ch)| ch.is_alphanumeric())
                    .map(|(j, &ch)| (ch, Point::new([i as isize, j as isize])))
                    .collect::<Vec<_>>()
            })
            .fold(Default::default(), |mut acc, (freq, pos)| {
                acc.entry(freq).or_default().insert(pos);
                acc
            });

        Self {
            items,
            frequencies,
            antinodes: Default::default(),
        }
    }

    fn count_unique_antinodes_positions(&mut self, proceed: bool) -> usize {
        self.locate_antinodes(proceed);
        self.antinodes.len()
    }

    fn locate_antinodes(&mut self, proceed: bool) {
        self.frequencies.iter().for_each(|(&freq, locations)| {
            locations.iter().combinations(2).for_each(|mut pairs| {
                pairs.sort_by(|&&a, &&b| {
                    a.get('x')
                        .cmp(&b.get('x'))
                        .then(a.get('y').cmp(&b.get('y')))
                });
                let line = Line::new(*pairs[0], *pairs[1]);
                if !proceed {
                    let next = line.next().unwrap();
                    if is_in_bounds(&self.items, &next).unwrap() {
                        self.antinodes.entry(next).or_default().insert(freq);
                    }
                    let prev = line.prev().unwrap();
                    if is_in_bounds(&self.items, &prev).unwrap() {
                        self.antinodes.entry(prev).or_default().insert(freq);
                    }
                } else {
                    self.antinodes.entry(*pairs[0]).or_default().insert(freq);
                    self.antinodes.entry(*pairs[1]).or_default().insert(freq);
                    let mut curr = line;
                    while let Some(next) = curr.next() {
                        if !is_in_bounds(&self.items, &next).unwrap() {
                            break;
                        }
                        self.antinodes.entry(next).or_default().insert(freq);
                        curr.to = curr.from;
                        curr.from = next;
                    }
                    let mut curr = line;
                    while let Some(prev) = curr.prev() {
                        if !is_in_bounds(&self.items, &prev).unwrap() {
                            break;
                        }
                        self.antinodes.entry(prev).or_default().insert(freq);
                        curr.from = curr.to;
                        curr.to = prev;
                    }
                }
            })
        });
    }

    #[allow(unused)]
    fn print(&self) {
        for (i, row) in self.items.iter().enumerate() {
            for (j, &ch) in row.iter().enumerate() {
                let ch_to_print = match (
                    ch,
                    self.antinodes.get(&Point::new([i as isize, j as isize])),
                ) {
                    (ch, _) if ch != '.' => ch,
                    (_, Some(_)) => '#',
                    _ => '.',
                };
                print!("{}", ch_to_print);
            }
            println!();
        }
        println!();
    }
}

#[derive(Debug, Clone, Copy)]
struct Line {
    from: Point<2, isize>,
    to: Point<2, isize>,
}
impl Line {
    fn new(from: Point<2, isize>, to: Point<2, isize>) -> Self {
        Line { from, to }
    }

    fn prev(&self) -> Option<Point<2, isize>> {
        let diff = (
            self.from.get('x')? - self.to.get('x')?,
            self.from.get('y')? - self.to.get('y')?,
        );
        let point = Point::new([
            self.to.get('x')? - diff.0.signum() * diff.0.abs(),
            self.to.get('y')? - diff.1.signum() * diff.1.abs(),
        ]);
        Some(point)
    }

    fn next(&self) -> Option<Point<2, isize>> {
        let diff = (
            self.from.get('x')? - self.to.get('x')?,
            self.from.get('y')? - self.to.get('y')?,
        );
        let point = Point::new([
            self.from.get('x')? + diff.0.signum() * diff.0.abs(),
            self.from.get('y')? + diff.1.signum() * diff.1.abs(),
        ]);
        Some(point)
    }
}

fn is_in_bounds(map: &[Vec<char>], pos: &Point<2, isize>) -> Option<bool> {
    Some(
        pos.get('x')? >= 0
            && pos.get('x')? < map.len() as isize
            && pos.get('y')? >= 0
            && pos.get('y')? < map[0].len() as isize,
    )
}

fn read_input(filename: &str) -> Map {
    let items = utils::read_to_string_in_module!(filename)
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    Map::new(items)
}

#[cfg(test)]
mod tests {
    use super::read_input;

    #[test]
    fn p1() {
        let mut map = read_input("test-input2.txt");
        assert_eq!(map.count_unique_antinodes_positions(false), 2);
        let mut map = read_input("test-input3.txt");
        assert_eq!(map.count_unique_antinodes_positions(false), 4);
        let mut map = read_input("test-input4.txt");
        assert_eq!(map.count_unique_antinodes_positions(false), 4);
        let mut map = read_input("test-input.txt");
        assert_eq!(map.count_unique_antinodes_positions(false), 14);
    }

    #[test]
    fn p2() {
        let mut map = read_input("test-input5.txt");
        assert_eq!(map.count_unique_antinodes_positions(true), 9);
        let mut map = read_input("test-input.txt");
        assert_eq!(map.count_unique_antinodes_positions(true), 34);
    }

    #[test]
    fn prod() {
        let mut map = read_input("input.txt");
        assert_eq!(map.count_unique_antinodes_positions(false), 354);
        assert_eq!(map.count_unique_antinodes_positions(true), 1263);
    }
}
