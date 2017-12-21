use std::ops::AddAssign;
use std::cmp::Ordering;

use failure::Error;


#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Vector {
    x: i64,
    y: i64,
    z: i64,
}

impl Vector {
    fn from_str(input: &str) -> Result<Vector, Error> {
        let brackets: &[char] = &['<', '>'];
        let mut it = input.trim_matches(brackets).trim().split(',');

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

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Particle {
    pos: Vector,
    vel: Vector,
    acc: Vector,
}

impl Particle {
    fn from_str(input: &str) -> Result<Particle, Error> {
        let vs: Vec<Vector> = input
            .split(", ")
            .map(|s| {
                let s: String = s.chars().skip(2).collect();
                Vector::from_str(&s)
            })
            .collect::<Result<_, Error>>()?;
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

fn parse(input: &str) -> Result<Vec<Particle>, Error> {
    input.lines().map(|s| Particle::from_str(s)).collect()
}

#[allow(dead_code)]
fn first(input: &str) -> Result<usize, Error> {
    let particles = parse(input)?;
    let result = particles
        .iter()
        .enumerate()
        .min_by(|(_, &x), (_, &y)| x.acc.cmp(&y.acc))
        .map(|(i, _)| i)
        .unwrap();
    Ok(result)
}

fn second(input: &str) -> Result<usize, Error> {
    let mut particles = parse(input)?;
    
    for _ in 0..1000 {
        let aux = particles.clone();
        particles.retain(|p| !aux.iter().any(|other| p != other && p.pos == other.pos));

        for p in &mut particles {
            p.update()
        }
    }

    Ok(particles.len())
}

pub fn run(input: &str) -> Result<usize, Error> {
    second(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use seventeen::check;

    fn check_first(input: &str, expected: usize) {
        check(first(input), expected);
    }

    fn check_second(input: &str, expected: usize) {
        check(second(input), expected);
    }

    #[test]
    fn test_first() {
        let input = "p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>\np=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>";
        check_first(input, 0);
    }

    #[test]
    fn test_second() {
        let input = "p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>\np=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>\np=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>\np=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>";
        check_second(input, 1);
    }

    use test::Bencher;
    #[bench]
    fn bench_second(b: &mut Bencher) {
        let input = include_str!("../../data/d20-test");
        b.iter(|| check_second(input, 471));
    }
}
