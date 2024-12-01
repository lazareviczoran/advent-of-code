pub fn run() {
    let num_of_elves = 3005290;
    println!(
        "part1 solution: {}",
        find_elf_with_all_presents(num_of_elves)
    );
    println!(
        "part2 solution: {}",
        find_elf_with_all_presents2(num_of_elves)
    );
}

fn find_elf_with_all_presents2(num_of_elves: usize) -> usize {
    let mut indices = (0..num_of_elves)
        .map(|i| {
            (
                (i + num_of_elves - 1) % num_of_elves,
                (i + 1) % num_of_elves,
            )
        })
        .collect::<Vec<_>>();
    let mut remaining = num_of_elves;
    let mut current = (num_of_elves / 2) % num_of_elves;
    while remaining > 0 {
        let target = indices[current];
        indices[target.1].0 = target.0;
        indices[target.0].1 = target.1;
        match remaining % 2 {
            0 => current = target.1,
            1 => current = indices[target.1].1,
            _ => unreachable!(),
        }
        remaining -= 1;
    }

    current + 1
}

fn find_elf_with_all_presents(num_of_elves: usize) -> usize {
    let mut indices = (0..num_of_elves)
        .map(|i| {
            (
                (i + num_of_elves - 1) % num_of_elves,
                (i + 1) % num_of_elves,
            )
        })
        .collect::<Vec<_>>();

    let mut current = 0;
    while indices[current].0 != indices[current].1 {
        let next = indices[indices[current].1].1;
        indices[next].0 = current;
        indices[current].1 = next;
        current = next;
    }

    current + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(find_elf_with_all_presents(5), 3);
    }

    #[test]
    fn part2_test() {
        assert_eq!(find_elf_with_all_presents2(5), 2);
    }
}
