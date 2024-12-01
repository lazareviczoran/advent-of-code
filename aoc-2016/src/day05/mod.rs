pub fn run() {
    let door_id = "abbhdwsy";
    println!("part1 solution: {}", find_password(door_id));
    println!("part2 solution: {}", find_password2(door_id));
}

fn find_password(door_id: &str) -> String {
    let mut pass = String::new();
    let mut i = 0;
    for _ in 0..8 {
        loop {
            let hash = format!("{:?}", md5::compute(format!("{}{}", door_id, i)));
            i += 1;
            if &hash[0..5] == "00000" {
                pass.push(hash.chars().nth(5).unwrap());
                break;
            }
        }
    }

    pass
}

fn find_password2(door_id: &str) -> String {
    let mut pass = [' '; 8];
    let mut used = [false; 8];
    let mut i = 0;
    loop {
        if used.iter().all(|x| *x) {
            break;
        }
        let hash = format!("{:?}", md5::compute(format!("{}{}", door_id, i)));
        i += 1;
        if &hash[0..5] == "00000" {
            let pos = (hash.chars().nth(5).unwrap() as u8 - b'0') as usize;
            if (0..=7).contains(&pos) && !used[pos] {
                used[pos] = true;
                pass[pos] = hash.chars().nth(6).unwrap();
            }
        }
    }

    pass.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let door_id = "abc";
        assert_eq!(find_password(door_id), "18f47a30");
    }

    #[test]
    fn part2_test() {
        let door_id = "abc";
        assert_eq!(find_password2(door_id), "05ace8e3");
    }
}
