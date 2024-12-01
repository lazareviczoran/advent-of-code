pub fn run() {
    let path = read("input.txt");
    let (final_dist, max_dist) = find_shortest_dist(&path);
    println!("part1 solution: {}", final_dist);
    println!("part2 solution: {}", max_dist);
}

fn find_shortest_dist(path: &[Dir]) -> (i32, i32) {
    let mut curr_pos = (0i32, 0i32);
    let mut max_dist = 0;
    for direction in path {
        match direction {
            Dir::North => curr_pos.1 -= 1,
            Dir::NorthEast => {
                curr_pos.0 += 1;
                curr_pos.1 -= 1;
            }
            Dir::NorthWest => curr_pos.0 -= 1,
            Dir::South => curr_pos.1 += 1,
            Dir::SouthEast => {
                curr_pos.0 += 1;
            }
            Dir::SouthWest => {
                curr_pos.0 -= 1;
                curr_pos.1 += 1;
            }
        }
        max_dist = max_dist.max(dist(curr_pos, (0, 0)));
    }
    (dist(curr_pos, (0, 0)), max_dist)
}

fn dist(a: (i32, i32), b: (i32, i32)) -> i32 {
    let (aq, ar) = a;
    let (bq, br) = b;
    ((aq - bq).abs() + (aq + ar - bq - br).abs() + (ar - br).abs()) / 2
}

enum Dir {
    NorthWest,
    North,
    NorthEast,
    SouthWest,
    South,
    SouthEast,
}

fn read(filename: &str) -> Vec<Dir> {
    utils::read_to_string_in_module!(filename)
        .split_terminator(',')
        .map(|s| match s {
            "n" => Dir::North,
            "ne" => Dir::NorthEast,
            "nw" => Dir::NorthWest,
            "s" => Dir::South,
            "se" => Dir::SouthEast,
            "sw" => Dir::SouthWest,
            _ => unreachable!(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(
            find_shortest_dist(&[Dir::NorthEast, Dir::NorthEast, Dir::NorthEast]).0,
            3
        );
        assert_eq!(
            find_shortest_dist(&[
                Dir::NorthEast,
                Dir::NorthEast,
                Dir::SouthWest,
                Dir::SouthWest
            ])
            .0,
            0
        );
        assert_eq!(
            find_shortest_dist(&[Dir::NorthEast, Dir::NorthEast, Dir::South, Dir::South]).0,
            2
        );
        assert_eq!(
            find_shortest_dist(&[
                Dir::SouthEast,
                Dir::SouthWest,
                Dir::SouthEast,
                Dir::SouthWest,
                Dir::SouthWest
            ])
            .0,
            3
        );
    }
}
