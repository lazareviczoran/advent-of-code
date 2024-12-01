use std::collections::HashMap;

pub fn run() {
    let mut comp = Computer::new(read("input.txt"));
    let mut i = 0;
    loop {
        comp.reset();
        comp.registers.insert('a', i);
        comp.run();

        if comp.signal.len() == 100 {
            println!("part1 solution: {}", i);
            return;
        }
        i += 1;
    }
}

#[derive(Debug)]
struct Computer {
    registers: HashMap<char, i32>,
    instructions: Vec<Instruction>,
    original_instructions: Vec<Instruction>,
    curr_instr: i32,
    signal: Vec<i32>,
}
impl Computer {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            registers: HashMap::new(),
            curr_instr: 0,
            original_instructions: instructions.clone(),
            instructions,
            signal: Vec::new(),
        }
    }

    fn run(&mut self) {
        while self.curr_instr < self.instructions.len() as i32 {
            let mut step = 1;
            match self.instructions[self.curr_instr as usize] {
                Instruction::CopyReg(register, to) => {
                    let val = *self.registers.get(&register).unwrap_or(&0);
                    *self.registers.entry(to).or_insert(0) = val;
                }
                Instruction::CopyVal(value, to) => {
                    *self.registers.entry(to).or_insert(0) = value;
                }
                Instruction::Inc(register) => {
                    if self.curr_instr + 4 < self.instructions.len() as i32 {
                        match (
                            self.instructions[self.curr_instr as usize + 1],
                            self.instructions[self.curr_instr as usize + 2],
                            self.instructions[self.curr_instr as usize + 3],
                            self.instructions[self.curr_instr as usize + 4],
                        ) {
                            (
                                Instruction::Dec(m1),
                                Instruction::JnzReg(_a1, _b1),
                                Instruction::Dec(m2),
                                Instruction::JnzReg(_a2, _b2),
                            ) => {
                                let value1 = *self.registers.entry(m1).or_insert(0);
                                let value2 = *self.registers.entry(m2).or_insert(0);
                                *self.registers.entry(register).or_insert(0) += value1 * value2;
                                *self.registers.entry(m1).or_insert(0) = 1;
                                *self.registers.entry(m2).or_insert(0) = 1;
                            }
                            _ => *self.registers.entry(register).or_insert(0) += 1,
                        }
                    } else {
                        *self.registers.entry(register).or_insert(0) += 1;
                    }
                }
                Instruction::Dec(register) => {
                    *self.registers.entry(register).or_insert(0) -= 1;
                }
                Instruction::JnzReg(register, by) => {
                    let val = *self.registers.get(&register).unwrap_or(&0);
                    if val != 0 {
                        step = by;
                    }
                }
                Instruction::JnzVal(value, by) => {
                    if value != 0 {
                        step = by;
                    }
                }
                Instruction::JnzRegToReg(register, target) => {
                    let val = *self.registers.get(&register).unwrap_or(&0);
                    if val != 0 {
                        step = *self.registers.get(&target).unwrap();
                    }
                }
                Instruction::JnzValToReg(value, target) => {
                    if value != 0 {
                        step = *self.registers.get(&target).unwrap();
                    }
                }
                Instruction::Toggle(register) => {
                    if let Some(val) = self.registers.get(&register) {
                        let target = self.curr_instr + *val;
                        if target >= 0 && target < self.instructions.len() as i32 {
                            let target_idx = target as usize;
                            let target_instr = self.instructions[target_idx];
                            match target_instr {
                                Instruction::Inc(a) => {
                                    self.instructions[target_idx] = Instruction::Dec(a);
                                }
                                Instruction::Dec(a) | Instruction::Toggle(a) => {
                                    self.instructions[target_idx] = Instruction::Inc(a);
                                }
                                Instruction::CopyReg(a, b) => {
                                    self.instructions[target_idx] = Instruction::JnzRegToReg(a, b)
                                }
                                Instruction::CopyVal(a, b) => {
                                    self.instructions[target_idx] = Instruction::JnzValToReg(a, b)
                                }
                                Instruction::JnzRegToReg(a, b) => {
                                    self.instructions[target_idx] = Instruction::CopyReg(a, b)
                                }
                                Instruction::JnzValToReg(a, b) => {
                                    self.instructions[target_idx] = Instruction::CopyVal(a, b)
                                }
                                Instruction::JnzReg(_, _)
                                | Instruction::JnzVal(_, _)
                                | Instruction::TransmitReg(_)
                                | Instruction::TransmitVal(_) => {}
                            }
                        }
                    }
                }
                Instruction::TransmitReg(register) => {
                    let val = *self.registers.get(&register).unwrap();
                    if (self.signal.is_empty() || val != 1) && val != 0
                        || !self.signal.is_empty() && self.signal[self.signal.len() - 1] == val
                    {
                        return;
                    }
                    self.signal.push(*self.registers.get(&register).unwrap());
                }
                Instruction::TransmitVal(val) => {
                    if (self.signal.is_empty() || val != 1) && val != 0
                        || !self.signal.is_empty() && self.signal[self.signal.len() - 1] == val
                    {
                        return;
                    }
                    self.signal.push(val);
                }
            }
            if self.signal.len() == 100 {
                return;
            }
            self.curr_instr += step;
            if self.curr_instr < 0 {
                panic!("index has negative value");
            }
        }
    }

    fn reset(&mut self) {
        self.curr_instr = 0;
        self.registers.clear();
        self.instructions = self.original_instructions.clone();
        self.signal.clear()
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    CopyVal(i32, char),
    CopyReg(char, char),
    Inc(char),
    Dec(char),
    JnzVal(i32, i32),
    JnzReg(char, i32),
    JnzValToReg(i32, char),
    JnzRegToReg(char, char),
    Toggle(char),
    TransmitReg(char),
    TransmitVal(i32),
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
                        Some(Instruction::CopyReg(val.chars().next()?, to))
                    } else {
                        Some(Instruction::CopyVal(val.parse().ok()?, to))
                    }
                }
                "inc" => Some(Instruction::Inc(parts.next()?.chars().next()?)),
                "dec" => Some(Instruction::Dec(parts.next()?.chars().next()?)),
                "jnz" => {
                    let val = parts.next()?;
                    let by = parts.next()?;
                    if by.chars().next()?.is_ascii_alphabetic() {
                        if val.chars().next()?.is_ascii_alphabetic() {
                            Some(Instruction::JnzRegToReg(
                                val.chars().next()?,
                                by.chars().next()?,
                            ))
                        } else {
                            Some(Instruction::JnzValToReg(
                                val.parse().ok()?,
                                by.chars().next()?,
                            ))
                        }
                    } else if val.chars().next()?.is_ascii_alphabetic() {
                        Some(Instruction::JnzReg(val.chars().next()?, by.parse().ok()?))
                    } else {
                        Some(Instruction::JnzVal(val.parse().ok()?, by.parse().ok()?))
                    }
                }
                "tgl" => {
                    let register = parts.next()?.chars().next()?;
                    Some(Instruction::Toggle(register))
                }
                "out" => {
                    let val = parts.next()?;
                    if val.chars().next()?.is_ascii_alphabetic() {
                        Some(Instruction::TransmitReg(val.chars().next()?))
                    } else {
                        Some(Instruction::TransmitVal(val.parse().ok()?))
                    }
                }
                _ => unreachable!(),
            }
        })
        .collect()
}
