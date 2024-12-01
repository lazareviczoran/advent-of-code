use std::collections::HashMap;

pub fn run() {
    let mut comp = Computer::new(read("input.txt"));
    comp.run();
    println!("part1 solution: {}", comp.registers.values().max().unwrap());
    println!("part2 solution: {}", comp.highest_reg_value);
}

struct Computer {
    pointer: i64,
    registers: HashMap<String, i64>,
    instructions: Vec<Instruction>,
    highest_reg_value: i64,
}
impl Computer {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            pointer: 0,
            registers: HashMap::new(),
            instructions,
            highest_reg_value: 0,
        }
    }

    fn run(&mut self) {
        while self.pointer < self.instructions.len() as i64 {
            let curr_instr = &self.instructions[self.pointer as usize];
            if self.evaluate_condition(&curr_instr.condition) {
                let register = self.registers.entry(curr_instr.reg.clone()).or_insert(0);
                match curr_instr.op {
                    Op::Inc => *register += curr_instr.value,
                    Op::Dec => *register -= curr_instr.value,
                }
                if *register > self.highest_reg_value {
                    self.highest_reg_value = *register;
                }
            }
            self.pointer += 1;
        }
    }

    fn evaluate_condition(&self, cond: &Condition) -> bool {
        let register_value = *self.registers.get(&cond.reg).unwrap_or(&0);
        match cond.compare_op {
            CmpOp::Eq => register_value == cond.value,
            CmpOp::Neq => register_value != cond.value,
            CmpOp::Gt => register_value > cond.value,
            CmpOp::Gte => register_value >= cond.value,
            CmpOp::Lte => register_value <= cond.value,
            CmpOp::Lt => register_value < cond.value,
        }
    }
}

struct Instruction {
    reg: String,
    op: Op,
    value: i64,
    condition: Condition,
}

struct Condition {
    reg: String,
    value: i64,
    compare_op: CmpOp,
}

enum Op {
    Inc,
    Dec,
}

enum CmpOp {
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
    Eq,
}

fn read(filename: &str) -> Vec<Instruction> {
    utils::read_to_string_in_module!(filename)
        .lines()
        .filter_map(|s| {
            let parts = s.split_terminator(' ').collect::<Vec<_>>();
            let reg = parts[0].to_string();
            let op = match parts[1] {
                "inc" => Op::Inc,
                "dec" => Op::Dec,
                _ => unimplemented!("operator {}", parts[1]),
            };
            let value = parts[2].parse().ok()?;
            let condition = Condition {
                reg: parts[4].to_string(),
                compare_op: match parts[5] {
                    "==" => CmpOp::Eq,
                    "!=" => CmpOp::Neq,
                    ">" => CmpOp::Gt,
                    ">=" => CmpOp::Gte,
                    "<=" => CmpOp::Lte,
                    "<" => CmpOp::Lt,
                    _ => unimplemented!("compare operator {}", parts[5]),
                },
                value: parts[6].parse().ok()?,
            };
            Some(Instruction {
                reg,
                op,
                value,
                condition,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut comp = Computer::new(read("test-input.txt"));
        comp.run();

        assert_eq!(comp.registers.values().max(), Some(&1));
        assert_eq!(comp.highest_reg_value, 10);
    }
}
