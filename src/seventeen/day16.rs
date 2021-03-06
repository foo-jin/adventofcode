use super::Result;

use self::Dancemove::{P, S, X};

const ABC: [char; 16] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
];

pub enum Dancemove {
    S(i32),
    X(i32, i32),
    P(char, char),
}

impl Dancemove {
    fn parse(input: &str) -> Result<Dancemove> {
        let mut c = input.chars();
        let first = c.next().unwrap();
        let s: String = c.collect();
        let mut rest = s.split('/');

        let result = match first {
            's' => {
                let offset = rest.next().unwrap().parse()?;
                S(offset)
            }
            'x' => {
                let p1 = rest.next().unwrap().parse()?;
                let p2 = rest.next().unwrap().parse()?;
                X(p1, p2)
            }
            'p' => {
                let c1 = rest.next().unwrap().parse()?;
                let c2 = rest.next().unwrap().parse()?;
                P(c1, c2)
            }
            _ => bail!("wrong input"),
        };

        Ok(result)
    }
}

fn shift(offset: i32, order: &[char]) -> String {
    let n = order.len() as i32;
    let mut result = String::new();
    for j in 0..n {
        let index = ((((j - offset) % n) + n) % n) as usize;
        result.push(order[index]);
    }
    result
}

pub fn parse_routine(s: &str) -> Result<Vec<Dancemove>> {
    s.trim()
        .split(',')
        .map(Dancemove::parse)
        .collect::<Result<_>>()
}

pub fn dance(routine: &[Dancemove], reps: usize) -> String {
    let order = &mut ABC;
    let n = order.len() as i32;
    let mut offset = 0;
    let mut seen = vec![shift(offset, order)];
    let mut result = String::new();

    for i in 0..reps {
        for m in routine {
            match m {
                S(k) => {
                    offset = (offset + k) % n;
                }
                X(p1, p2) => {
                    let p1 = ((((p1 - offset) % n) + n) % n) as usize;
                    let p2 = ((((p2 - offset) % n) + n) % n) as usize;
                    order.swap(p1, p2);
                }
                P(c1, c2) => {
                    let p1 = order.iter().position(|c| c == c1).unwrap();
                    let p2 = order.iter().position(|c| c == c2).unwrap();
                    order.swap(p1, p2);
                }
            }
        }

        result = shift(offset, order);

        if !seen.contains(&result) {
            seen.push(result.clone());
        } else {
            result = seen.remove(reps % (i + 1));
            break;
        }
    }

    result
}

pub fn solve() -> Result<()> {
    let input = super::get_input()?;
    let routine = parse_routine(&input)?;
    let first = dance(&routine, 1);
    let second = dance(&routine, 1_000_000_000);

    println!("Day 16:\nPart 1: {}\nPart 2: {}\n", first, second);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const FULL: &str = include_str!("../../data/d16-test");

    #[test]
    fn test_first() {
        let routine = parse_routine(FULL).unwrap();
        assert_eq!(dance(&routine, 1), "ociedpjbmfnkhlga".to_owned())
    }
}
