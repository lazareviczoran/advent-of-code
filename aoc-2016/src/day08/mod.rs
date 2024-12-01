use std::collections::VecDeque;

pub fn run() {
    let rects = read("input.txt");
    let mut screen = Screen::new(50, 6);
    screen.apply_rects(&rects);

    println!(
        "part1 solution: {:?}",
        screen
            .pixels
            .iter()
            .map(|row| row.iter().filter(|&&c| c).count())
            .sum::<usize>()
    );

    screen.print_screen()
}

#[derive(Debug)]
struct Screen {
    pixels: Vec<VecDeque<bool>>,
}
impl Screen {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            pixels: vec![VecDeque::from(vec![false; x]); y],
        }
    }

    pub fn apply_rects(&mut self, rects: &[Rect]) {
        for r in rects {
            self.apply_rect(r);
        }
    }

    pub fn apply_rect(&mut self, rect: &Rect) {
        for y in 0..rect.dim.1 {
            for x in 0..rect.dim.0 {
                self.pixels[y][x] = true;
            }
        }
        for rot in &rect.rotations {
            match rot.side {
                Side::X(x) => {
                    let actual_step = rot.step_size % self.pixels.len();
                    if actual_step > 0 {
                        let old_values = self.pixels.iter().map(|row| row[x]).collect::<Vec<_>>();
                        for (y, old_val) in old_values.iter().enumerate() {
                            let target_idx = (y + actual_step) % self.pixels.len();
                            self.pixels[target_idx][x] = *old_val;
                        }
                    }
                }
                Side::Y(i) => self.pixels[i].rotate_right(rot.step_size),
            }
        }
    }

    fn print_screen(&self) {
        println!("\n\n");
        let s = self
            .pixels
            .iter()
            .map(|row| {
                let mut s = row
                    .iter()
                    .map(|&is_on| if is_on { '#' } else { ' ' })
                    .collect::<String>();
                s.push('\n');
                s
            })
            .collect::<String>();
        println!("{}", s);
    }
}

#[derive(Debug)]
struct Rect {
    dim: (usize, usize),
    rotations: Vec<Rotation>,
}

#[derive(Debug)]
struct Rotation {
    side: Side,
    step_size: usize,
}

#[derive(Debug)]
enum Side {
    X(usize),
    Y(usize),
}

fn read(filename: &str) -> Vec<Rect> {
    utils::read_to_string_in_module!(filename)
        .split_terminator("rect ")
        .skip(1)
        .map(|s| {
            let lines = s.lines().collect::<Vec<_>>();
            let dim = lines[0]
                .split_terminator('x')
                .filter_map(|d| d.parse().ok())
                .collect::<Vec<usize>>();
            let rotations = lines
                .iter()
                .skip(1)
                .filter_map(|l| {
                    let parts = l.split_terminator(' ').collect::<Vec<_>>();
                    let side = if parts[2].starts_with('x') {
                        Side::X(parts[2].split_terminator('=').next_back()?.parse().ok()?)
                    } else {
                        Side::Y(parts[2].split_terminator('=').next_back()?.parse().ok()?)
                    };
                    Some(Rotation {
                        side,
                        step_size: parts[4].parse().ok()?,
                    })
                })
                .collect::<Vec<_>>();
            Rect {
                dim: (dim[0], dim[1]),
                rotations,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_tests() {
        let mut screen = Screen::new(7, 3);
        let rect = Rect {
            dim: (3, 2),
            rotations: vec![
                Rotation {
                    side: Side::X(1),
                    step_size: 1,
                },
                Rotation {
                    side: Side::Y(0),
                    step_size: 4,
                },
                Rotation {
                    side: Side::X(1),
                    step_size: 1,
                },
            ],
        };
        screen.apply_rect(&rect);
    }
}
