pub fn run() {
    let blacklist = read("input.txt");
    let allowed_ips = get_allowed_ips(&blacklist);
    println!("part1 solution: {:?}", allowed_ips[0].0);
    println!(
        "part2 solution: {:?}",
        allowed_ips
            .iter()
            .map(|range| range.1 - range.0 + 1)
            .sum::<u32>()
    );
}

fn get_allowed_ips(blacklist: &[(u32, u32)]) -> Vec<(u32, u32)> {
    let mut allowed_ranges = vec![(0, u32::MAX)];
    for &(from, to) in blacklist {
        allowed_ranges = allowed_ranges.iter().fold(vec![], |mut acc, range| {
            if range.0 > from && range.1 < to {
                return acc;
            }
            if range.1 < from || range.0 > to {
                acc.push(*range);
            }
            if range.0 < from && range.1 > to {
                acc.push((range.0, from - 1));
                acc.push((to + 1, range.1));
            }
            if range.0 < from && range.1 >= from && range.1 <= to {
                acc.push((range.0, from - 1));
            }
            if range.0 >= from && range.0 <= to && range.1 > to {
                acc.push((to + 1, range.1));
            }
            acc
        });
    }

    allowed_ranges
}

fn read(filename: &str) -> Vec<(u32, u32)> {
    utils::read_to_string_in_module!(filename)
        .split_terminator('\n')
        .map(|s| {
            let parts = s
                .split_terminator('-')
                .filter_map(|n| n.parse().ok())
                .collect::<Vec<_>>();
            (parts[0], parts[1])
        })
        .collect()
}
