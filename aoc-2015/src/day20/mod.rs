pub fn run() {
    let input = 36000000;

    utils::run_solution!(|| find_first_house_with_at_least_n_presents(input), "part1");

    utils::run_solution!(
        || find_first_house_with_at_least_n_presents_p2(input),
        "part2"
    );
}

fn find_first_house_with_at_least_n_presents(n: usize) -> usize {
    let limit = n / 10;
    let mut houses = vec![0; limit];
    for elf in 1..limit {
        let mut house = elf;
        while house < limit {
            houses[house] += elf * 10;
            house += elf;
        }
    }

    houses.into_iter().position(|p| p >= n).unwrap()
}

fn find_first_house_with_at_least_n_presents_p2(n: usize) -> usize {
    let limit = n / 11;
    let mut houses = vec![0; limit];
    for elf in 1..n {
        let mut house = elf;
        let mut count = 0;
        while house < limit && count < 50 {
            houses[house] += elf * 11;
            house += elf;
            count += 1;
        }
    }

    houses.into_iter().position(|p| p >= n).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_find_first_house_with_at_least_n_presents() {
        assert_eq!(find_first_house_with_at_least_n_presents(130), 8);
        assert_eq!(find_first_house_with_at_least_n_presents(60), 4);
        assert_eq!(find_first_house_with_at_least_n_presents(120), 6);
        assert_eq!(find_first_house_with_at_least_n_presents(80), 6);
        assert_eq!(find_first_house_with_at_least_n_presents(125), 8);
    }
}
