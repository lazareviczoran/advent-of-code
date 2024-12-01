pub fn run() {
    // Disc #1 has 5 positions; at time=0, it is at position 2.
    // Disc #2 has 13 positions; at time=0, it is at position 7.
    // Disc #3 has 17 positions; at time=0, it is at position 10.
    // Disc #4 has 3 positions; at time=0, it is at position 2.
    // Disc #5 has 19 positions; at time=0, it is at position 9.
    // Disc #6 has 7 positions; at time=0, it is at position 0.

    let mut discs = vec![(5, 2), (13, 7), (17, 10), (3, 2), (19, 9), (7, 0)];
    println!(
        "part1 solution: {:?}",
        find_first_time_when_passes_all(&discs)
    );

    discs.push((11, 0));
    println!(
        "part2 solution: {:?}",
        find_first_time_when_passes_all(&discs)
    );
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

fn find_first_time_when_passes_all(discs: &[(usize, usize)]) -> usize {
    let residues = discs
        .iter()
        .enumerate()
        .map(|(i, (positions, curr_pos))| {
            let pos = (*curr_pos + i + 1) % *positions;
            ((positions - pos) % positions) as i64
        })
        .collect::<Vec<_>>();
    let modulii = discs
        .iter()
        .map(|(positions, _)| *positions as i64)
        .collect::<Vec<_>>();

    match chinese_remainder(&residues, &modulii) {
        Some(sol) => sol as usize,
        None => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        // Disc #1 has 5 positions; at time=0, it is at position 4.
        // Disc #2 has 2 positions; at time=0, it is at position 1.
        let discs = vec![(5, 4), (2, 1)];
        assert_eq!(find_first_time_when_passes_all(&discs), 5);
    }
}
