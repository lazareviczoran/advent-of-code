pub fn run() {
    let triangles = read("input.txt");
    println!(
        "part1 solution: {}",
        triangles.iter().filter(|t| t.is_valid()).count()
    );

    let triangles = read2("input.txt");
    println!(
        "part2 solution: {}",
        triangles.iter().filter(|t| t.is_valid()).count()
    );
}

#[derive(Debug)]
struct Triangle {
    a: usize,
    b: usize,
    c: usize,
}
impl Triangle {
    pub fn new(sides: &[usize]) -> Self {
        Self {
            a: sides[0],
            b: sides[1],
            c: sides[2],
        }
    }

    pub fn is_valid(&self) -> bool {
        self.a + self.b > self.c && self.a + self.c > self.b && self.c + self.b > self.a
    }
}

fn read(filename: &str) -> Vec<Triangle> {
    utils::read_to_string_in_module!(filename)
        .split_terminator('\n')
        .map(|s| {
            let sides = s
                .split_terminator(' ')
                .filter_map(|side| side.parse().ok())
                .collect::<Vec<_>>();
            Triangle::new(&sides)
        })
        .collect()
}

fn read2(filename: &str) -> Vec<Triangle> {
    let (col1, col2, col3) = utils::read_to_string_in_module!(filename)
        .split_terminator('\n')
        .fold((Vec::new(), Vec::new(), Vec::new()), |mut acc, s| {
            let row = s
                .split_terminator(' ')
                .filter_map(|side| side.parse().ok())
                .collect::<Vec<_>>();
            acc.0.push(row[0]);
            acc.1.push(row[1]);
            acc.2.push(row[2]);
            acc
        });
    let mut arr = Vec::new();
    arr.extend(&col1);
    arr.extend(&col2);
    arr.extend(&col3);
    arr.chunks_exact(3).map(Triangle::new).collect()
}
