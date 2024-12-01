use std::collections::HashMap;

pub fn run() {
    let rules = read("input.txt");
    let mut grid = Grid::init();
    grid.transform_n_times(&rules, 5);
    println!("part1 solution: {:?}", grid.count_active());

    let mut grid = Grid::init();
    grid.transform_n_times(&rules, 18);
    println!("part2 solution: {:?}", grid.count_active());
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Grid {
    items: Vec<Vec<char>>,
}

impl Grid {
    fn init() -> Self {
        Self {
            items: vec![
                vec!['.', '#', '.'],
                vec!['.', '.', '#'],
                vec!['#', '#', '#'],
            ],
        }
    }

    fn new(items: Vec<Vec<char>>) -> Self {
        Self { items }
    }

    fn transform_n_times(&mut self, rules: &HashMap<Grid, Grid>, n: usize) {
        for _ in 0..n {
            self.transform(rules);
        }
    }

    fn transform(&mut self, rules: &HashMap<Grid, Grid>) {
        let mut small_grids = self.split();
        for sm in &mut small_grids {
            for _ in 0..4 {
                if let Some(new_grid) = rules.get(sm) {
                    *sm = new_grid.clone();
                    break;
                }
                sm.flip();
                if let Some(new_grid) = rules.get(sm) {
                    *sm = new_grid.clone();
                    break;
                }
                sm.flip();
                sm.rotate();
            }
        }
        self.merge(small_grids);
    }

    fn split(&self) -> Vec<Grid> {
        let grid_size = match self.items.len() {
            n if n % 2 == 0 => 2,
            n if n % 3 == 0 => 3,
            _ => unreachable!(),
        };
        self.items
            .chunks(grid_size)
            .flat_map(|chunk| {
                let mut small_grids = vec![];
                for i in 0..self.items.len() / grid_size {
                    small_grids.push(Grid::new(
                        (0..grid_size)
                            .map(|y| {
                                (0..grid_size)
                                    .map(|x| chunk[y][i * grid_size + x])
                                    .collect()
                            })
                            .collect(),
                    ));
                }
                small_grids
            })
            .collect()
    }

    fn merge(&mut self, mut small_grids: Vec<Grid>) {
        if small_grids.len() == 1 {
            *self = small_grids.pop().unwrap();
        } else {
            let number_of_grids = small_grids.len();
            let grid_size = small_grids[0].items.len();
            let size = (number_of_grids as f64).sqrt() as usize;
            let mut res: Vec<Vec<char>> = vec![vec![]; size * grid_size];
            for (i, row) in res.iter_mut().enumerate() {
                for j in 0..size {
                    row.extend(&small_grids[(i / grid_size) * size + j].items[i % grid_size]);
                }
            }
            self.items = res;
        }
    }

    fn count_active(&self) -> usize {
        self.items.iter().flatten().filter(|&&ch| ch == '#').count()
    }

    fn rotate(&mut self) {
        let n = self.items.len();
        for i in 0..n / 2 {
            for j in i..n - 1 - i {
                swap(&mut self.items, (i, j), (j, n - 1 - i));
                swap(&mut self.items, (i, j), (n - 1 - i, n - 1 - j));
                swap(&mut self.items, (i, j), (n - 1 - j, i));
            }
        }
    }

    fn flip(&mut self) {
        let n = self.items[0].len();
        self.items.iter_mut().for_each(|row| {
            for i in 0..n / 2 {
                row.swap(i, n - 1 - i);
            }
        });
    }
}

fn swap(content: &mut [Vec<char>], pos1: (usize, usize), pos2: (usize, usize)) {
    let temp = content[pos1.0][pos1.1];
    content[pos1.0][pos1.1] = content[pos2.0][pos2.1];
    content[pos2.0][pos2.1] = temp;
}

fn read(filename: &str) -> HashMap<Grid, Grid> {
    utils::read_to_string_in_module!(filename)
        .lines()
        .map(|s| {
            let parts = s.split_terminator(" => ").collect::<Vec<_>>();
            let from = Grid::new(
                parts[0]
                    .split_terminator('/')
                    .map(|row| row.chars().collect())
                    .collect(),
            );
            let to = Grid::new(
                parts[1]
                    .split_terminator('/')
                    .map(|row| row.chars().collect())
                    .collect(),
            );
            (from, to)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let rules = read("test-input.txt");
        let mut grid = Grid::init();

        grid.transform_n_times(&rules, 2);
        assert_eq!(grid.count_active(), 12);
    }
}
