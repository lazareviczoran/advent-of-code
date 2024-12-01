use std::cmp::min;

pub fn run() {
    let presents = read_input("input.txt");

    utils::run_solution!(
        || presents
            .iter()
            .fold(0, |acc, p| acc + p.get_required_wrapping_paper_amount()),
        "part1"
    );

    utils::run_solution!(
        || presents
            .iter()
            .fold(0, |acc, p| acc + p.get_required_ribbon_amount()),
        "part2"
    );
}

fn read_input(file_path: &str) -> Vec<Prism> {
    let contents = utils::read_to_string_in_module!(file_path);
    contents
        .split_terminator('\n')
        .map(|v| {
            let dimensions = v
                .split_terminator('x')
                .map(|p| p.parse::<usize>().unwrap())
                .collect();
            Prism::new(dimensions)
        })
        .collect()
}

struct Prism {
    l: usize,
    w: usize,
    h: usize,
}
impl Prism {
    pub fn new(dimensions: Vec<usize>) -> Prism {
        Prism {
            l: dimensions[0],
            w: dimensions[1],
            h: dimensions[2],
        }
    }

    pub fn get_required_wrapping_paper_amount(&self) -> usize {
        let surface_areas = [self.l * self.w, self.w * self.h, self.h * self.l];
        let smallest_area = min(min(surface_areas[0], surface_areas[1]), surface_areas[2]);

        surface_areas.iter().fold(0, |acc, area| acc + 2 * area) + smallest_area
    }

    pub fn get_required_ribbon_amount(&self) -> usize {
        let mut dimensions = [self.l, self.w, self.h];
        dimensions.sort();

        2 * dimensions[0] + 2 * dimensions[1] + dimensions.iter().product::<usize>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_input1() {
        let present = Prism::new(vec![2, 3, 4]);
        assert_eq!(present.get_required_wrapping_paper_amount(), 58);
    }

    #[test]
    fn part1_input2() {
        let present = Prism::new(vec![1, 1, 10]);
        assert_eq!(present.get_required_wrapping_paper_amount(), 43);
    }

    #[test]
    fn part2_input1() {
        let present = Prism::new(vec![2, 3, 4]);
        assert_eq!(present.get_required_ribbon_amount(), 34);
    }

    #[test]
    fn part2_input2() {
        let present = Prism::new(vec![1, 1, 10]);
        assert_eq!(present.get_required_ribbon_amount(), 14);
    }
}
