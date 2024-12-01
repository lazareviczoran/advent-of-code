use regex::Regex;
use std::collections::HashMap;
use std::hash::Hash;

pub fn run() {
    let instructions = read_input("input.txt");
    utils::run_solution!(
        || {
            let mut computer = Computer::new(&instructions);
            computer.run();
            *computer.registers.get(&Register::B).unwrap()
        },
        "part1"
    );

    utils::run_solution!(
        || {
            let mut computer = Computer::new(&instructions);
            computer.registers.insert(Register::A, 1);
            computer.run();
            *computer.registers.get(&Register::B).unwrap()
        },
        "part2"
    );
}

struct Computer {
    registers: HashMap<Register, usize>,
    instructions: Vec<Instruction>,
    pos: i32,
}
impl Computer {
    pub fn new(instructions: &[Instruction]) -> Computer {
        let mut registers = HashMap::new();
        registers.insert(Register::A, 0);
        registers.insert(Register::B, 0);
        Computer {
            registers,
            instructions: instructions.to_owned(),
            pos: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            let curr_instruction = self.instructions[self.pos as usize];
            match curr_instruction.command {
                Command::Hlf => {
                    let value = self
                        .registers
                        .get_mut(&curr_instruction.register.unwrap())
                        .unwrap();
                    *value /= 2;
                }
                Command::Tpl => {
                    let value = self
                        .registers
                        .get_mut(&curr_instruction.register.unwrap())
                        .unwrap();
                    *value *= 3;
                }
                Command::Inc => {
                    let value = self
                        .registers
                        .get_mut(&curr_instruction.register.unwrap())
                        .unwrap();
                    *value += 1;
                }
                Command::Jmp => {
                    let offset = &curr_instruction.offset.unwrap();
                    self.pos += offset;
                    continue;
                }
                Command::Jie => {
                    let offset = &curr_instruction.offset.unwrap();
                    let value = self
                        .registers
                        .get(&curr_instruction.register.unwrap())
                        .unwrap();
                    if *value % 2 == 0 {
                        self.pos += offset;
                        continue;
                    }
                }
                Command::Jio => {
                    let offset = &curr_instruction.offset.unwrap();
                    let value = self
                        .registers
                        .get(&curr_instruction.register.unwrap())
                        .unwrap();
                    if *value == 1 {
                        self.pos += offset;
                        if self.pos < 0 || self.pos >= self.instructions.len() as i32 {
                            return;
                        }
                        continue;
                    }
                }
            }
            self.pos += 1;
            if self.pos < 0 || self.pos >= self.instructions.len() as i32 {
                return;
            }
        }
    }
}

#[derive(Clone, Copy)]
struct Instruction {
    command: Command,
    register: Option<Register>,
    offset: Option<i32>,
}
impl Instruction {
    pub fn new(command: Command, register: Option<Register>, offset: Option<i32>) -> Instruction {
        Instruction {
            command,
            register,
            offset,
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Register {
    A,
    B,
}

#[derive(Copy, Clone, PartialEq)]
enum Command {
    Hlf,
    Tpl,
    Inc,
    Jmp,
    Jie,
    Jio,
}

fn read_input(filename: &str) -> Vec<Instruction> {
    let contents = utils::read_to_string_in_module!(filename);
    let mut instructions = Vec::new();
    let re = Regex::new(r"(.+?)\s(a|b)?,?\s?\+?(-?\d+)?").unwrap();
    for s in contents.split_terminator('\n') {
        let captures = re.captures(s).unwrap();
        let command = match &captures[1] {
            "hlf" => Command::Hlf,
            "tpl" => Command::Tpl,
            "inc" => Command::Inc,
            "jmp" => Command::Jmp,
            "jie" => Command::Jie,
            "jio" => Command::Jio,
            _ => panic!("Unknown command {}", &captures[1]),
        };
        let mut offset = None;
        let mut register = None;
        if let Some(reg) = captures.get(2) {
            register = match reg.as_str() {
                "a" => Some(Register::A),
                "b" => Some(Register::B),
                _ => panic!("Unknown register {}", &captures[2]),
            }
        }
        if captures.get(3).is_some() {
            offset = Some(captures[3].parse::<i32>().unwrap());
        }
        instructions.push(Instruction::new(command, register, offset));
    }

    instructions
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let instructions = read_input("test-input.txt");
        let mut computer = Computer::new(&instructions);
        computer.run();
        assert_eq!(*computer.registers.get(&Register::A).unwrap(), 2);
    }
}
