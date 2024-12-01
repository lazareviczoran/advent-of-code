use regex::Regex;
use utils::read_to_string_in_module;

pub fn run() {
    // let points = read_input("test-input.txt");
    let points = read_input("input.txt");

    let (message, time_spent) = find_message(&points);
    println!("Day 10: The Stars Align part1 solution\n{}", message);

    println!("Day 10: The Stars Align part2 solution\n {}", time_spent);
}

fn find_message(init_points: &[Point]) -> (String, usize) {
    let mut points = init_points.to_vec();
    let mut res = init_points.to_vec();
    let (min_x, max_x, min_y, max_y) = find_edge_points(&points);
    let mut min_diff_x = max_x - min_x;
    let mut min_diff_y = max_y - min_y;
    let mut time = 0;
    loop {
        move_points(&mut points);
        let (min_x, max_x, min_y, max_y) = find_edge_points(&points);
        let diff_x = max_x - min_x;
        let diff_y = max_y - min_y;
        if diff_x > min_diff_x && diff_y > min_diff_y {
            break;
        }
        if diff_x < min_diff_x {
            min_diff_x = diff_x;
        }
        if diff_y < min_diff_y {
            min_diff_y = diff_y;
        }
        res = points.clone();
        time += 1;
    }

    (prepare_message(&res), time)
}

fn move_points(points: &mut [Point]) {
    for p in points.iter_mut() {
        p.pos_x += p.vel_x;
        p.pos_y += p.vel_y;
    }
}

fn prepare_message(points: &[Point]) -> String {
    let (min_x, max_x, min_y, max_y) = find_edge_points(points);
    let mut fields = vec![
        vec![' '; (max_y - min_y).unsigned_abs() as usize + 1];
        (max_x - min_x).unsigned_abs() as usize + 1
    ];

    for p in points {
        fields[(p.pos_x - min_x) as usize][(p.pos_y - min_y) as usize] = '#';
    }

    let mut res = String::new();
    for y in 0..fields[0].len() {
        for row in &fields {
            res.push(row[y]);
        }
        res.push('\n');
    }
    res
}

fn find_edge_points(points: &[Point]) -> (i32, i32, i32, i32) {
    let mut min_x = points[0].pos_x;
    let mut max_x = points[0].pos_x;
    let mut min_y = points[0].pos_y;
    let mut max_y = points[0].pos_y;
    for p in points.iter().skip(1) {
        if p.pos_x < min_x {
            min_x = p.pos_x;
        }
        if p.pos_x > max_x {
            max_x = p.pos_x;
        }
        if p.pos_y < min_y {
            min_y = p.pos_y;
        }
        if p.pos_y > max_y {
            max_y = p.pos_y;
        }
    }
    (min_x, max_x, min_y, max_y)
}

#[derive(Debug, Clone, Copy)]
struct Point {
    pos_x: i32,
    pos_y: i32,
    vel_x: i32,
    vel_y: i32,
}
impl Point {
    pub fn new(pos_x: i32, pos_y: i32, vel_x: i32, vel_y: i32) -> Point {
        Point {
            pos_x,
            pos_y,
            vel_x,
            vel_y,
        }
    }
}

fn read_input(filename: &str) -> Vec<Point> {
    let re =
        Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>").unwrap();

    read_to_string_in_module!(filename)
        .split_terminator('\n')
        .map(|s| {
            let caps = re.captures(s).unwrap();
            Point::new(
                caps[1].parse::<i32>().unwrap(),
                caps[2].parse::<i32>().unwrap(),
                caps[3].parse::<i32>().unwrap(),
                caps[4].parse::<i32>().unwrap(),
            )
        })
        .collect()
}
