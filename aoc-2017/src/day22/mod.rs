use std::collections::HashMap;

pub fn run() {
    let map = read("input.txt");
    let mut virus = Virus::new(&map);
    virus.run(10000);
    println!("part1 solution: {:?}", virus.infection_bursts_count);

    let mut virus = Virus::new(&map);
    virus.run_evolved(10000000);
    println!("part2 solution: {:?}", virus.infection_bursts_count);
}

struct Virus {
    position: (i64, i64),
    direction: (i64, i64),
    map: HashMap<(i64, i64), char>,
    infection_bursts_count: usize,
}
impl Virus {
    fn new(map: &[Vec<char>]) -> Self {
        Self {
            direction: (0, -1),
            position: (map[0].len() as i64 / 2, map.len() as i64 / 2),
            map: map
                .iter()
                .enumerate()
                .flat_map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(x, &ch)| ((x as i64, y as i64), ch))
                        .collect::<Vec<_>>()
                })
                .collect(),
            infection_bursts_count: 0,
        }
    }

    fn run(&mut self, n: usize) {
        for _ in 0..n {
            self.move_basic();
        }
    }

    fn run_evolved(&mut self, n: usize) {
        for _ in 0..n {
            self.move_evolved();
        }
    }

    fn move_basic(&mut self) {
        let curr_value = self.map.entry(self.position).or_insert('.');
        if curr_value == &'#' {
            self.direction = match self.direction {
                (1, 0) => (0, 1),
                (-1, 0) => (0, -1),
                (0, 1) => (-1, 0),
                (0, -1) => (1, 0),
                _ => unreachable!(),
            };
            *curr_value = '.';
        } else {
            self.direction = match self.direction {
                (1, 0) => (0, -1),
                (-1, 0) => (0, 1),
                (0, 1) => (1, 0),
                (0, -1) => (-1, 0),
                _ => unreachable!(),
            };
            self.infection_bursts_count += 1;
            *curr_value = '#';
        }
        self.position.0 += self.direction.0;
        self.position.1 += self.direction.1;
    }

    fn move_evolved(&mut self) {
        let curr_value = self.map.entry(self.position).or_insert('.');
        match curr_value {
            '.' => {
                self.direction = match self.direction {
                    (1, 0) => (0, -1),
                    (-1, 0) => (0, 1),
                    (0, 1) => (1, 0),
                    (0, -1) => (-1, 0),
                    _ => unreachable!(),
                };
                *curr_value = 'W';
            }
            'W' => {
                self.infection_bursts_count += 1;
                *curr_value = '#';
            }
            '#' => {
                self.direction = match self.direction {
                    (1, 0) => (0, 1),
                    (-1, 0) => (0, -1),
                    (0, 1) => (-1, 0),
                    (0, -1) => (1, 0),
                    _ => unreachable!(),
                };
                *curr_value = 'F';
            }
            'F' => {
                self.direction.0 *= -1;
                self.direction.1 *= -1;
                *curr_value = '.';
            }
            _ => unreachable!(),
        }
        self.position.0 += self.direction.0;
        self.position.1 += self.direction.1;
    }
}

fn read(filename: &str) -> Vec<Vec<char>> {
    utils::read_to_string_in_module!(filename)
        .lines()
        .map(|s| s.chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let map = read("test-input.txt");
        let mut virus = Virus::new(&map);
        virus.run(70);

        assert_eq!(virus.infection_bursts_count, 41);
        let mut virus = Virus::new(&map);
        virus.run(10000);
        assert_eq!(virus.infection_bursts_count, 5587);

        let mut virus = Virus::new(&map);
        virus.run_evolved(100);
        assert_eq!(virus.infection_bursts_count, 26);

        let mut virus = Virus::new(&map);
        virus.run_evolved(10000000);
        assert_eq!(virus.infection_bursts_count, 2511944);
    }
}
