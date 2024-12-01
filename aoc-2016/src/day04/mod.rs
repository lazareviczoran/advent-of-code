use std::collections::HashMap;

pub fn run() {
    let rooms = read("input.txt");
    let real_rooms = get_real_rooms(&rooms);
    println!(
        "part1 solution: {}",
        real_rooms.iter().map(|s| s.1).sum::<usize>()
    );

    let decrypted = decrypt_rooms(&real_rooms);
    println!(
        "part2 solution: {:?}",
        decrypted
            .iter()
            .find(|(s, _id)| s == "northpole object storage")
            .map(|(_s, id)| id)
            .expect("not found")
    );
}

fn decrypt_rooms(rooms: &[(String, usize, String)]) -> Vec<(String, usize)> {
    rooms
        .iter()
        .map(|(content, id, _)| {
            let step = (id % 26) as u8;
            let mut s = String::new();
            for ch in content.chars() {
                if ch == '-' {
                    s.push(' ');
                } else {
                    s.push(((ch as u8 - b'a' + step) % 26 + b'a') as char);
                }
            }
            s.pop();
            (s, *id)
        })
        .collect::<Vec<_>>()
}

fn get_real_rooms(rooms: &[(String, usize, String)]) -> Vec<(String, usize, String)> {
    rooms
        .iter()
        .filter(|r| is_real(r))
        .cloned()
        .collect::<Vec<_>>()
}

fn is_real(room: &(String, usize, String)) -> bool {
    let mut counts = HashMap::new();
    for ch in room.0.chars() {
        if ch != '-' {
            *counts.entry(ch).or_insert(0) += 1;
        }
    }
    let mut most_frequent = counts.iter().collect::<Vec<_>>();
    most_frequent.sort_by(|a, b| b.1.cmp(a.1));

    for curr_ch in room.2.chars() {
        let first_count = most_frequent[0].1;
        let mut removed = false;
        let mut i = 0;
        while i < most_frequent.len() && most_frequent[i].1 == first_count {
            if most_frequent[i].0 == &curr_ch {
                most_frequent.remove(i);
                removed = true;
                continue;
            }
            i += 1;
        }
        if !removed {
            return false;
        }
    }
    true
}

fn read(filename: &str) -> Vec<(String, usize, String)> {
    utils::read_to_string_in_module!(filename)
        .split_terminator('\n')
        .filter_map(|s| {
            let parts = s.split_terminator('[').collect::<Vec<_>>();
            let (p1, p2) = parts[0].split_at(parts[0].len() - 3);
            let p3 = parts[1].strip_suffix(']')?;
            Some((p1.to_string(), p2.parse().ok()?, p3.to_string()))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let rooms = vec![
            ("aaaaa-bbb-z-y-x-".into(), 123, "abxyz".into()),
            ("a-b-c-d-e-f-g-h-".into(), 987, "abcde".into()),
            ("not-a-real-room-".into(), 404, "oarel".into()),
            ("totally-real-room-".into(), 200, "decoy".into()),
        ];
        assert_eq!(
            get_real_rooms(&rooms).iter().map(|s| s.1).sum::<usize>(),
            1514
        );
    }

    #[test]
    fn part2_test() {
        let room = vec![("qzmt-zixmtkozy-ivhz-".into(), 343, "asfdf".into())];
        assert_eq!(decrypt_rooms(&room), [("very encrypted name".into(), 343)])
    }
}
