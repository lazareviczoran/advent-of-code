use std::collections::HashMap;

pub fn run() {
    let mut comp = Computer::new(read("input.txt"));
    comp.run();
    println!("part1 solution: {}", comp.registers.get(&'a').unwrap());

    comp.reset();
    comp.registers.insert('c', 1);
    comp.run();
    println!("part2 solution: {}", comp.registers.get(&'a').unwrap());
}

struct Computer {
    registers: HashMap<char, i32>,
    instructions: Vec<Instruction>,
    curr_instr: i32,
}
impl Computer {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            registers: HashMap::new(),
            curr_instr: 0,
            instructions,
        }
    }

    fn run(&mut self) {
        while self.curr_instr < self.instructions.len() as i32 {
            let mut step = 1;
            match self.instructions[self.curr_instr as usize] {
                Instruction::CopyReg { register, to } => {
                    let val = *self.registers.get(&register).unwrap_or(&0);
                    *self.registers.entry(to).or_insert(0) = val;
                }
                Instruction::CopyVal { value, to } => {
                    *self.registers.entry(to).or_insert(0) = value;
                }
                Instruction::Inc { register } => {
                    *self.registers.entry(register).or_insert(0) += 1;
                }
                Instruction::Dec { register } => {
                    *self.registers.entry(register).or_insert(0) -= 1;
                }
                Instruction::JnzReg { register, by } => {
                    let val = *self.registers.get(&register).unwrap_or(&0);
                    if val != 0 {
                        step = by;
                    }
                }
                Instruction::JnzVal { value, by } => {
                    if value != 0 {
                        step = by;
                    }
                }
            }
            self.curr_instr += step;
            if self.curr_instr < 0 {
                panic!("index has negative value");
            }
        }
    }

    fn reset(&mut self) {
        self.curr_instr = 0;
        self.registers = HashMap::new();
    }
}

enum Instruction {
    CopyVal { value: i32, to: char },
    CopyReg { register: char, to: char },
    Inc { register: char },
    Dec { register: char },
    JnzVal { value: i32, by: i32 },
    JnzReg { register: char, by: i32 },
}

fn read(filename: &str) -> Vec<Instruction> {
    utils::read_to_string_in_module!(filename)
        .split_terminator('\n')
        .filter_map(|s| {
            let mut parts = s.split_terminator(' ');
            let cmd = parts.next()?;
            match cmd {
                "cpy" => {
                    let val = parts.next()?;
                    let to = parts.next()?.chars().next()?;
                    if val.chars().next()?.is_ascii_alphabetic() {
                        Some(Instruction::CopyReg {
                            register: val.chars().next()?,
                            to,
                        })
                    } else {
                        Some(Instruction::CopyVal {
                            value: val.parse().ok()?,
                            to,
                        })
                    }
                }
                "inc" => {
                    let register = parts.next()?.chars().next()?;
                    Some(Instruction::Inc { register })
                }
                "dec" => {
                    let register = parts.next()?.chars().next()?;
                    Some(Instruction::Dec { register })
                }
                "jnz" => {
                    let val = parts.next()?;
                    let by = parts.next()?.parse().ok()?;
                    if val.chars().next()?.is_ascii_alphabetic() {
                        Some(Instruction::JnzReg {
                            register: val.chars().next()?,
                            by,
                        })
                    } else {
                        Some(Instruction::JnzVal {
                            value: val.parse().ok()?,
                            by,
                        })
                    }
                }
                _ => unreachable!(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_tests() {
        let mut comp = Computer::new(read("test-input.txt"));
        comp.run();
        assert_eq!(comp.registers.get(&'a'), Some(&42));
    }
}
