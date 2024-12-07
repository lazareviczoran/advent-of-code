use std::collections::HashSet;

pub fn run() {
    let equations = read_input("input.txt");

    utils::run_solution!(|| compute_sum(&equations, Op::candidates(true)), "part1");
    utils::run_solution!(|| compute_sum(&equations, Op::candidates(false)), "part2");
}

fn compute_sum(equations: &[CalibrationEquation], candidates: &[Op]) -> u128 {
    equations
        .iter()
        .filter(|eq| eq.is_valid(candidates))
        .map(|eq| eq.target)
        .sum()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CalibrationEquation {
    target: u128,
    values: Vec<u128>,
}
impl CalibrationEquation {
    fn is_valid(&self, candidates: &[Op]) -> bool {
        let mut successful_ops = HashSet::new();
        self.find_valid_ops_rec(&mut successful_ops, candidates);
        !successful_ops.is_empty()
    }

    fn find_valid_ops_rec(&self, ops_acc: &mut HashSet<(Self, Vec<Op>)>, candidates: &[Op]) {
        candidates.iter().for_each(|&op| {
            self.find_valid_ops(
                op,
                self.values[0],
                &self.values[1..],
                &Vec::new(),
                ops_acc,
                candidates,
            )
        });
    }

    fn find_valid_ops(
        &self,
        curr_op: Op,
        curr_res: u128,
        remaining_values: &[u128],
        curr_ops: &[Op],
        ops_acc: &mut HashSet<(Self, Vec<Op>)>,
        candidates: &[Op],
    ) {
        if remaining_values.is_empty() && curr_res == self.target {
            ops_acc.insert((self.clone(), curr_ops.to_vec()));
            return;
        }
        if remaining_values.is_empty() || curr_res > self.target {
            return;
        }
        let next_res = match curr_op {
            Op::Add => curr_res + remaining_values[0],
            Op::Mul => curr_res * remaining_values[0],
            Op::Concat => {
                curr_res * 10_i32.pow(remaining_values[0].ilog10() + 1) as u128
                    + remaining_values[0]
            }
        };

        let mut next_ops = curr_ops.to_vec();
        if !remaining_values.is_empty() && candidates.contains(&curr_op) {
            next_ops.push(curr_op);
        }
        candidates.iter().for_each(|&op| {
            self.find_valid_ops(
                op,
                next_res,
                &remaining_values[1..],
                &next_ops,
                ops_acc,
                candidates,
            );
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Op {
    Add,
    Mul,
    Concat,
}
impl Op {
    fn candidates(skip_concat: bool) -> &'static [Self] {
        let all = &[Op::Add, Op::Mul, Op::Concat];
        match skip_concat {
            true => &all[0..2],
            false => all,
        }
    }
}

fn read_input(filename: &str) -> Vec<CalibrationEquation> {
    utils::read_to_string_in_module!(filename)
        .lines()
        .map(|line| {
            let (target, values) = line.split_once(": ").unwrap();
            CalibrationEquation {
                target: target.parse().unwrap(),
                values: values
                    .split(' ')
                    .map(|val| val.trim().parse().unwrap())
                    .collect(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::compute_sum;
    use super::read_input;
    use super::Op;

    #[test]
    fn tests() {
        let equations = read_input("test-input.txt");
        assert_eq!(compute_sum(&equations, Op::candidates(true)), 3749);
        assert_eq!(compute_sum(&equations, Op::candidates(false)), 11387);
    }

    #[test]
    fn prod() {
        let equations = read_input("input.txt");
        assert_eq!(compute_sum(&equations, Op::candidates(true)), 2654749936343);
        assert_eq!(
            compute_sum(&equations, Op::candidates(false)),
            124060392153684
        );
    }
}
