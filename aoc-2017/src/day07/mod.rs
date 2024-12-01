use std::collections::{HashMap, HashSet};

use regex::Regex;

pub fn run() {
    let nodes = read("input.txt");
    let tree = build_tree(nodes);
    println!("part1 solution: {}", tree.name);
    println!("part2 solution: {}", find_sum(&tree).1);
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    val: usize,
    children_names: Vec<String>,
    children: HashMap<String, Node>,
}

fn find_sum(node: &Node) -> (usize, usize) {
    if node.children.is_empty() {
        return (node.val, 0);
    }
    let mut values = HashMap::new();
    for (_, n) in node.children.iter() {
        let (sum, correction) = find_sum(n);
        if correction > 0 {
            return (0, correction);
        }
        values.entry(sum).or_insert_with(Vec::new).push(n.val);
    }
    if values.len() > 1 {
        let (bad_sum, node_value) = values
            .iter()
            .filter_map(|(&w, app)| {
                if app.len() == 1 {
                    Some((w, app[0]))
                } else {
                    None
                }
            })
            .next()
            .unwrap();
        values.remove(&bad_sum);
        let (target_sum, _) = values.into_iter().next().unwrap();
        let corrected = node_value as i64 + target_sum as i64 - bad_sum as i64;
        return (0, corrected as usize);
    }
    (
        node.val + node.children.len() * values.keys().next().unwrap(),
        0,
    )
}

fn build_tree(mut nodes: HashMap<String, Node>) -> Node {
    let mut names = nodes.keys().cloned().collect::<HashSet<String>>();
    for node in nodes.values() {
        for child in &node.children_names {
            names.remove(child);
        }
    }
    let mut root = nodes.remove(names.iter().next().unwrap()).unwrap();
    build_tree_rec(&mut root, &mut nodes);
    root
}

fn build_tree_rec(node: &mut Node, nodes: &mut HashMap<String, Node>) {
    for child in &node.children_names {
        let mut child_node = nodes.remove(child).unwrap();
        build_tree_rec(&mut child_node, nodes);
        node.children.insert(child.clone(), child_node);
    }
}

fn read(filename: &str) -> HashMap<String, Node> {
    let re = Regex::new(r"(.+?)\s\((\d+)\)(\s->\s(.*))?").unwrap();
    utils::read_to_string_in_module!(filename)
        .lines()
        .filter_map(|s| {
            let caps = re
                .captures(s)?
                .iter()
                .filter_map(|s| {
                    let m = s?;
                    Some(m.as_str().to_string())
                })
                .collect::<Vec<_>>();
            let mut children_names = Vec::new();
            if caps.len() > 3 {
                children_names.extend(caps[4].split_terminator(", ").map(|p| p.to_string()));
            }
            Some((
                caps[1].clone(),
                Node {
                    name: caps[1].clone(),
                    val: caps[2].parse().ok()?,
                    children: HashMap::new(),
                    children_names,
                },
            ))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nodes = read("test-input.txt");
        let tree = build_tree(nodes);
        assert_eq!(tree.name, "tknk");
        assert_eq!(find_sum(&tree).1, 60);
    }
}
