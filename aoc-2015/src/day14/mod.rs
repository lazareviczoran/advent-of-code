use regex::Regex;
use std::collections::HashMap;

pub fn run() {
    let reindeers = read_input("input.txt");
    let mut race = Race::new(reindeers);

    utils::run_solution!(|| { race.start_race(2503).distance }, "part1");

    race.reset();

    utils::run_solution!(|| { race.start_point_race(2503).points }, "part2");
}

fn read_input(filename: &str) -> HashMap<String, Reindeer> {
    let mut reindeers = HashMap::new();
    let contents = utils::read_to_string_in_module!(filename);
    let re = Regex::new(r"(.+?)\s.*?\s(\d+)\skm/s\sfor\s(\d+)\s.+\s(\d+)").unwrap();
    for string in contents.split_terminator('\n') {
        let cap = re.captures(string).unwrap();
        let name = String::from(&cap[1]);
        reindeers.insert(
            name.clone(),
            Reindeer::new(
                name,
                cap[2].parse::<usize>().unwrap(),
                cap[3].parse::<usize>().unwrap(),
                cap[4].parse::<usize>().unwrap(),
            ),
        );
    }

    reindeers
}

struct Race {
    reindeers: HashMap<String, Reindeer>,
}
impl Race {
    pub fn new(reindeers: HashMap<String, Reindeer>) -> Race {
        Race { reindeers }
    }

    pub fn start_point_race(&mut self, target_time: usize) -> Reindeer {
        let mut current_max_dist = 0;
        for _ in 1..=target_time {
            for (_, r) in self.reindeers.iter_mut() {
                if r.remaining_duration != 0 {
                    r.distance += r.speed;
                    r.remaining_duration -= 1;
                } else {
                    r.remaining_recovery -= 1;
                    if r.remaining_recovery == 0 {
                        r.remaining_recovery = r.recovery;
                        r.remaining_duration = r.duration;
                    }
                }
                if current_max_dist < r.distance {
                    current_max_dist = r.distance;
                }
            }
            for (_, r) in self.reindeers.iter_mut() {
                if r.distance == current_max_dist {
                    r.points += 1;
                }
            }
        }

        let mut winner: Option<Reindeer> = None;
        for (_, r) in self.reindeers.iter() {
            if winner.is_none() || r.points > winner.clone().unwrap().points {
                winner = Some(r.clone());
            }
        }
        winner.unwrap()
    }

    pub fn start_race(&mut self, target_time: usize) -> Reindeer {
        for _ in 1..=target_time {
            for (_, r) in self.reindeers.iter_mut() {
                if r.remaining_duration != 0 {
                    r.distance += r.speed;
                    r.remaining_duration -= 1;
                } else {
                    r.remaining_recovery -= 1;
                    if r.remaining_recovery == 0 {
                        r.remaining_recovery = r.recovery;
                        r.remaining_duration = r.duration;
                    }
                }
            }
        }

        let mut winner: Option<Reindeer> = None;
        for (_, r) in self.reindeers.iter() {
            if winner.is_none() || r.distance > winner.clone().unwrap().distance {
                winner = Some(r.clone());
            }
        }
        winner.unwrap()
    }

    pub fn reset(&mut self) {
        for (_, r) in self.reindeers.iter_mut() {
            r.distance = 0;
            r.points = 0;
            r.remaining_duration = r.duration;
            r.remaining_recovery = r.recovery;
        }
    }
}

#[derive(Clone, Debug)]
struct Reindeer {
    _name: String,
    points: usize,
    distance: usize,
    speed: usize,
    duration: usize,
    recovery: usize,
    remaining_duration: usize,
    remaining_recovery: usize,
}
impl Reindeer {
    pub fn new(name: String, speed: usize, duration: usize, recovery: usize) -> Reindeer {
        Reindeer {
            _name: name,
            points: 0,
            distance: 0,
            speed,
            duration,
            remaining_duration: duration,
            recovery,
            remaining_recovery: recovery,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_input1() {
        let reindeers = read_input("test-input.txt");
        let mut race = Race::new(reindeers);
        let winner = race.start_race(1000);

        assert_eq!(winner.distance, 1120);
    }

    #[test]
    fn part2_input1() {
        let reindeers = read_input("test-input.txt");
        let mut race = Race::new(reindeers);
        let winner = race.start_point_race(1000);

        assert_eq!(winner.points, 689);
    }
}
