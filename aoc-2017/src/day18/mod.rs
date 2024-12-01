use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};

use std::rc::Rc;

use either::Either;

pub fn run() {
    let instructions = read("input.txt");
    let mut comp = Computer::new(0, instructions.clone());
    comp.run();
    println!("part1 solution: {}", comp.channel.back().unwrap());
    let mut network = Network::new(vec![
        Computer::new(0, instructions.clone()),
        Computer::new(1, instructions),
    ]);
    network.run_til_all_stops();
    println!(
        "part2 solution: {}",
        network.computers[1].borrow().send_count
    );
}

struct Network {
    computers: Vec<Rc<RefCell<Computer>>>,
}
impl Network {
    fn new(computers: Vec<Computer>) -> Self {
        let computers = computers
            .into_iter()
            .map(|mut c| {
                c.registers.insert('p', c.id as i64);
                Rc::new(RefCell::new(c))
            })
            .collect();
        Self { computers }
    }

    fn run_til_all_stops(&mut self) {
        loop {
            self.computers[0]
                .borrow_mut()
                .run_single_instruction(&self.computers[1]);
            self.computers[1]
                .borrow_mut()
                .run_single_instruction(&self.computers[0]);

            if self.computers[0].borrow().pointer == i64::MAX
                && self.computers[1].borrow().pointer == i64::MAX
                || self.computers[0].borrow().pointer < i64::MAX
                    && self.computers[0].borrow().is_blocked(&self.computers[1])
                    && self.computers[1].borrow().pointer < i64::MAX
                    && self.computers[1].borrow().is_blocked(&self.computers[0])
            {
                return;
            }
        }
    }
}

struct Computer {
    id: usize,
    pointer: i64,
    registers: HashMap<char, i64>,
    instructions: Vec<Instruction>,
    channel: VecDeque<i64>,
    send_count: usize,
}
impl Computer {
    fn new(id: usize, instructions: Vec<Instruction>) -> Self {
        Self {
            id,
            pointer: 0,
            registers: HashMap::new(),
            instructions,
            channel: VecDeque::new(),
            send_count: 0,
        }
    }

    fn run(&mut self) {
        while self.pointer < self.instructions.len() as i64 {
            let curr_instr = &self.instructions[self.pointer as usize];
            match curr_instr {
                Instruction::Snd(val) => {
                    self.channel.push_back(self.get_value(val));
                    self.send_count += 1;
                }
                Instruction::Rcv(val) => {
                    let value = self.get_value(val);
                    if value > 0 {
                        return;
                    }
                }
                Instruction::Set(val1, val2) => {
                    *self.registers.entry(*val1).or_insert(0) = self.get_value(val2);
                }
                Instruction::Add(val1, val2) => {
                    *self.registers.entry(*val1).or_insert(0) += self.get_value(val2);
                }
                Instruction::Mul(val1, val2) => {
                    *self.registers.entry(*val1).or_insert(0) *= self.get_value(val2);
                }
                Instruction::Mod(val1, val2) => {
                    *self.registers.entry(*val1).or_insert(0) %= self.get_value(val2);
                }
                Instruction::Jgz(val1, val2) => {
                    let value = self.get_value(val1);
                    if value > 0 {
                        self.pointer += self.get_value(val2);
                        continue;
                    }
                }
            }
            self.pointer += 1;
        }
    }

    fn run_single_instruction(&mut self, other: &Rc<RefCell<Computer>>) {
        if self.pointer < self.instructions.len() as i64 {
            let curr_instr = &self.instructions[self.pointer as usize];
            match curr_instr {
                Instruction::Snd(val) => {
                    other.borrow_mut().channel.push_back(self.get_value(val));
                    self.send_count += 1;
                }
                Instruction::Rcv(val) => {
                    let reg = val.left().unwrap();
                    if let Some(msg) = self.channel.pop_front() {
                        *self.registers.entry(reg).or_insert(0) = msg;
                    } else {
                        return;
                    }
                }
                Instruction::Set(val1, val2) => {
                    *self.registers.entry(*val1).or_insert(0) = self.get_value(val2);
                }
                Instruction::Add(val1, val2) => {
                    *self.registers.entry(*val1).or_insert(0) += self.get_value(val2);
                }
                Instruction::Mul(val1, val2) => {
                    *self.registers.entry(*val1).or_insert(0) *= self.get_value(val2);
                }
                Instruction::Mod(val1, val2) => {
                    *self.registers.entry(*val1).or_insert(0) %= self.get_value(val2);
                }
                Instruction::Jgz(val1, val2) => {
                    let value = self.get_value(val1);
                    if value > 0 {
                        self.pointer += self.get_value(val2);
                        return;
                    }
                }
            }
            self.pointer += 1;
        } else {
            self.pointer = i64::MAX;
        }
    }

    fn is_blocked(&self, other: &Rc<RefCell<Computer>>) -> bool {
        match self.instructions[self.pointer as usize] {
            Instruction::Rcv(_) => other.borrow().channel.is_empty(),
            _ => false,
        }
    }

    fn get_value(&self, value: &Either<char, i64>) -> i64 {
        match value {
            Either::Left(reg) => *self.registers.get(reg).unwrap_or(&0),
            Either::Right(val) => *val,
        }
    }
}

#[derive(Clone)]
enum Instruction {
    Snd(Either<char, i64>),
    Rcv(Either<char, i64>),
    Set(char, Either<char, i64>),
    Add(char, Either<char, i64>),
    Mul(char, Either<char, i64>),
    Mod(char, Either<char, i64>),
    Jgz(Either<char, i64>, Either<char, i64>),
}

fn read(filename: &str) -> Vec<Instruction> {
    utils::read_to_string_in_module!(filename)
        .lines()
        .map(|s| {
            let parts = s.split_terminator(' ').collect::<Vec<_>>();
            match parts[0] {
                "snd" => Instruction::Snd(parse_arg(parts[1])),
                "rcv" => Instruction::Rcv(parse_arg(parts[1])),
                "set" => Instruction::Set(parts[1].chars().next().unwrap(), parse_arg(parts[2])),
                "add" => Instruction::Add(parts[1].chars().next().unwrap(), parse_arg(parts[2])),
                "mul" => Instruction::Mul(parts[1].chars().next().unwrap(), parse_arg(parts[2])),
                "mod" => Instruction::Mod(parts[1].chars().next().unwrap(), parse_arg(parts[2])),
                "jgz" => Instruction::Jgz(parse_arg(parts[1]), parse_arg(parts[2])),
                _ => unimplemented!("operator {}", parts[1]),
            }
        })
        .collect()
}

fn parse_arg(arg: &str) -> Either<char, i64> {
    if arg.chars().peekable().peek().unwrap().is_alphabetic() {
        Either::Left(arg.chars().next().unwrap())
    } else {
        Either::Right(arg.parse().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let mut comp = Computer::new(0, read("test-input.txt"));
        comp.run();
        assert_eq!(comp.channel.back(), Some(&4));
    }
}
