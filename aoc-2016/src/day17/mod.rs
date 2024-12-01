use std::collections::VecDeque;

pub fn run() {
    let passcode = "pgflpeqp";
    println!("part1 solution: {}", find_shortest_path(passcode));
    println!("part2 solution: {}", find_longest_path_length(passcode));
}

fn find_longest_path_length(passcode: &str) -> usize {
    let target_pos = (3, 3);
    let mut max_length = 0;
    let mut q = VecDeque::from(vec![(passcode.to_string(), (0, 0), 0)]);
    while !q.is_empty() {
        let (curr_passcode, curr_pos, steps) = q.pop_front().unwrap();
        if curr_pos == target_pos {
            if steps > max_length {
                max_length = steps;
            }
            continue;
        }
        let hash = format!("{:?}", md5::compute(&curr_passcode));
        let mut chars = hash.chars();
        if matches!(chars.next().unwrap(), 'b'..='f') && curr_pos.1 > 0 {
            // move up
            let mut next_passcode = curr_passcode.clone();
            next_passcode.push('U');
            q.push_back((next_passcode, (curr_pos.0, curr_pos.1 - 1), steps + 1));
        }
        if matches!(chars.next().unwrap(), 'b'..='f') && curr_pos.1 < 3 {
            // move down
            let mut next_passcode = curr_passcode.clone();
            next_passcode.push('D');
            q.push_back((next_passcode, (curr_pos.0, curr_pos.1 + 1), steps + 1));
        }
        if matches!(chars.next().unwrap(), 'b'..='f') && curr_pos.0 > 0 {
            // move left
            let mut next_passcode = curr_passcode.clone();
            next_passcode.push('L');
            q.push_back((next_passcode, (curr_pos.0 - 1, curr_pos.1), steps + 1));
        }
        if matches!(chars.next().unwrap(), 'b'..='f') && curr_pos.0 < 3 {
            // move right
            let mut next_passcode = curr_passcode.clone();
            next_passcode.push('R');
            q.push_back((next_passcode, (curr_pos.0 + 1, curr_pos.1), steps + 1));
        }
    }
    max_length
}

fn find_shortest_path(passcode: &str) -> String {
    let target_pos = (3, 3);
    let mut q = VecDeque::from(vec![(passcode.to_string(), (0, 0))]);
    while !q.is_empty() {
        let (curr_passcode, curr_pos) = q.pop_front().unwrap();
        if curr_pos == target_pos {
            return curr_passcode.strip_prefix(passcode).unwrap().into();
        }
        let hash = format!("{:?}", md5::compute(&curr_passcode));
        let mut chars = hash.chars();
        if matches!(chars.next().unwrap(), 'b'..='f') && curr_pos.1 > 0 {
            // move up
            let mut next_passcode = curr_passcode.clone();
            next_passcode.push('U');
            q.push_back((next_passcode, (curr_pos.0, curr_pos.1 - 1)));
        }
        if matches!(chars.next().unwrap(), 'b'..='f') && curr_pos.1 < 3 {
            // move down
            let mut next_passcode = curr_passcode.clone();
            next_passcode.push('D');
            q.push_back((next_passcode, (curr_pos.0, curr_pos.1 + 1)));
        }
        if matches!(chars.next().unwrap(), 'b'..='f') && curr_pos.0 > 0 {
            // move left
            let mut next_passcode = curr_passcode.clone();
            next_passcode.push('L');
            q.push_back((next_passcode, (curr_pos.0 - 1, curr_pos.1)));
        }
        if matches!(chars.next().unwrap(), 'b'..='f') && curr_pos.0 < 3 {
            // move right
            let mut next_passcode = curr_passcode.clone();
            next_passcode.push('R');
            q.push_back((next_passcode, (curr_pos.0 + 1, curr_pos.1)));
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_tests() {
        assert_eq!(find_shortest_path("ihgpwlah"), "DDRRRD");
        assert_eq!(find_shortest_path("kglvqrro"), "DDUDRLRRUDRD");
        assert_eq!(
            find_shortest_path("ulqzkmiv"),
            "DRURDRUDDLLDLUURRDULRLDUUDDDRR"
        );
    }

    #[test]
    fn part2_tests() {
        assert_eq!(find_longest_path_length("ihgpwlah"), 370);
        assert_eq!(find_longest_path_length("kglvqrro"), 492);
        assert_eq!(find_longest_path_length("ulqzkmiv"), 830);
    }
}
