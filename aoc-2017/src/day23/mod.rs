use std::collections::BTreeMap;

use either::Either;

pub fn run() {
    let instructions = read("input.txt");
    let mut comp = Computer::new(instructions.clone());
    comp.run();
    println!("part1 solution: {}", comp.mul_count);

    let mut comp = Computer::new(instructions);
    comp.registers.insert('a', 1);
    comp.run_optimized();
    println!("part2 solution: {}", comp.registers.get(&'h').unwrap());
}

struct Computer {
    pointer: i64,
    registers: BTreeMap<char, i64>,
    instructions: Vec<Instruction>,
    mul_count: usize,
}
impl Computer {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            pointer: 0,
            registers: BTreeMap::new(),
            instructions,
            mul_count: 0,
        }
    }

    fn run(&mut self) {
        while self.pointer < self.instructions.len() as i64 {
            self.run_single_instruction();
        }
    }

    fn run_optimized(&mut self) {
        while self.pointer < self.instructions.len() as i64 {
            self.run_single_instruction_optimized();
        }
    }

    fn run_single_instruction(&mut self) {
        if self.pointer < self.instructions.len() as i64 {
            let curr_instr = &self.instructions[self.pointer as usize];
            match curr_instr {
                Instruction::Set(val1, val2) => {
                    *self.registers.entry(*val1).or_insert(0) = self.get_value(val2);
                }
                Instruction::Add(val1, val2) => {
                    *self.registers.entry(*val1).or_insert(0) += self.get_value(val2);
                }
                Instruction::Sub(val1, val2) => {
                    *self.registers.entry(*val1).or_insert(0) -= self.get_value(val2);
                }
                Instruction::Mul(val1, val2) => {
                    self.mul_count += 1;
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
                Instruction::Jnz(val1, val2) => {
                    let value = self.get_value(val1);
                    if value != 0 {
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

    fn run_single_instruction_optimized(&mut self) {
        if self.pointer < self.instructions.len() as i64 {
            let curr_instr = &self.instructions[self.pointer as usize];
            match curr_instr {
                Instruction::Set(val1, val2) => {
                    *self.registers.entry(*val1).or_insert(0) = self.get_value(val2);
                }
                Instruction::Add(val1, val2) => {
                    *self.registers.entry(*val1).or_insert(0) += self.get_value(val2);
                }
                Instruction::Sub(val1, val2) => {
                    *self.registers.entry(*val1).or_insert(0) -= self.get_value(val2);
                }
                Instruction::Mul(val1, val2) => {
                    self.mul_count += 1;
                    if self.pointer + 3 < self.instructions.len() as i64 {
                        match (
                            &self.instructions[self.pointer as usize + 1],
                            &self.instructions[self.pointer as usize + 2],
                            &self.instructions[self.pointer as usize + 3],
                        ) {
                            (
                                Instruction::Sub(reg1, target),
                                Instruction::Jnz(reg2, _),
                                Instruction::Set(reg3, _),
                            ) if reg1 == val1 && reg2.left() == Some(*val1) => {
                                let target_val = self.get_value(target);
                                if !is_prime(target_val) {
                                    *self.registers.entry(*reg3).or_insert(0) = 0;
                                }
                                self.pointer += 12;
                                return;
                            }
                            _ => {}
                        }
                    }
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
                Instruction::Jnz(val1, val2) => {
                    let value = self.get_value(val1);
                    if value != 0 {
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

    fn get_value(&self, value: &Either<char, i64>) -> i64 {
        match value {
            Either::Left(reg) => *self.registers.get(reg).unwrap_or(&0),
            Either::Right(val) => *val,
        }
    }
}

fn is_prime(num: i64) -> bool {
    (2..(num as f64).sqrt() as i64).all(|i| num % i != 0)
}

#[derive(Debug, Clone)]
enum Instruction {
    Set(char, Either<char, i64>),
    Add(char, Either<char, i64>),
    Sub(char, Either<char, i64>),
    Mul(char, Either<char, i64>),
    Mod(char, Either<char, i64>),
    Jgz(Either<char, i64>, Either<char, i64>),
    Jnz(Either<char, i64>, Either<char, i64>),
}

fn read(filename: &str) -> Vec<Instruction> {
    utils::read_to_string_in_module!(filename)
        .lines()
        .map(|s| {
            let parts = s.split_terminator(' ').collect::<Vec<_>>();
            match parts[0] {
                "set" => Instruction::Set(parts[1].chars().next().unwrap(), parse_arg(parts[2])),
                "add" => Instruction::Add(parts[1].chars().next().unwrap(), parse_arg(parts[2])),
                "sub" => Instruction::Sub(parts[1].chars().next().unwrap(), parse_arg(parts[2])),
                "mul" => Instruction::Mul(parts[1].chars().next().unwrap(), parse_arg(parts[2])),
                "mod" => Instruction::Mod(parts[1].chars().next().unwrap(), parse_arg(parts[2])),
                "jgz" => Instruction::Jgz(parse_arg(parts[1]), parse_arg(parts[2])),
                "jnz" => Instruction::Jnz(parse_arg(parts[1]), parse_arg(parts[2])),
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
