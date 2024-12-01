use regex::Regex;

pub fn run() {
    let contents = utils::read_to_string_in_module!("input.txt");

    let commands: Vec<&str> = contents.split_terminator('\n').collect();
    let commands_list = convert_to_structured_commands(&commands);
    let mut grid = vec![vec![false; 1000]; 1000];

    utils::run_solution!(
        || run_instuctions_and_count_lit_lights(&mut grid, &commands_list),
        "part1"
    );

    let mut grid_v2 = vec![vec![0; 1000]; 1000];
    utils::run_solution!(
        || run_instuctions_and_count_lit_lights2(&mut grid_v2, &commands_list),
        "part2"
    );
}

fn run_instuctions_and_count_lit_lights2(
    grid: &mut [Vec<usize>],
    commands_list: &Vec<Command>,
) -> usize {
    for cmd in commands_list {
        apply_command2(grid, cmd);
    }
    count_total_brightness(grid)
}

fn apply_command2(grid: &mut [Vec<usize>], cmd: &Command) {
    for y in cmd.from.y..=cmd.to.y {
        for row in grid.iter_mut().take(cmd.to.x + 1).skip(cmd.from.x) {
            match cmd.cmd_type {
                CommandType::Toggle => {
                    row[y] += 2;
                }
                CommandType::TurnOff => {
                    if row[y] > 0 {
                        row[y] -= 1;
                    }
                }
                CommandType::TurnOn => {
                    row[y] += 1;
                }
            }
        }
    }
}

fn run_instuctions_and_count_lit_lights(
    grid: &mut [Vec<bool>],
    commands_list: &Vec<Command>,
) -> usize {
    for cmd in commands_list {
        apply_command(grid, cmd);
    }
    count_lit_lights(grid)
}

fn apply_command(grid: &mut [Vec<bool>], cmd: &Command) {
    for y in cmd.from.y..=cmd.to.y {
        for row in grid.iter_mut().take(cmd.to.x + 1).skip(cmd.from.x) {
            match cmd.cmd_type {
                CommandType::Toggle => {
                    row[y] = !row[y];
                }
                CommandType::TurnOff => {
                    row[y] = false;
                }
                CommandType::TurnOn => {
                    row[y] = true;
                }
            }
        }
    }
}

fn convert_to_structured_commands(strings: &Vec<&str>) -> Vec<Command> {
    let mut commands_list = Vec::new();
    let re = Regex::new(r"(\d+),(\d+) through (\d+),(\d+)").unwrap();
    for s in strings {
        let captures = re.captures(s).unwrap();
        let mut cmd_type = CommandType::Toggle;
        if s.starts_with("turn on") {
            cmd_type = CommandType::TurnOn;
        } else if s.starts_with("turn off") {
            cmd_type = CommandType::TurnOff;
        }
        commands_list.push(Command::new(
            cmd_type,
            Position::new(
                captures[1].parse::<usize>().unwrap(),
                captures[2].parse::<usize>().unwrap(),
            ),
            Position::new(
                captures[3].parse::<usize>().unwrap(),
                captures[4].parse::<usize>().unwrap(),
            ),
        ));
    }
    commands_list
}

enum CommandType {
    Toggle,
    TurnOff,
    TurnOn,
}

struct Position {
    x: usize,
    y: usize,
}
impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
}

struct Command {
    cmd_type: CommandType,
    from: Position,
    to: Position,
}

impl Command {
    pub fn new(cmd_type: CommandType, from: Position, to: Position) -> Command {
        Command { cmd_type, from, to }
    }
}

fn count_total_brightness(map: &mut [Vec<usize>]) -> usize {
    map.iter().map(|row| row.iter().sum::<usize>()).sum()
}

fn count_lit_lights(map: &mut [Vec<bool>]) -> usize {
    map.iter()
        .map(|row| row.iter().filter(|&&x| x).count())
        .sum()
}
