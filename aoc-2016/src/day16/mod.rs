pub fn run() {
    let input = "11110010111001001";
    println!("part1 solution: {}", checksum(272, input.into()));
    println!("part2 solution: {}", checksum(35651584, input.into()));
}

fn checksum(disk_size: usize, initial_state: String) -> String {
    let data = fill_disk(disk_size, initial_state);
    find_checksum_for(data)
}

fn fill_disk(disk_size: usize, initial_state: String) -> String {
    let mut res = initial_state;
    while res.len() < disk_size {
        let other = res
            .chars()
            .rev()
            .map(|ch| if ch == '1' { '0' } else { '1' })
            .collect::<String>();
        res.push('0');
        res.push_str(&other);
    }

    res.truncate(disk_size);
    res
}

fn find_checksum_for(data: String) -> String {
    let mut res = data;
    while res.len() % 2 == 0 {
        res = res
            .as_bytes()
            .chunks_exact(2)
            .map(|pair| if pair[0] == pair[1] { '1' } else { '0' })
            .collect();
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fill_disk() {
        assert_eq!(fill_disk(3, "1".into()), "100");
        assert_eq!(fill_disk(3, "0".into()), "001");
        assert_eq!(fill_disk(11, "11111".into()), "11111000000");
        assert_eq!(
            fill_disk(25, "111100001010".into()),
            "1111000010100101011110000"
        );
    }

    #[test]
    fn test_find_checksum_for() {
        assert_eq!(find_checksum_for("110010110100".into()), "100");
    }

    #[test]
    fn test_checksum() {
        assert_eq!(checksum(20, "10000".into()), "01100");
    }
}
