pub fn run() {
    let equations = read_input("input.txt");

    utils::run_solution!(|| compute_sum(&equations, Op::candidates(true)), "part1");
    utils::run_solution!(|| compute_sum(&equations, Op::candidates(false)), "part2");
}

fn compute_sum(equations: &[CalibrationEquation], candidates: &[Op]) -> i128 {
    equations
        .iter()
        .filter(|eq| eq.is_valid(candidates))
        .map(|eq| eq.target)
        .sum()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CalibrationEquation {
    target: i128,
    values: Vec<i128>,
}
impl CalibrationEquation {
    fn is_valid(&self, candidates: &[Op]) -> bool {
        self.find_valid_ops_rec(candidates)
    }

    fn find_valid_ops_rec(&self, candidates: &[Op]) -> bool {
        candidates
            .iter()
            .any(|&op| Self::find_valid_ops(op, self.target, &self.values[..], candidates))
    }

    fn find_valid_ops(
        curr_op: Op,
        curr_res: i128,
        remaining_values: &[i128],
        candidates: &[Op],
    ) -> bool {
        if remaining_values.is_empty() && curr_res == 0 {
            return true;
        }
        if remaining_values.is_empty() || curr_res <= 0 {
            return false;
        }
        let curr_item = *remaining_values.last().unwrap();
        let next_res = match curr_op {
            Op::Add => curr_res - curr_item,
            Op::Mul => {
                if curr_res % curr_item == 0 {
                    curr_res / curr_item
                } else {
                    return false;
                }
            }
            Op::Concat => {
                let val = 10_i32.pow(curr_item.ilog10() + 1) as i128;
                if curr_res - curr_item > 0 && (curr_res - curr_item) % val == 0 {
                    curr_res / val
                } else {
                    return false;
                }
            }
        };

        candidates.iter().any(|&op| {
            Self::find_valid_ops(
                op,
                next_res,
                &remaining_values[..remaining_values.len() - 1],
                candidates,
            )
        })
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
    #[cfg(feature = "include-main-input")]
    fn prod() {
        use itertools::Itertools;
        let (pt1, pt2) = utils::read_to_string_in_module!("results.txt")
            .lines()
            .map(|line| line.parse::<i128>().unwrap())
            .collect_tuple()
            .unwrap();
        let equations = read_input("input.txt");
        assert_eq!(compute_sum(&equations, Op::candidates(true)), pt1);
        assert_eq!(compute_sum(&equations, Op::candidates(false)), pt2);
    }
}
