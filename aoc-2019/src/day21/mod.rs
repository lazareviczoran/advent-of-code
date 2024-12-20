use std::collections::HashMap;

use utils::read_to_string_in_module;

pub fn run() {
    let mut memory_map: HashMap<i64, i64> = HashMap::new();
    for (i, v) in read_to_string_in_module!("input.txt")
        .split_terminator(',')
        .enumerate()
    {
        let val = v.parse::<i64>().unwrap();
        memory_map.insert(i as i64, val);
    }

    println!(
        "Springdroid Adventure part1 Solution: {}",
        run_program(&mut memory_map.clone())
    );

    println!(
        "Springdroid Adventure part2 Solution: {}",
        run_program2(&mut memory_map)
    );
}

fn run_program2(memory: &mut HashMap<i64, i64>) -> i64 {
    let mut op_pos = 0;
    let mut rel_pos = 0;
    let mut input_pos = 0;
    let mut input = vec![];
    let output = compute(
        memory,
        &mut input,
        &mut op_pos,
        &mut rel_pos,
        &mut input_pos,
    );
    println!("{}", convert_to_string(output));

    let mut input = Vec::new();
    let commands = vec![
        "NOT A J", "NOT C T", "AND H T", "OR E T", "OR T J", "NOT B T", "AND H T", "OR T J",
        "AND D J", "RUN",
    ];
    let final_output = run_commands(
        memory,
        &mut op_pos,
        &mut rel_pos,
        &mut input_pos,
        &mut input,
        commands,
    );

    final_output[final_output.len() - 1]
}

fn run_program(memory: &mut HashMap<i64, i64>) -> i64 {
    let mut op_pos = 0;
    let mut rel_pos = 0;
    let mut input_pos = 0;
    let mut input = vec![];
    let output = compute(
        memory,
        &mut input,
        &mut op_pos,
        &mut rel_pos,
        &mut input_pos,
    );
    println!("{}", convert_to_string(output));

    let mut input = Vec::new();
    let commands = vec!["NOT A J", "NOT C T", "OR T J", "AND D J", "WALK"];
    let final_output = run_commands(
        memory,
        &mut op_pos,
        &mut rel_pos,
        &mut input_pos,
        &mut input,
        commands,
    );

    final_output[final_output.len() - 1]
}

fn run_commands(
    memory: &mut HashMap<i64, i64>,
    op_pos: &mut i64,
    rel_pos: &mut i64,
    input_pos: &mut usize,
    input: &mut Vec<i64>,
    cmds: Vec<&str>,
) -> Vec<i64> {
    let mut final_output = Vec::new();
    for cmd in cmds {
        println!("Running: {}", cmd);
        input.append(&mut convert_to_int_arr(cmd.to_string()));
        let output = compute(memory, input, op_pos, rel_pos, input_pos);

        final_output = output.clone();
        println!("{}", convert_to_string(output));
    }

    final_output
}

fn convert_to_string(array: Vec<i64>) -> String {
    let mut res = String::new();
    for el in array {
        res.push((el as u8) as char);
    }

    res
}

fn convert_to_int_arr(string: String) -> Vec<i64> {
    let mut res = Vec::new();
    for ch in string.chars() {
        res.push(ch as i64);
    }
    res.push('\n' as i64);

    res
}

fn compute(
    memory: &mut HashMap<i64, i64>,
    input: &mut [i64],
    op_pos: &mut i64,
    rel_pos: &mut i64,
    input_pos: &mut usize,
) -> Vec<i64> {
    let mut output = Vec::new();
    loop {
        let (op_code, param_modes) = extract_op_code_and_param_modes(memory, *op_pos);

        let move_by;
        match op_code {
            99 => break,
            1 => {
                let write_address =
                    get_write_address(memory, op_code, *op_pos, *rel_pos, param_modes[2]);
                let args = get_argument_values(memory, *op_pos, *rel_pos, param_modes);
                memory.insert(write_address, args[0] + args[1]);
                move_by = 4;
            }
            2 => {
                let write_address =
                    get_write_address(memory, op_code, *op_pos, *rel_pos, param_modes[2]);
                let args = get_argument_values(memory, *op_pos, *rel_pos, param_modes);
                memory.insert(write_address, args[0] * args[1]);
                move_by = 4;
            }
            3 => {
                let write_address =
                    get_write_address(memory, op_code, *op_pos, *rel_pos, param_modes[0]);
                if *input_pos == input.len() {
                    return output;
                }
                memory.insert(write_address, input[*input_pos]);
                *input_pos += 1;
                move_by = 2;
            }
            4 => {
                let args = get_argument_values(memory, *op_pos, *rel_pos, param_modes);
                output.push(args[0]);
                move_by = 2;
            }
            5 => {
                let args = get_argument_values(memory, *op_pos, *rel_pos, param_modes);
                if args[0] > 0 {
                    *op_pos = args[1];
                    continue;
                }
                move_by = 3;
            }
            6 => {
                let args = get_argument_values(memory, *op_pos, *rel_pos, param_modes);
                if args[0] == 0 {
                    *op_pos = args[1];
                    continue;
                }
                move_by = 3;
            }
            7 => {
                let write_address =
                    get_write_address(memory, op_code, *op_pos, *rel_pos, param_modes[2]);
                let args = get_argument_values(memory, *op_pos, *rel_pos, param_modes);
                if args[0] < args[1] {
                    memory.insert(write_address, 1);
                } else {
                    memory.insert(write_address, 0);
                }
                move_by = 4;
            }
            8 => {
                let write_address =
                    get_write_address(memory, op_code, *op_pos, *rel_pos, param_modes[2]);
                let args = get_argument_values(memory, *op_pos, *rel_pos, param_modes);
                if args[0] == args[1] {
                    memory.insert(write_address, 1);
                } else {
                    memory.insert(write_address, 0);
                }
                move_by = 4;
            }
            9 => {
                let args = get_argument_values(memory, *op_pos, *rel_pos, param_modes);
                *rel_pos += args[0];
                move_by = 2;
            }
            _ => panic!("Something went wrong: {}", op_code),
        }
        *op_pos += move_by;
    }
    output
}

fn get_value(memory: &mut HashMap<i64, i64>, key: i64) -> i64 {
    if let Some(value) = memory.get(&key) {
        *value
    } else {
        let value = 0;
        memory.insert(key, value);
        value
    }
}

fn get_argument_values(
    memory: &mut HashMap<i64, i64>,
    op_position: i64,
    rel_position: i64,
    param_modes: Vec<i64>,
) -> Vec<i64> {
    let mut args = Vec::new();
    for (i, &mode) in param_modes.iter().enumerate() {
        match mode {
            0 => {
                let pos = get_value(memory, op_position + (i as i64) + 1);
                args.push(get_value(memory, pos));
            }
            1 => {
                args.push(get_value(memory, op_position + (i as i64) + 1));
            }
            2 => {
                let pos = rel_position + get_value(memory, op_position + (i as i64) + 1);
                args.push(get_value(memory, pos));
            }
            _ => panic!("Unexpected param mode"),
        }
    }
    args
}

fn get_write_address(
    memory: &mut HashMap<i64, i64>,
    op_code: i64,
    op_position: i64,
    rel_position: i64,
    param_mode: i64,
) -> i64 {
    let mut offset = 3;
    if op_code == 3 {
        offset = 1;
    }
    match param_mode {
        0 => get_value(memory, op_position + offset),
        2 => rel_position + get_value(memory, op_position + offset),
        _ => panic!("Unexpected param mode"),
    }
}

fn extract_op_code_and_param_modes(memory: &mut HashMap<i64, i64>, pos: i64) -> (i64, Vec<i64>) {
    let val = get_value(memory, pos);
    let op_code = val % 100;
    let mut modes = Vec::new();
    let mut modes_digits = val / 100;
    let param_num = match op_code {
        1 | 2 | 7 | 8 => 3,
        5 | 6 => 2,
        3 | 4 | 9 => 1,
        99 => 0,
        _ => panic!("Invalid op code {}", op_code),
    };
    for _ in 0..param_num {
        modes.push(modes_digits % 10);
        modes_digits /= 10;
    }
    (op_code, modes)
}
