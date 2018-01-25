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

fn first(input: &str) -> Result<usize> {
    let particles = parse(input)?;
    let result = particles
        .iter()
        .enumerate()
        .min_by(|(_, x), (_, y)| x.acc.cmp(&y.acc))
        .map(|(i, _)| i)
        .unwrap();

    Ok(result)
}

fn second(input: &str) -> Result<usize> {
    let mut particles = parse(input)?;

    for _ in 0..1000 {
        let mut seen = HashMap::new();
        for p in &particles {
            *seen.entry(p.pos).or_insert(0) += 1;
        }

        particles.retain(|p| seen[&p.pos] < 2);

        for p in &mut particles {
            p.update()
        }
    }

    Ok(particles.len())
}

pub fn run(input: &str) -> Result<usize> {
    second(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use seventeen::check;

    #[test]
    fn test_first() {
        let input = "p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>\np=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>";
        check(first(input), 0);
    }

    #[test]
    fn test_second() {
        let input = "p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>\np=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>\np=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>\np=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>";
        check(second(input), 1);
    }

    use test::Bencher;
    const FULL: &str = include_str!("../../data/d20-test");

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        b.iter(|| check(first(FULL), 119))
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        b.iter(|| check(second(FULL), 471))
    }
}
