pub fn run() {
    // Input text:
    // To continue, please consult the code grid in the manual.
    // Enter the code at row 3010, column 3019.
    let mut table = vec![vec![-1; 7000]; 7000];
    table[0][0] = 20151125;

    utils::run_solution!(|| get_code_at(&mut table, 3010, 3019), "part1");
}

fn get_code_at(table: &mut [Vec<i128>], row: usize, column: usize) -> i128 {
    let actual_row = row - 1;
    let actual_column = column - 1;
    let mut i = 0;
    let mut j = 0;
    let mut diagonal_count = 1;
    while i != actual_row || j != actual_column {
        let prev_row = i;
        let prev_column = j;
        if i == 0 {
            i = diagonal_count;
            j = 0;
            diagonal_count += 1;
        } else {
            j += 1;
            i -= 1;
        }
        if table[i][j] < 0 {
            table[i][j] = (table[prev_row][prev_column] * 252533) % 33554393;
        }
    }
    table[actual_row][actual_column]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let mut table = vec![vec![-1; 7000]; 7000];
        table[0][0] = 20151125;
        assert_eq!(get_code_at(&mut table, 4, 2), 32451966);
    }
}
