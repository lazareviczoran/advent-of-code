pub fn run() {
    let packages = vec![
        1, 2, 3, 7, 11, 13, 17, 19, 23, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
        101, 103, 107, 109, 113,
    ];

    utils::run_solution!(|| find_ideal_quantum_entanglement(&packages, 3), "part1");

    utils::run_solution!(|| find_ideal_quantum_entanglement(&packages, 4), "part2");
}

fn find_ideal_quantum_entanglement(packages: &Vec<usize>, num_of_groups: usize) -> u128 {
    let group_weight = packages.iter().sum::<usize>() / num_of_groups;

    // generate subsets
    let mut best_subset = (usize::MAX, u128::MAX);
    find_best_subset(
        packages,
        packages,
        group_weight,
        num_of_groups,
        num_of_groups,
        vec![],
        &mut best_subset,
    );

    best_subset.1
}

fn find_best_subset(
    packages: &Vec<usize>,
    curr_input: &[usize],
    target_sum: usize,
    num_of_groups: usize,
    remaining_groups: usize,
    curr_groups: Vec<usize>,
    best: &mut (usize, u128),
) {
    if remaining_groups == 1 {
        let mut quantum_entanglement = 1;
        for (i, &item) in packages.iter().enumerate() {
            if has_bit(curr_groups[0], i as u8) {
                quantum_entanglement *= item as u128;
            }
        }
        if best.1 > quantum_entanglement {
            *best = (curr_groups[0], quantum_entanglement);
        }
        return;
    }

    let subsets = generate_subsets_dp(curr_input, target_sum);
    for s in subsets {
        if remaining_groups == num_of_groups && s.count_ones() > best.0.count_ones() {
            return;
        }
        let complement_set = !s;
        let mut new_input = Vec::new();
        let mut new_input_sum = 0;
        for (i, &item) in curr_input.iter().enumerate() {
            if has_bit(complement_set, i as u8) {
                new_input.push(item);
                new_input_sum += item;
            }
        }
        if remaining_groups - 1 == 1 && new_input_sum != target_sum {
            continue;
        }
        let mut new_curr_groups = curr_groups.clone();
        new_curr_groups.push(s);
        find_best_subset(
            packages,
            &new_input,
            target_sum,
            num_of_groups,
            remaining_groups - 1,
            new_curr_groups,
            best,
        );
    }
}

fn generate_subsets_dp(input: &[usize], target_sum: usize) -> Vec<usize> {
    let n = input.len();
    let mut memo = vec![vec![false; target_sum + 1]; n];
    for item in memo.iter_mut().take(n) {
        item[0] = true;
    }
    if input[0] <= target_sum {
        memo[0][input[0]] = true;
    }
    for i in 1..n {
        for j in 0..=target_sum {
            if input[i] <= j {
                memo[i][j] = memo[i - 1][j] || memo[i - 1][j - input[i]];
            } else {
                memo[i][j] = memo[i - 1][j];
            }
        }
    }

    let mut valid_subsets: Vec<usize> = Vec::new();
    let mut queue: Vec<(usize, usize, usize)> = vec![(n - 1, target_sum, 0)];
    while !queue.is_empty() {
        let (curr_i, sum, mut curr_subset) = queue.remove(0);

        // If we reached end and sum is non-zero. We include
        // subset only if input[0] is equal to sum OR memo[0][sum]
        // is true.
        if curr_i == 0 && sum != 0 && memo[0][sum] {
            curr_subset = set_bit(curr_subset, curr_i as u8);
            if valid_subsets.is_empty() || curr_subset.count_ones() <= valid_subsets[0].count_ones()
            {
                valid_subsets.insert(0, curr_subset);
            }

            continue;
        }

        // If sum becomes 0
        if curr_i == 0 && sum == 0 {
            if valid_subsets.is_empty() || curr_subset.count_ones() <= valid_subsets[0].count_ones()
            {
                valid_subsets.insert(0, curr_subset);
            }
            continue;
        }

        // If given sum can be achieved after ignoring
        // current element.
        if memo[curr_i - 1][sum] {
            queue.insert(0, (curr_i - 1, sum, curr_subset));
        }

        // If given sum can be achieved after considering
        // current element.
        if sum >= input[curr_i] && memo[curr_i - 1][sum - input[curr_i]] {
            curr_subset = set_bit(curr_subset, curr_i as u8);
            queue.insert(0, (curr_i - 1, sum - input[curr_i], curr_subset))
        }
    }

    valid_subsets
}

fn set_bit(keys: usize, i: u8) -> usize {
    keys | (1 << i)
}

fn has_bit(keys: usize, i: u8) -> bool {
    keys & (1 << i) == (1 << i)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let packages = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        assert_eq!(find_ideal_quantum_entanglement(&packages, 3), 99);
    }

    #[test]
    fn part2_test() {
        let packages = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        assert_eq!(find_ideal_quantum_entanglement(&packages, 4), 44);
    }
}
