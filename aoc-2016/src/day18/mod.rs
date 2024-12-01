pub fn run() {
    let input = ".^.^..^......^^^^^...^^^...^...^....^^.^...^.^^^^....^...^^.^^^...^^^^.^^.^.^^..^.^^^..^^^^^^.^^^..^";
    println!(
        "part1 solution: {:?}",
        generate_rows(input.chars().collect(), 40)
            .iter()
            .map(|row| row.iter().filter(|&&ch| ch == '.').count())
            .sum::<usize>()
    );

    println!(
        "part2 solution: {:?}",
        generate_rows(input.chars().collect(), 400000)
            .iter()
            .map(|row| row.iter().filter(|&&ch| ch == '.').count())
            .sum::<usize>()
    );
}

fn generate_rows(initial_row: Vec<char>, num_of_rows: usize) -> Vec<Vec<char>> {
    let mut rows = vec![initial_row];
    let mut i = 0;
    while rows.len() < num_of_rows {
        let mut next_row = Vec::new();
        for j in 0..rows[0].len() {
            let left = if j == 0 { '.' } else { rows[i][j - 1] };
            let right = if j == rows[i].len() - 1 {
                '.'
            } else {
                rows[i][j + 1]
            };
            if left == '^' && right != '^' || left != '^' && right == '^' {
                next_row.push('^');
            } else {
                next_row.push('.');
            }
        }

        rows.push(next_row);
        i += 1;
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_tests() {
        assert_eq!(
            generate_rows("..^^.".chars().collect(), 3),
            vec![
                vec!['.', '.', '^', '^', '.'],
                vec!['.', '^', '^', '^', '^'],
                vec!['^', '^', '.', '.', '^'],
            ]
        );
        assert_eq!(
            generate_rows(".^^.^.^^^^".chars().collect(), 10),
            vec![
                vec!['.', '^', '^', '.', '^', '.', '^', '^', '^', '^'],
                vec!['^', '^', '^', '.', '.', '.', '^', '.', '.', '^'],
                vec!['^', '.', '^', '^', '.', '^', '.', '^', '^', '.'],
                vec!['.', '.', '^', '^', '.', '.', '.', '^', '^', '^'],
                vec!['.', '^', '^', '^', '^', '.', '^', '^', '.', '^'],
                vec!['^', '^', '.', '.', '^', '.', '^', '^', '.', '.'],
                vec!['^', '^', '^', '^', '.', '.', '^', '^', '^', '.'],
                vec!['^', '.', '.', '^', '^', '^', '^', '.', '^', '^'],
                vec!['.', '^', '^', '^', '.', '.', '^', '.', '^', '^'],
                vec!['^', '^', '.', '^', '^', '^', '.', '.', '^', '^'],
            ]
        );
    }
}
