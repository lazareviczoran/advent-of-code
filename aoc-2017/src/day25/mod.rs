use std::collections::HashMap;

pub fn run() {
    let mut machine = Machine::new();
    machine.run_for_n_steps(12172063);
    println!("part1 solution: {}", machine.diagnostic_checksum());
}

struct Machine {
    cursor: i64,
    state: State,
    tape: HashMap<i64, u8>,
    step: usize,
}
impl Machine {
    fn new() -> Self {
        Self {
            cursor: 0,
            state: State::A,
            tape: HashMap::new(),
            step: 0,
        }
    }

    fn run_for_n_steps(&mut self, steps: usize) {
        while self.step < steps {
            self.change_state();
        }
    }

    fn diagnostic_checksum(&self) -> usize {
        self.tape.values().filter(|&&v| v == 1).count()
    }

    fn change_state(&mut self) {
        self.step += 1;
        let value = self.tape.entry(self.cursor).or_insert(0);
        match self.state {
            State::A => {
                if *value == 0 {
                    *value = 1;
                    self.cursor += 1;
                    self.state = State::B;
                } else {
                    *value = 0;
                    self.cursor -= 1;
                    self.state = State::C;
                }
            }
            State::B => {
                if *value == 0 {
                    *value = 1;
                    self.cursor -= 1;
                    self.state = State::A;
                } else {
                    self.cursor -= 1;
                    self.state = State::D;
                }
            }
            State::C => {
                if *value == 0 {
                    *value = 1;
                    self.cursor += 1;
                    self.state = State::D;
                } else {
                    *value = 0;
                    self.cursor += 1;
                }
            }
            State::D => {
                if *value == 0 {
                    self.cursor -= 1;
                    self.state = State::B;
                } else {
                    *value = 0;
                    self.cursor += 1;
                    self.state = State::E;
                }
            }
            State::E => {
                if *value == 0 {
                    *value = 1;
                    self.cursor += 1;
                    self.state = State::C;
                } else {
                    self.cursor -= 1;
                    self.state = State::F;
                }
            }
            State::F => {
                if *value == 0 {
                    *value = 1;
                    self.cursor -= 1;
                    self.state = State::E;
                } else {
                    self.cursor += 1;
                    self.state = State::A;
                }
            }
        }
    }
}

enum State {
    A,
    B,
    C,
    D,
    E,
    F,
}
