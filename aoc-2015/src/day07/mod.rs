use regex::Regex;
use std::collections::HashMap;

pub fn run() {
    let contents = utils::read_to_string_in_module!("input.txt");

    let instructions = contents.split_terminator('\n').collect();
    let mut map = build_connections(&instructions);
    let addr_a_value = utils::run_solution!(|| get_value(&mut map, "a"), "part1");

    map = build_connections(&instructions);
    let b_conn = map.get_mut("b").unwrap();
    b_conn.value = Some(addr_a_value);
    utils::run_solution!(|| get_value(&mut map, "a"), "part2");
}

#[derive(Clone, Debug)]
enum Operator {
    And,
    Or,
    LShift,
    RShift,
    Not,
    Assign,
}

#[derive(Clone, Debug)]
struct Connection {
    args: Vec<String>,
    operator: Operator,
    value: Option<u16>,
}

impl Connection {
    pub fn new(args: Vec<String>, operator: Operator) -> Connection {
        Connection {
            args,
            operator,
            value: None,
        }
    }
}

fn build_connections(instructions: &Vec<&str>) -> HashMap<String, Connection> {
    let mut map = HashMap::new();
    let dual_op_regex = Regex::new(r"(.+)\s(AND|OR|LSHIFT|RSHIFT)\s(.+)\s->\s(.+)").unwrap();
    let not_op_regex = Regex::new(r"NOT\s(.+)\s->\s(.+)").unwrap();
    let assign_op_regex = Regex::new(r"(.+)\s->\s(.+)").unwrap();

    for i in instructions {
        if i.contains("AND") {
            let captures = dual_op_regex.captures(i).unwrap();
            map.insert(
                captures[4].to_string(),
                Connection::new(
                    vec![captures[1].to_string(), captures[3].to_string()],
                    Operator::And,
                ),
            );
        } else if i.contains("OR") {
            let captures = dual_op_regex.captures(i).unwrap();
            map.insert(
                captures[4].to_string(),
                Connection::new(
                    vec![captures[1].to_string(), captures[3].to_string()],
                    Operator::Or,
                ),
            );
        } else if i.contains("LSHIFT") {
            let captures = dual_op_regex.captures(i).unwrap();
            map.insert(
                captures[4].to_string(),
                Connection::new(
                    vec![captures[1].to_string(), captures[3].to_string()],
                    Operator::LShift,
                ),
            );
        } else if i.contains("RSHIFT") {
            let captures = dual_op_regex.captures(i).unwrap();
            map.insert(
                captures[4].to_string(),
                Connection::new(
                    vec![captures[1].to_string(), captures[3].to_string()],
                    Operator::RShift,
                ),
            );
        } else if i.contains("NOT") {
            let captures = not_op_regex.captures(i).unwrap();
            map.insert(
                captures[2].to_string(),
                Connection::new(vec![captures[1].to_string()], Operator::Not),
            );
        } else {
            let captures = assign_op_regex.captures(i).unwrap();
            let parsed_val = captures[1].parse::<u16>();
            if parsed_val.is_ok() {
                let mut connection = Connection::new(vec![], Operator::Assign);
                connection.value = Some(parsed_val.unwrap());
                map.insert(captures[2].to_string(), connection);
            } else {
                map.insert(
                    captures[2].to_string(),
                    Connection::new(vec![captures[1].to_string()], Operator::Assign),
                );
            }
        }
    }
    map
}

fn get_value(map: &mut HashMap<String, Connection>, target_wire: &str) -> u16 {
    let mut connection = map.get_mut(target_wire).unwrap().clone();
    if let Some(val) = connection.value {
        return val;
    }
    let mut values = Vec::new();
    for conn in connection.args.clone() {
        let parsed_val = conn.parse::<u16>();
        if let Ok(val) = parsed_val {
            values.push(val);
        } else {
            values.push(get_value(map, &conn));
        }
    }
    let addr_value = match connection.operator {
        Operator::And => values[0] & values[1],
        Operator::Or => values[0] | values[1],
        Operator::LShift => values[0] << values[1],
        Operator::RShift => values[0] >> values[1],
        Operator::Not => !values[0],
        Operator::Assign => values[0],
    };
    connection.value = Some(addr_value);
    map.insert(target_wire.to_string(), connection);

    addr_value
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_input1() {
        let instructions = vec![
            "123 -> x",
            "456 -> y",
            "x AND y -> d",
            "x OR y -> e",
            "x LSHIFT 2 -> f",
            "y RSHIFT 2 -> g",
            "NOT x -> h",
            "NOT y -> i",
        ];
        let mut map = build_connections(&instructions);
        assert_eq!(get_value(&mut map, "d"), 72);
        assert_eq!(get_value(&mut map, "e"), 507);
        assert_eq!(get_value(&mut map, "f"), 492);
        assert_eq!(get_value(&mut map, "g"), 114);
        assert_eq!(get_value(&mut map, "h"), 65412);
        assert_eq!(get_value(&mut map, "i"), 65079);
        assert_eq!(get_value(&mut map, "x"), 123);
        assert_eq!(get_value(&mut map, "y"), 456);
    }
}
