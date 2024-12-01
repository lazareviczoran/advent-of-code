use std::collections::HashMap;

pub fn run() {
    let input = "zpqevtbw";
    println!("part1 solution: {}", find_64th_key_index(input, false));
    println!("part2 solution: {}", find_64th_key_index(input, true));
}

fn find_64th_key_index(salt: &str, stretched: bool) -> usize {
    let mut generated_keys = HashMap::new();
    let mut found_keys = 0;
    let mut idx = 0;

    loop {
        let hash = generated_keys
            .entry(idx)
            .or_insert_with(|| apply_hash(&format!("{}{}", salt, idx), stretched))
            .clone();

        if let Some(candidate3) = hash
            .as_bytes()
            .windows(3)
            .find(|w| w[0] == w[1] && w[0] == w[2])
        {
            for i in idx + 1..=idx + 1000 {
                let next_hash = generated_keys
                    .entry(i)
                    .or_insert_with(|| apply_hash(&format!("{}{}", salt, i), stretched));
                if next_hash
                    .as_bytes()
                    .windows(5)
                    .any(|w| w.iter().all(|&ch| ch == candidate3[0]))
                {
                    found_keys += 1;
                    if found_keys == 64 {
                        return idx;
                    }
                    break;
                }
            }
        }
        idx += 1;
    }
}

fn apply_hash(msg: &str, stretched: bool) -> String {
    let mut hash = format!("{:?}", md5::compute(msg));
    if stretched {
        for _ in 0..2016 {
            hash = format!("{:?}", md5::compute(hash));
        }
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(find_64th_key_index("abc", false), 22728);
    }

    #[test]
    fn part2_test() {
        assert_eq!(find_64th_key_index("abc", true), 22551);
    }
}
