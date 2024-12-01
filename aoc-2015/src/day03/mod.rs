use std::collections::HashMap;

pub fn run() {
    let contents = utils::read_to_string_in_module!("input.txt");

    let mut env = Environment::new(1);
    utils::run_solution!(
        || get_total_houses_with_presents_count(&mut env, &contents),
        "part1"
    );

    env = Environment::new(2);
    utils::run_solution!(
        || get_total_houses_with_presents_count(&mut env, &contents),
        "part2"
    );
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Position {
    x: i32,
    y: i32,
}
impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
}

struct Environment {
    workers: Vec<Worker>,
    map: HashMap<Position, usize>,
}
impl Environment {
    pub fn new(num_of_workers: usize) -> Environment {
        let mut workers = Vec::new();
        for _ in 0..num_of_workers {
            workers.push(Worker::new());
        }
        let mut map = HashMap::new();
        map.insert(Position::new(0, 0), num_of_workers);
        Environment { workers, map }
    }

    pub fn run(&mut self, moves: &Vec<Moves>) -> usize {
        let mut current_worker = 0;
        let num_of_workers = self.workers.len();
        for m in moves {
            self.workers[current_worker].move_to(&mut self.map, m);
            if num_of_workers > 1 {
                current_worker += 1;
                if current_worker == num_of_workers {
                    current_worker = 0;
                }
            }
        }
        self.map.len()
    }
}

struct Worker {
    curr_pos: Position,
}
impl Worker {
    pub fn new() -> Worker {
        Worker {
            curr_pos: Position::new(0, 0),
        }
    }
    pub fn move_to(&mut self, map: &mut HashMap<Position, usize>, dir: &Moves) {
        let mut next_pos = self.curr_pos.clone();
        match dir {
            Moves::UP => next_pos.y += 1,
            Moves::DOWN => next_pos.y -= 1,
            Moves::RIGHT => next_pos.x += 1,
            Moves::LEFT => next_pos.x -= 1,
        }
        self.curr_pos = next_pos.clone();
        if let Some(val) = map.get_mut(&next_pos) {
            *val += 1;
        } else {
            map.insert(next_pos, 1);
        }
    }
}

enum Moves {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn read_moves(input: &String) -> Vec<Moves> {
    let chars = input.chars();
    let mut res = Vec::new();
    for ch in chars {
        match ch {
            '^' => res.push(Moves::UP),
            'v' => res.push(Moves::DOWN),
            '>' => res.push(Moves::RIGHT),
            '<' => res.push(Moves::LEFT),
            _ => panic!("Unexpected char {}", ch),
        }
    }
    res
}

fn get_total_houses_with_presents_count(env: &mut Environment, input: &String) -> usize {
    let moves = read_moves(&input);
    env.run(&moves)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_input1() {
        let mut env = Environment::new(1);
        assert_eq!(
            get_total_houses_with_presents_count(&mut env, &String::from(">")),
            2
        );
    }

    #[test]
    fn part1_input2() {
        let mut env = Environment::new(1);
        assert_eq!(
            get_total_houses_with_presents_count(&mut env, &String::from("^>v<")),
            4
        );
    }

    #[test]
    fn part1_input3() {
        let mut env = Environment::new(1);
        assert_eq!(
            get_total_houses_with_presents_count(&mut env, &String::from("^v^v^v^v^v")),
            2
        );
    }

    #[test]
    fn part2_input1() {
        let mut env = Environment::new(2);
        assert_eq!(
            get_total_houses_with_presents_count(&mut env, &String::from("^v")),
            3
        );
    }

    #[test]
    fn part2_input2() {
        let mut env = Environment::new(2);
        assert_eq!(
            get_total_houses_with_presents_count(&mut env, &String::from("^>v<")),
            3
        );
    }

    #[test]
    fn part2_input3() {
        let mut env = Environment::new(2);
        assert_eq!(
            get_total_houses_with_presents_count(&mut env, &String::from("^v^v^v^v^v")),
            11
        );
    }
}
