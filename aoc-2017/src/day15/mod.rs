pub fn run() {
    // Generator A starts with 512
    // Generator B starts with 191
    let mut gen_a = Generator::new(512, 16807, 4);
    let mut gen_b = Generator::new(191, 48271, 8);
    println!("part1 solution: {}", judge_count(&mut gen_a, &mut gen_b));
    gen_a.reset();
    gen_b.reset();
    println!(
        "part2 solution: {}",
        judge_count_with_criteria(&mut gen_a, &mut gen_b)
    );
}

fn judge_count(gen_a: &mut Generator, gen_b: &mut Generator) -> usize {
    (0..40_000_000)
        .filter(|_| gen_a.next() as u16 == gen_b.next() as u16)
        .count()
}

fn judge_count_with_criteria(gen_a: &mut Generator, gen_b: &mut Generator) -> usize {
    let mut count = 0;
    for _ in 0..5_000_000 {
        if gen_a.next_with_criteria() as u16 == gen_b.next_with_criteria() as u16 {
            count += 1;
        }
    }
    count
}

struct Generator {
    curr_value: usize,
    init_value: usize,
    criteria: usize,
    factor: usize,
}
impl Generator {
    fn new(curr_value: usize, factor: usize, criteria: usize) -> Self {
        Self {
            curr_value,
            init_value: curr_value,
            criteria,
            factor,
        }
    }

    fn next(&mut self) -> usize {
        let value = (self.curr_value * self.factor) % 2147483647;
        self.curr_value = value;
        value
    }

    fn next_with_criteria(&mut self) -> usize {
        loop {
            let value = (self.curr_value * self.factor) % 2147483647;
            self.curr_value = value;
            if value % self.criteria == 0 {
                return value;
            }
        }
    }

    fn reset(&mut self) {
        self.curr_value = self.init_value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let mut gen_a = Generator::new(65, 16807, 4);
        let mut gen_b = Generator::new(8921, 48271, 8);
        assert_eq!(gen_a.next(), 1092455);
        assert_eq!(gen_a.next(), 1181022009);
        assert_eq!(gen_a.next(), 245556042);
        assert_eq!(gen_a.next(), 1744312007);
        assert_eq!(gen_a.next(), 1352636452);

        assert_eq!(gen_b.next(), 430625591);
        assert_eq!(gen_b.next(), 1233683848);
        assert_eq!(gen_b.next(), 1431495498);
        assert_eq!(gen_b.next(), 137874439);
        assert_eq!(gen_b.next(), 285222916);

        gen_a.reset();
        gen_b.reset();
        assert_eq!(judge_count(&mut gen_a, &mut gen_b), 588);

        gen_a.reset();
        gen_b.reset();

        assert_eq!(gen_a.next_with_criteria(), 1352636452);
        assert_eq!(gen_a.next_with_criteria(), 1992081072);
        assert_eq!(gen_a.next_with_criteria(), 530830436);
        assert_eq!(gen_a.next_with_criteria(), 1980017072);
        assert_eq!(gen_a.next_with_criteria(), 740335192);

        assert_eq!(gen_b.next_with_criteria(), 1233683848);
        assert_eq!(gen_b.next_with_criteria(), 862516352);
        assert_eq!(gen_b.next_with_criteria(), 1159784568);
        assert_eq!(gen_b.next_with_criteria(), 1616057672);
        assert_eq!(gen_b.next_with_criteria(), 412269392);

        gen_a.reset();
        gen_b.reset();
        assert_eq!(judge_count_with_criteria(&mut gen_a, &mut gen_b), 309);
    }
}
