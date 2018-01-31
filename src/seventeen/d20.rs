use std::ops::AddAssign;
use std::cmp::Ordering;
use std::collections::HashMap;

use super::Result;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Vector {
    x: i64,
    y: i64,
    z: i64,
}

impl Vector {
    fn parse(input: &str) -> Result<Vector> {
        let brackets: &[char] = &['<', '>', ' ', '\n', '\t'];
        let mut it = input.trim_matches(brackets).split(',');

        let x = it.next().unwrap().parse()?;
        let y = it.next().unwrap().parse()?;
        let z = it.next().unwrap().parse()?;

        Ok(Vector { x, y, z })
    }

    fn sum(&self) -> usize {
        (self.x.abs() + self.y.abs() + self.z.abs()) as usize
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        *self = Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Ord for Vector {
    fn cmp(&self, rhs: &Vector) -> Ordering {
        self.sum().cmp(&rhs.sum())
    }
}

impl PartialOrd for Vector {
    fn partial_cmp(&self, rhs: &Vector) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

#[derive(Clone, Copy, Debug)]
struct Particle {
    pos: Vector,
    vel: Vector,
    acc: Vector,
}

impl Particle {
    fn parse(input: &str) -> Result<Particle> {
        let vs: Vec<Vector> = input
            .split(", ")
            .map(|s| {
                let s: String = s.chars().skip(2).collect();
                Vector::parse(&s)
            })
            .collect::<Result<_>>()?;

        let pos = vs[0];
        let vel = vs[1];
        let acc = vs[2];

        Ok(Particle { pos, vel, acc })
    }

    fn update(&mut self) {
        self.vel += self.acc;
        self.pos += self.vel;
    }
}

fn parse(input: &str) -> Result<Vec<Particle>> {
    input.lines().map(Particle::parse).collect()
}

fn first(particles: &[Particle]) -> usize {
    particles
        .iter()
        .enumerate()
        .min_by(|(_, x), (_, y)| x.acc.cmp(&y.acc))
        .map(|(i, _)| i)
        .unwrap()
}

fn second(mut particles: Vec<Particle>) -> usize {
    for _ in 0..1000 {
        let mut seen = HashMap::new();
        for p in &particles {
            *seen.entry(p.pos).or_insert(0) += 1;
        }

        particles.retain(|p| seen[&p.pos] < 2);
        particles.iter_mut().for_each(|p| p.update());
    }

    particles.len()
}

pub fn solve() -> Result<()> {
    let input = super::get_input()?;
    let particles = parse(&input)?;
    let first = first(&particles);
    let second = second(particles);

    println!(
        "Day 20:\n\
         Part 1: {}\n\
         Part 2: {}\n",
        first, second
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let particles = parse(
            "p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>\n\
             p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>",
        ).unwrap();
        assert_eq!(first(&particles), 0);
    }

    #[test]
    fn test_second() {
        let particles = parse(
            "p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>\n\
             p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>\n\
             p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>\n\
             p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>",
        ).unwrap();
        assert_eq!(second(particles), 1);
    }

    use test::Bencher;
    const FULL: &str = include_str!("../../data/d20-test");

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        let particles = parse(FULL).unwrap();
        b.iter(|| assert_eq!(first(&particles), 119))
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        let particles = parse(FULL).unwrap();
        b.iter(|| assert_eq!(second(particles.clone()), 471))
    }
}
