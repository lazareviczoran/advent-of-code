pub fn run() {
    println!(
        "part1 solution: {}",
        solve(
            (0..256).collect(),
            &[63, 144, 180, 149, 1, 255, 167, 84, 125, 65, 188, 0, 2, 254, 229, 24]
        )
    );

    println!(
        "part2 solution: {}",
        knot_hash("63,144,180,149,1,255,167,84,125,65,188,0,2,254,229,24")
    );
}

pub fn knot_hash(lengths: &str) -> String {
    let mut items = (0..256).collect::<Vec<_>>();
    let mut lengths = lengths.as_bytes().to_vec();
    lengths.extend(&[17, 31, 73, 47, 23]);
    let n = items.len();
    let mut curr_idx = 0;
    let mut skip = 0;
    for _ in 0..64 {
        for &l in &lengths {
            for i in 0..l / 2 {
                let from = (curr_idx + i as usize) % n;
                let to = (curr_idx + l as usize - i as usize - 1) % n;
                items.swap(from, to);
            }
            curr_idx += l as usize + skip;
            curr_idx %= n;
            skip += 1;
        }
    }

    items
        .chunks(16)
        .map(|chunk| chunk.iter().fold(0, |acc, val| acc ^ val))
        .fold(String::new(), |mut acc, val| {
            acc.push_str(&format!("{:02x}", val));
            acc
        })
}

fn solve(mut items: Vec<usize>, lengths: &[usize]) -> usize {
    let n = items.len();
    let mut curr_idx = 0;
    for (skip, l) in lengths.iter().enumerate() {
        for i in 0..l / 2 {
            let from = (curr_idx + i) % n;
            let to = (curr_idx + l - i - 1) % n;
            items.swap(from, to);
        }
        curr_idx += l + skip;
        curr_idx %= n;
    }

    items[0] * items[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(solve(vec![0, 1, 2, 3, 4], &[3, 4, 1, 5]), 12);

        assert_eq!(knot_hash(""), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(knot_hash("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(knot_hash("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(knot_hash("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
