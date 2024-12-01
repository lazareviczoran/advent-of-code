use utils::read_to_string_in_module;

pub fn run() {
    let content = read_to_string_in_module!("input.txt");

    println!("part1 solution: {}", decompress(&content).len());
    println!("part2 solution: {}", decompress_v2_length(&content));
}

fn decompress(content: &str) -> String {
    let mut s = String::new();
    let mut chars = content.chars();
    while let Some(ch) = chars.next() {
        match ch {
            '(' => {
                let mut marker_content = String::new();
                loop {
                    let marker_ch = chars.next().unwrap();
                    if marker_ch == ')' {
                        break;
                    }
                    marker_content.push(marker_ch);
                }
                let parts = marker_content
                    .split_terminator('x')
                    .filter_map(|s| s.parse().ok())
                    .collect::<Vec<usize>>();
                let mut data = String::new();
                for _ in 0..parts[0] {
                    data.push(chars.next().unwrap());
                }
                for _ in 0..parts[1] {
                    s.push_str(&data);
                }
            }
            ' ' => {}
            _ => s.push(ch),
        }
    }
    s
}

fn decompress_v2_length(content: &str) -> usize {
    let mut length = 0;
    let mut chars = content.chars();
    while let Some(ch) = chars.next() {
        match ch {
            '(' => {
                let mut marker_content = String::new();
                loop {
                    let marker_ch = chars.next().unwrap();
                    if marker_ch == ')' {
                        break;
                    }
                    marker_content.push(marker_ch);
                }
                let parts = marker_content
                    .split_terminator('x')
                    .filter_map(|s| s.parse().ok())
                    .collect::<Vec<usize>>();
                let mut data = String::new();
                for _ in 0..parts[0] {
                    data.push(chars.next().unwrap());
                }
                length += parts[1] * decompress_v2_length(&data);
            }
            ' ' => {}
            _ => length += 1,
        }
    }
    length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_tests() {
        assert_eq!(decompress("ADVENT"), "ADVENT");
        assert_eq!(decompress("A(1x5)BC"), "ABBBBBC");
        assert_eq!(decompress("(3x3)XYZ"), "XYZXYZXYZ");
        assert_eq!(decompress("A(2x2)BCD(2x2)EFG"), "ABCBCDEFEFG");
        assert_eq!(decompress("(6x1)(1x3)A"), "(1x3)A");
        assert_eq!(decompress("X(8x2)(3x3)ABCY"), "X(3x3)ABC(3x3)ABCY");
    }

    #[test]
    fn part2_tests() {
        assert_eq!(decompress_v2_length("(3x3)XYZ"), "XYZXYZXYZ".len());
        assert_eq!(
            decompress_v2_length("X(8x2)(3x3)ABCY"),
            "XABCABCABCABCABCABCY".len()
        );
        assert_eq!(
            decompress_v2_length("(27x12)(20x12)(13x14)(7x10)(1x12)A"),
            241920
        );
        assert_eq!(
            decompress_v2_length("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
            445
        );
    }
}
