use std::collections::HashMap;

use regex::Regex;

pub fn run() {
    let particles = read("input.txt");
    println!(
        "part1 solution: {}",
        find_long_term_closest_to_origin(&particles)
    );
    println!(
        "part2 solution: {}",
        count_particles_after_collisions(particles)
    );
}

fn count_particles_after_collisions(mut particles: HashMap<usize, Particle>) -> usize {
    let mut positions = HashMap::new();
    let mut steps_since_collision = 0;
    while steps_since_collision < 100 {
        particles = particles
            .into_iter()
            .map(|(i, mut p)| {
                p.move_tick();
                positions.entry(p.position).or_insert_with(Vec::new).push(i);
                (i, p)
            })
            .collect();
        let to_remove = positions
            .values()
            .fold(Vec::new(), |mut acc: Vec<usize>, candidates| {
                if candidates.len() > 1 {
                    acc.extend(candidates.iter());
                }
                acc
            });
        if to_remove.is_empty() {
            steps_since_collision += 1;
        } else {
            for c in to_remove {
                particles.remove(&c);
            }
            steps_since_collision = 0;
        }
        positions.clear();
    }
    particles.len()
}

fn find_long_term_closest_to_origin(particles: &HashMap<usize, Particle>) -> usize {
    let mut accelerations = particles
        .iter()
        .map(|(i, p)| {
            (
                i,
                p.acceleration.0.pow(2) + p.acceleration.1.pow(2) + p.acceleration.2.pow(2),
                p.velocity.0.pow(2) + p.velocity.1.pow(2) + p.velocity.2.pow(2),
                p.position.0.abs() + p.position.1.abs() + p.position.2.abs(),
            )
        })
        .collect::<Vec<_>>();
    accelerations.sort_by(|a, b| a.1.cmp(&b.1).then(a.2.cmp(&b.2).then(b.3.cmp(&a.3))));
    accelerations
        .into_iter()
        .map(|(idx, ..)| *idx)
        .next()
        .unwrap()
}

struct Particle {
    position: (i64, i64, i64),
    velocity: (i64, i64, i64),
    acceleration: (i64, i64, i64),
}
impl Particle {
    fn move_tick(&mut self) {
        self.velocity = (
            self.velocity.0 + self.acceleration.0,
            self.velocity.1 + self.acceleration.1,
            self.velocity.2 + self.acceleration.2,
        );
        self.position = (
            self.position.0 + self.velocity.0,
            self.position.1 + self.velocity.1,
            self.position.2 + self.velocity.2,
        );
    }
}

fn read(filename: &str) -> HashMap<usize, Particle> {
    let re = Regex::new(r"(-?\d+)").unwrap();
    utils::read_to_string_in_module!(filename)
        .lines()
        .enumerate()
        .map(|(i, s)| {
            let caps = re
                .captures_iter(s)
                .filter_map(|m| m[0].parse().ok())
                .collect::<Vec<_>>();
            (
                i,
                Particle {
                    position: (caps[0], caps[1], caps[2]),
                    velocity: (caps[3], caps[4], caps[5]),
                    acceleration: (caps[6], caps[7], caps[8]),
                },
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let particles = read("test-input.txt");
        assert_eq!(find_long_term_closest_to_origin(&particles), 0);

        let particles = read("test-input2.txt");
        assert_eq!(count_particles_after_collisions(particles), 1);
    }
}
