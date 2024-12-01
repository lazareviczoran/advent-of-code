use std::collections::VecDeque;

pub fn run() {
    let step_size = 343;
    println!("part1 solution: {}", find_value_after_2017(step_size));
    println!("part2 solution: {}", find_value_after_0(step_size));
}

fn find_value_after_2017(step_size: usize) -> usize {
    let mut values = VecDeque::from(vec![0]);
    for i in 1..2018 {
        values.rotate_right(step_size % values.len());
        values.push_front(i);
    }
    values.rotate_right(1);
    *values.front().unwrap()
}

fn find_value_after_0(step_size: usize) -> usize {
    let mut values = VecDeque::from(vec![0]);
    for i in 1..50_000_000 {
        values.rotate_right(step_size % values.len());
        values.push_front(i);
    }
    values.rotate_left(values.iter().position(|&c| c == 0).unwrap());
    values.rotate_right(1);
    *values.front().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(find_value_after_2017(3), 638);
    }
}
