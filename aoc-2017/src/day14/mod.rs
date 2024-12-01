use crate::day10::knot_hash;
use std::collections::HashSet;

pub fn run() {
    let input = "ffayrhll";
    let grid = build_grid(input);
    println!("part1 solution: {}", count_used(&grid));
    println!("part2 solution: {}", count_regions(&grid));
}

fn count_used(grid: &[Vec<char>]) -> usize {
    grid.iter()
        .map(|row| row.iter().filter(|&&ch| ch == '#').count())
        .sum()
}

fn count_regions(grid: &[Vec<char>]) -> usize {
    let mut visited = HashSet::new();

    let mut region = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if ch == &'#' && !visited.contains(&(x as i32, y as i32)) {
                mark_region(grid, (x as i32, y as i32), &mut visited);
                region += 1;
            }
        }
    }
    region
}

fn mark_region(grid: &[Vec<char>], start_pos: (i32, i32), visited: &mut HashSet<(i32, i32)>) {
    let mut q = vec![start_pos];
    while let Some((x, y)) = q.pop() {
        visited.insert((x, y));

        [(-1, 0), (1, 0), (0, -1), (0, 1)].iter().for_each(|diff| {
            let (new_x, new_y) = (x + diff.0, y + diff.1);
            if (0..grid[y as usize].len() as i32).contains(&new_x)
                && (0..grid.len() as i32).contains(&new_y)
                && grid[new_y as usize][new_x as usize] == '#'
                && !visited.contains(&(new_x, new_y))
            {
                q.push((new_x, new_y));
            }
        });
    }
}

fn build_grid(input: &str) -> Vec<Vec<char>> {
    (0..128)
        .map(|i| {
            build_row(&knot_hash(&format!("{}-{}", input, i)))
                .iter()
                .map(|ch| match *ch {
                    '0' => '.',
                    '1' => '#',
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn build_row(text: &str) -> Vec<char> {
    text.chars()
        .filter_map(|ch| u8::from_str_radix(&format!("{}", ch), 16).ok())
        .fold(Vec::new(), |mut acc, value| {
            acc.extend(format!("{:04b}", value).chars());
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let row = build_row("a0c2017a0e66e6e86e3812dcb672a272")
            .iter()
            .collect::<String>();
        assert!(row.starts_with("1010000011000010000000010111"));

        let grid = build_grid("flqrgnkx");
        let subgrid: Vec<Vec<char>> = grid
            .iter()
            .take(8)
            .map(|row| row.iter().take(8).cloned().collect())
            .collect();

        assert_eq!(
            subgrid,
            &[
                &['#', '#', '.', '#', '.', '#', '.', '.',],
                &['.', '#', '.', '#', '.', '#', '.', '#',],
                &['.', '.', '.', '.', '#', '.', '#', '.',],
                &['#', '.', '#', '.', '#', '#', '.', '#',],
                &['.', '#', '#', '.', '#', '.', '.', '.',],
                &['#', '#', '.', '.', '#', '.', '.', '#',],
                &['.', '#', '.', '.', '.', '#', '.', '.',],
                &['#', '#', '.', '#', '.', '#', '#', '.',]
            ]
        );
        assert_eq!(count_used(&grid), 8108);
        assert_eq!(count_regions(&grid), 1242);
    }
}
