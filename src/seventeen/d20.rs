use failure::Error;
use std::ops::AddAssign;

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
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        *self = Vector{ x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
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
        let pos = *vs.get(0).unwrap();
        let vel = *vs.get(1).unwrap();
        let acc = *vs.get(2).unwrap();

        Ok(Particle { pos, vel, acc })
    }

    fn update(&mut self) {
        self.vel += self.acc;
        self.pos += self.vel;
    }

    fn eq_pos(&self, rhs: Particle) -> bool {
        self.pos == rhs.pos
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
        .map(|p| {
            let Vector { x, y, z } = p.acc;
            (x.abs() + y.abs() + z.abs()) as usize
        })
        .enumerate()
        .min_by(|(_,x), (_, y)| x.cmp(y))
        .map(|(i, _)| i)
        .unwrap();
    Ok(result)
}

fn second(input: &str) -> Result<usize, Error> {
    let mut particles = parse(input)?;
    for _ in 0..1000 {
        let aux = particles.clone();
        particles.retain(|p| !aux.iter().any(|p2| p != p2 && p.eq_pos(*p2)));
        for p in particles.iter_mut() {
            p.update();
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

    fn check_first(input: &str, expected: usize) {
        match first(input) {
            Ok(result) => {
                assert_eq!(result, expected);
            }
            Err(e) => {
                for cause in e.causes() {
                    println!("{}", cause);
                }
                panic!("test failed");
            }
        };
    }

    fn check_second(input: &str, expected: usize) {
        match second(input) {
            Ok(result) => {
                assert_eq!(result, expected);
            }
            Err(e) => {
                for cause in e.causes() {
                    println!("{}", cause);
                }
                panic!("test failed");
            }
        };
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
}
