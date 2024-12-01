use std::collections::HashSet;

pub fn run() {
    let passphrases = read("input.txt");
    println!(
        "part1 solution: {}",
        passphrases.iter().filter(|pass| is_valid(pass)).count()
    );
    println!(
        "part2 solution: {}",
        passphrases.iter().filter(|pass| is_valid2(pass)).count()
    );
}

fn is_valid(passphrase: &[String]) -> bool {
    let mut unique = HashSet::new();
    for p in passphrase {
        if unique.contains(p) {
            return false;
        }
        unique.insert(p.clone());
    }
    true
}

fn is_valid2(passphrase: &[String]) -> bool {
    let mut unique = HashSet::new();
    for p in passphrase {
        let mut letters = [0; 26];
        for ch in p.chars() {
            letters[ch as usize - 'a' as usize] += 1;
        }
        if unique.contains(&letters) {
            return false;
        }
        unique.insert(letters);
    }
    true
}

fn read(filename: &str) -> Vec<Vec<String>> {
    utils::read_to_string_in_module!(filename)
        .lines()
        .map(|s| s.split_terminator(' ').map(|p| p.to_string()).collect())
        .collect()
}
