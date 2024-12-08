use itertools::Itertools;
use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;

pub fn run() {
    let (rules, ordered, unordered) = read_input("input.txt");

    utils::run_solution!(|| sum_middle_pages_ordered(&ordered), "part1");
    utils::run_solution!(|| sum_middle_pages_unordered(&rules, &unordered), "part2");
}

fn sum_middle_pages_ordered(lists: &[Vec<usize>]) -> usize {
    lists.iter().map(|list| list.middle_item()).sum()
}

fn sum_middle_pages_unordered(
    rules: &FxHashMap<usize, FxHashSet<usize>>,
    lists: &[Vec<usize>],
) -> usize {
    lists
        .iter()
        .map(|list| {
            list.iter()
                .map(|&curr| {
                    (
                        curr,
                        rules
                            .get(&curr)
                            .map(|to| to.iter().filter(|to| list.contains(to)).count())
                            .unwrap_or(0),
                    )
                })
                .sorted_by_key(|a| a.1)
                .map(|a| a.0)
                .collect_vec()
                .middle_item()
        })
        .sum()
}

trait MiddleItem {
    fn middle_item(&self) -> usize;
}
impl MiddleItem for [usize] {
    fn middle_item(&self) -> usize {
        self[self.len() / 2]
    }
}

type Input = (
    FxHashMap<usize, FxHashSet<usize>>,
    Vec<Vec<usize>>,
    Vec<Vec<usize>>,
);
fn read_input(filename: &str) -> Input {
    let content = utils::read_to_string_in_module!(filename);
    let (rules, lists) = content.split_once("\n\n").expect("failed to split input");
    let rules = rules.lines().fold(
        FxHashMap::default(),
        |mut acc: FxHashMap<usize, FxHashSet<usize>>, line| {
            let (from, to) = line
                .split_terminator('|')
                .map(|a| a.parse().unwrap())
                .collect_tuple()
                .unwrap();
            acc.entry(from).or_default().insert(to);
            acc
        },
    );
    let (ordered, unordered): (Vec<_>, Vec<_>) = lists
        .lines()
        .map(|line| {
            line.split_terminator(',')
                .map(|x| x.parse().unwrap())
                .collect_vec()
        })
        .partition(|list| {
            list.windows(2).all(|window| {
                rules
                    .get(&window[0])
                    .and_then(|from| from.get(&window[1]))
                    .is_some()
            })
        });

    (rules, ordered, unordered)
}

#[cfg(test)]
mod tests {
    use super::read_input;
    use super::sum_middle_pages_ordered;
    use super::sum_middle_pages_unordered;

    #[test]
    fn p1() {
        let (_, lists, _) = read_input("test-input.txt");
        assert_eq!(sum_middle_pages_ordered(&lists), 143);
    }

    #[test]
    fn p2() {
        let (rules, _, lists) = read_input("test-input.txt");
        assert_eq!(sum_middle_pages_unordered(&rules, &lists), 123);
    }

    #[test]
    fn prod() {
        let (rules, ordered, unordered) = read_input("input.txt");
        assert_eq!(sum_middle_pages_ordered(&ordered), 5964);
        assert_eq!(sum_middle_pages_unordered(&rules, &unordered), 4719);
    }
}
