pub fn run() {
    let ips = read("input.txt");

    println!(
        "part1 solution: {}",
        ips.iter().filter(|&s| is_tls(s)).count()
    );
    println!(
        "part2 solution: {}",
        ips.iter().filter(|&s| is_ssl(s)).count()
    );
}

fn is_ssl(ip: &[Vec<char>]) -> bool {
    ip.iter().step_by(2).any(|seg| {
        seg.windows(3).any(|w| {
            is_aba(w)
                && ip
                    .iter()
                    .skip(1)
                    .step_by(2)
                    .any(|seg2| seg2.windows(3).any(|w2| w2 == [w[1], w[0], w[1]]))
        })
    })
}

fn is_aba(segment: &[char]) -> bool {
    segment[0] != segment[1] && segment[0] == segment[2]
}

fn is_tls(ip: &[Vec<char>]) -> bool {
    ip.iter()
        .skip(1)
        .step_by(2)
        .all(|seg| seg.windows(4).all(|w| !is_abba(w)))
        && ip
            .iter()
            .step_by(2)
            .any(|seg| seg.windows(4).any(|w| is_abba(w)))
}

fn is_abba(segment: &[char]) -> bool {
    segment[0] != segment[1] && segment[0] == segment[3] && segment[1] == segment[2]
}

fn read(filename: &str) -> Vec<Vec<Vec<char>>> {
    utils::read_to_string_in_module!(filename)
        .split_terminator('\n')
        .map(|s| {
            s.split(|c| c == '[' || c == ']')
                .map(|s| s.chars().collect())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let ips = read("test-input.txt");
        assert_eq!(ips.iter().filter(|s| is_tls(s)).count(), 2);
    }

    #[test]
    fn part2_test() {
        let ips = read("test-input2.txt");
        assert_eq!(ips.iter().filter(|s| is_ssl(s)).count(), 3);
    }
}
