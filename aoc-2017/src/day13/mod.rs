use std::collections::{BTreeMap, HashMap};

pub fn run() {
    let layers = read("input.txt");
    println!("part1 solution: {}", calculate_trip_severity(&layers));
    println!("part2 solution: {}", find_best_delay(&layers));
}

fn calculate_trip_severity(layers: &BTreeMap<usize, usize>) -> usize {
    let mut cought = HashMap::new();
    for (layer, size) in layers {
        let cycle = (size - 1) * 2;
        if layer % cycle == 0 {
            cought.insert(layer, size);
        }
    }

    cought.iter().map(|(&&l, &&size)| l * size).sum()
}

fn find_best_delay(layers: &BTreeMap<usize, usize>) -> usize {
    let mut delay = 1;
    while layers
        .iter()
        .any(|(&l, &size)| (l + delay) % ((size - 1) * 2) == 0)
    {
        delay += 1;
    }

    delay
}

fn read(filename: &str) -> BTreeMap<usize, usize> {
    utils::read_to_string_in_module!(filename)
        .lines()
        .map(|s| {
            let parts = s
                .split_terminator(": ")
                .filter_map(|p| p.parse().ok())
                .collect::<Vec<_>>();
            (parts[0], parts[1])
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let layers = read("test-input.txt");
        assert_eq!(calculate_trip_severity(&layers), 24);
        assert_eq!(find_best_delay(&layers), 10);
    }
}
