pub fn run() {
    let disk_map = read_input("input.txt");

    utils::run_solution!(
        || compute_checksum(&compact(&expand_disk_map(&disk_map))),
        "part1"
    );
    utils::run_solution!(
        || compute_checksum(&compact_v2(&expand_disk_map(&disk_map))),
        "part2"
    );
}

fn compute_checksum(blocks: &[usize]) -> usize {
    blocks
        .iter()
        .enumerate()
        .filter(|(_, &c)| c != usize::MAX)
        .map(|(i, &ch)| i * ch)
        .sum()
}

fn compact(blocks: &[usize]) -> Vec<usize> {
    let mut blocks = blocks.to_vec();
    let (mut i, mut j) = (0, blocks.len() - 1);
    while i < j {
        if blocks[i] == usize::MAX {
            while blocks[j] == usize::MAX {
                j -= 1;
            }
            blocks.swap(i, j);
            j -= 1;
        }
        i += 1;
    }
    blocks
}

fn compact_v2(blocks: &[usize]) -> Vec<usize> {
    let mut blocks = blocks.to_vec();
    let mut j = (blocks.len() - 1) as isize;
    let mut first_gap = blocks.iter().position(|c| *c == usize::MAX).unwrap();

    while j > first_gap as isize {
        while blocks[j as usize] == usize::MAX {
            j -= 1;
        }
        let curr = blocks[j as usize];
        let mut block_size: usize = 0;

        while j > 0 && block_size <= j as usize && blocks[j as usize - block_size] == curr {
            block_size += 1;
        }
        let mut i = first_gap;
        while i < j as usize {
            let mut gap_size = 0;
            if blocks[i] == usize::MAX {
                let mut k = i;
                while k < blocks.len() && blocks[k] == usize::MAX {
                    gap_size += 1;
                    k += 1;
                }
            }
            if gap_size >= block_size {
                for k in 0..block_size {
                    blocks.swap(i + k, j as usize - k);
                }
                if i == first_gap {
                    first_gap += block_size;
                }
                break;
            }
            i += 1;
        }
        j -= block_size as isize;
    }
    blocks
}

fn expand_disk_map(disk_map: &str) -> Vec<usize> {
    let mut blocks = Vec::new();
    let mut curr = 0usize;
    for (i, ch) in disk_map.chars().enumerate() {
        ('0'..ch).for_each(|_| blocks.push(if i % 2 == 1 { usize::MAX } else { curr }));
        if i % 2 == 0 {
            curr += 1;
        }
    }
    blocks
}

fn read_input(filename: &str) -> String {
    utils::read_to_string_in_module!(filename)
}

#[cfg(test)]
mod tests {
    use crate::day09::compact;
    use crate::day09::compact_v2;
    use crate::day09::compute_checksum;
    use crate::day09::expand_disk_map;

    use super::read_input;

    #[test]
    fn p1() {
        let disk_map = read_input("test-input.txt");
        assert_eq!(
            compute_checksum(&compact(&expand_disk_map(&disk_map))),
            1928
        );

        let disk_map = read_input("test-input2.txt");
        assert_eq!(
            compute_checksum(&compact(&expand_disk_map(&disk_map))),
            30 + 14 + 16
        );
    }

    #[test]
    fn p2() {
        let disk_map = read_input("test-input.txt");
        assert_eq!(
            compute_checksum(&compact_v2(&expand_disk_map(&disk_map))),
            2858
        );
    }

    #[test]
    #[cfg(feature = "include-main-input")]
    fn prod() {
        use itertools::Itertools;
        let (pt1, pt2) = utils::read_to_string_in_module!("results.txt")
            .lines()
            .map(|line| line.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        let disk_map = read_input("input.txt");
        assert_eq!(compute_checksum(&compact(&expand_disk_map(&disk_map))), pt1);
        assert_eq!(
            compute_checksum(&compact_v2(&expand_disk_map(&disk_map))),
            pt2
        );
    }
}
