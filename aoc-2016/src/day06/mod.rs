use std::collections::HashMap;

pub fn run() {
    let messages = read("input.txt");
    let mut appearances = vec![HashMap::new(); messages[0].len()];
    for m in messages {
        for (i, ch) in m.char_indices() {
            *appearances[i].entry(ch).or_insert(0) += 1;
        }
    }
    println!(
        "part1 solution: {}",
        appearances
            .iter()
            .map(|map| {
                let mut entries = map.iter().collect::<Vec<_>>();
                entries.sort_by(|a, b| b.1.cmp(a.1));
                *entries[0].0
            })
            .collect::<String>()
    );
    println!(
        "part2 solution: {}",
        appearances
            .iter()
            .map(|map| {
                let mut entries = map.iter().collect::<Vec<_>>();
                entries.sort_by(|a, b| a.1.cmp(b.1));
                *entries[0].0
            })
            .collect::<String>()
    );
}

fn read(filename: &str) -> Vec<String> {
    utils::read_to_string_in_module!(filename)
        .split_terminator('\n')
        .map(|s| s.into())
        .collect()
}
