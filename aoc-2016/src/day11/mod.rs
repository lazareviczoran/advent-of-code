pub fn run() {
    // The first floor contains a polonium generator, a thulium generator, a thulium-compatible microchip,
    // a promethium generator, a ruthenium generator, a ruthenium-compatible microchip, a cobalt generator,
    // and a cobalt-compatible microchip.
    // The second floor contains a polonium-compatible microchip and a promethium-compatible microchip.
    // The third floor contains nothing relevant.
    // The fourth floor contains nothing relevant.
    let mut floors = vec![
        vec![
            ("polonium".into(), "generator".into()),
            ("thulium".into(), "generator".into()),
            ("thulium".into(), "microchip".into()),
            ("promethium".into(), "generator".into()),
            ("ruthenium".into(), "generator".into()),
            ("ruthenium".into(), "microchip".into()),
            ("cobalt".into(), "generator".into()),
            ("cobalt".into(), "microchip".into()),
        ],
        vec![
            ("polonium".into(), "microchip".into()),
            ("promethium".into(), "microchip".into()),
        ],
        vec![],
        vec![],
    ];

    println!("part1 solution: {}", find_min_steps(&mut floors));

    let mut floors = vec![
        vec![
            ("polonium".into(), "generator".into()),
            ("thulium".into(), "generator".into()),
            ("thulium".into(), "microchip".into()),
            ("promethium".into(), "generator".into()),
            ("ruthenium".into(), "generator".into()),
            ("ruthenium".into(), "microchip".into()),
            ("cobalt".into(), "generator".into()),
            ("cobalt".into(), "microchip".into()),
            ("elerium".into(), "generator".into()),
            ("elerium".into(), "microchip".into()),
            ("dilithium".into(), "generator".into()),
            ("dilithium".into(), "microchip".into()),
        ],
        vec![
            ("polonium".into(), "microchip".into()),
            ("promethium".into(), "microchip".into()),
        ],
        vec![],
        vec![],
    ];
    println!("part2 solution: {}", find_min_steps(&mut floors));
}

fn find_min_steps(floors: &mut [Vec<(String, String)>]) -> usize {
    let items_count = floors.iter().map(|floor| floor.len()).sum::<usize>();
    let mut steps = 0;
    let mut elevator = 0;
    while floors[3].len() < items_count {
        move_up(floors, &mut elevator, &mut steps);
        if !floors[elevator - 1].is_empty() {
            move_down(floors, &mut elevator, &mut steps);
        }
    }
    steps
}

fn move_up(floors: &mut [Vec<(String, String)>], elevator: &mut usize, steps: &mut usize) {
    let bag = next_best_candidate_up(floors, *elevator);

    floors[*elevator].retain(|item| !bag.contains(item));

    *elevator += 1;
    *steps += 1;

    floors[*elevator].extend(bag);
}

fn move_down(floors: &mut [Vec<(String, String)>], elevator: &mut usize, steps: &mut usize) {
    let bag = next_best_candidate_down(floors, *elevator);

    floors[*elevator].retain(|item| !bag.contains(item));

    *elevator -= 1;
    *steps += 1;

    floors[*elevator].extend(bag);
}

fn next_best_candidate_up(
    floors: &mut [Vec<(String, String)>],
    elevator: usize,
) -> Vec<(String, String)> {
    let (generators, chips): (Vec<_>, Vec<_>) = floors[elevator]
        .iter()
        .cloned()
        .partition(|item| item.1 == "generator");
    let (shorter, longer) = if generators.len() > chips.len() {
        (chips, generators)
    } else {
        (generators, chips)
    };
    match longer.len() - shorter.len() {
        0 => {
            let item1 = longer[0].clone();
            let item2 = shorter.iter().find(|c| c.0 == item1.0).unwrap().clone();
            vec![item1, item2]
        }
        1 => longer
            .iter()
            .filter(|g| shorter.iter().all(|c| g.0 != c.0))
            .chain(
                longer
                    .iter()
                    .filter(|g| shorter.iter().any(|c| c.0 == g.0))
                    .take(1),
            )
            .cloned()
            .collect(),
        _ => longer
            .iter()
            .filter(|g| shorter.iter().all(|c| g.0 != c.0))
            .take(2)
            .cloned()
            .collect(),
    }
}

fn next_best_candidate_down(
    floors: &mut [Vec<(String, String)>],
    elevator: usize,
) -> Vec<(String, String)> {
    let (generators, chips): (Vec<_>, Vec<_>) = floors[elevator]
        .iter()
        .cloned()
        .partition(|item| item.1 == "generator");
    let (shorter, longer) = if generators.len() > chips.len() {
        (chips, generators)
    } else {
        (generators, chips)
    };
    if shorter.is_empty() {
        return longer.iter().take(1).cloned().collect();
    } else {
        match longer.len() - shorter.len() {
            0 => longer.iter().take(1).cloned().collect(),
            _ => vec![longer
                .iter()
                .find(|g| shorter.iter().all(|c| g.0 != c.0))
                .unwrap()
                .clone()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        // The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
        // The second floor contains a hydrogen generator.
        // The third floor contains a lithium generator.
        // The fourth floor contains nothing relevant.

        let mut floors = vec![
            vec![
                ("hydrogen".into(), "microchip".into()),
                ("lithium".into(), "microchip".into()),
            ],
            vec![("hydrogen".into(), "generator".into())],
            vec![("lithium".into(), "generator".into())],
            vec![],
        ];
        assert_eq!(find_min_steps(&mut floors), 9);
    }
}
