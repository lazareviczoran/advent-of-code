pub fn run() {
    let fields = read_input("input.txt", 100);

    utils::run_solution!(|| animate(&fields, 100), "part1");

    utils::run_solution!(|| animate_v2(&fields, 100), "part2");
}

fn count_turned_on_lights(fields: &[Vec<char>]) -> usize {
    fields
        .iter()
        .map(|row| row.iter().filter(|&&c| is_turned_on(c)).count())
        .sum()
}

fn animate_v2(fields: &[Vec<char>], iterations: usize) -> usize {
    let mut curr_fields = fields.to_vec();
    turn_on_corner_lights(&mut curr_fields);
    for _ in 0..iterations {
        let next_fields = apply_animation_step(&curr_fields);
        curr_fields = next_fields;
        turn_on_corner_lights(&mut curr_fields);
    }

    count_turned_on_lights(&curr_fields)
}

fn animate(fields: &[Vec<char>], iterations: usize) -> usize {
    let mut curr_fields = fields.to_vec();
    for _ in 0..iterations {
        let next_fields = apply_animation_step(&curr_fields);
        curr_fields = next_fields;
    }

    count_turned_on_lights(&curr_fields)
}

fn apply_animation_step(fields: &[Vec<char>]) -> Vec<Vec<char>> {
    let on_char = '#';
    let off_char = '.';
    let w = fields.len();
    let h = fields[0].len();
    let mut res = vec![vec![' '; h]; w];
    for j in 0..h {
        for i in 0..w {
            let curr_val = fields[i][j];
            let (on, _) = check_neighbors(fields, (i, j));
            if curr_val == on_char && on != 2 && on != 3 {
                res[i][j] = off_char;
            } else if curr_val == off_char && on == 3 {
                res[i][j] = on_char;
            } else {
                res[i][j] = fields[i][j];
            }
        }
    }

    res
}

fn check_neighbors(fields: &[Vec<char>], pos: (usize, usize)) -> (usize, usize) {
    let (x, y) = pos;
    let mut from_x = x;
    let mut to_x = x;
    let mut from_y = y;
    let mut to_y = y;
    if x == 0 {
        to_x += 1;
    } else if x == fields.len() - 1 {
        from_x -= 1;
    } else {
        from_x -= 1;
        to_x += 1;
    }
    if y == 0 {
        to_y += 1;
    } else if y == fields[0].len() - 1 {
        from_y -= 1;
    } else {
        from_y -= 1;
        to_y += 1;
    }
    let mut count_on = 0;
    for j in from_y..=to_y {
        for (i, row) in fields.iter().enumerate().take(to_x + 1).skip(from_x) {
            if i == x && j == y {
                continue;
            }
            if is_turned_on(row[j]) {
                count_on += 1;
            }
        }
    }

    (count_on, 8 - count_on)
}

fn is_turned_on(light: char) -> bool {
    match light {
        '#' => true,
        '.' => false,
        _ => panic!("Unexpeced char: {}", light),
    }
}

fn turn_on_corner_lights(fields: &mut [Vec<char>]) {
    let w = fields.len() - 1;
    let h = fields[0].len() - 1;
    for (x, y) in [(0, 0), (w, 0), (0, h), (w, h)].iter() {
        fields[*x][*y] = '#';
    }
}

#[allow(unused)]
fn print_fields(fields: &[Vec<char>]) {
    let w = fields.len();
    let h = fields[0].len();
    let mut string = String::new();
    for row in fields {
        for ch in row {
            string.push(*ch);
        }
        string.push('\n');
    }

    println!("{}", string);
}

fn read_input(filename: &str, size: usize) -> Vec<Vec<char>> {
    let contents = utils::read_to_string_in_module!(filename);
    let mut fields = vec![vec![' '; size]; size];
    for (curr_y, x) in contents.split_terminator('\n').enumerate() {
        for (curr_x, ch) in x.chars().enumerate() {
            fields[curr_x][curr_y] = ch;
        }
    }

    fields
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_input1() {
        let fields = read_input("test-input.txt", 6);
        assert_eq!(animate(&fields, 4), 4);
    }

    #[test]
    fn part2_input1() {
        let fields = read_input("test-input.txt", 6);
        assert_eq!(animate_v2(&fields, 5), 17);
    }
}
