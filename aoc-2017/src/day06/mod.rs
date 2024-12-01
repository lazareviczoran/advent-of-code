use std::collections::HashMap;

pub fn run() {
    let memory_banks = vec![2, 8, 8, 5, 4, 2, 3, 1, 5, 5, 1, 2, 15, 13, 5, 14];

    let (first_repeated_pos, loop_size) = find_steps_til_first_repeated_state(&memory_banks);
    println!("part1 solution: {:?}", first_repeated_pos);
    println!("part2 solution: {:?}", loop_size);
}

fn find_steps_til_first_repeated_state(memory: &[usize]) -> (usize, usize) {
    let mut i = 0;
    let mut memo = HashMap::new();
    let mut curr_state = memory.to_vec();

    while !memo.contains_key(&curr_state) {
        memo.insert(curr_state.clone(), i);
        let (mut pos, mut val) =
            curr_state
                .iter()
                .enumerate()
                .fold(
                    (0, curr_state[0]),
                    |acc, (pos, &m)| if m > acc.1 { (pos, m) } else { acc },
                );
        curr_state[pos] = 0;
        while val > 0 {
            pos = (pos + 1) % curr_state.len();
            curr_state[pos] += 1;
            val -= 1;
        }
        i += 1;
    }

    (i, i - memo.get(&curr_state).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(find_steps_til_first_repeated_state(&[0, 2, 7, 0]), (5, 4));
    }
}
