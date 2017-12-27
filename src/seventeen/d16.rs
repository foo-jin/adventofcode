use failure::Error;

use self::Dancemove::{P, S, X};

const ABC: [char; 16] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'
];

enum Dancemove {
    S(i32),
    X(i32, i32),
    P(char, char),
}

impl Dancemove {
    fn parse(input: &str) -> Result<Dancemove, Error> {
        let mut c = input.chars();
        let first = c.next().unwrap();
        let s = c.collect::<String>();
        let mut rest = s.split('/');

        let result = match first {
            's' => {
                let offset: i32 = rest.next().unwrap().parse()?;
                S(offset)
            }
            'x' => {
                let p1: i32 = rest.next().unwrap().parse()?;
                let p2: i32 = rest.next().unwrap().parse()?;
                X(p1, p2)
            }
            'p' => {
                let c1 = rest.next().unwrap().parse()?;
                let c2 = rest.next().unwrap().parse()?;
                P(c1, c2)
            }
            _ => panic!("wrong input"),
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

fn dance(input: &str, reps: usize, n: usize) -> Result<String, Error> {
    let routine: Vec<Dancemove> = input
        .trim()
        .split(',')
        .map(|s| Dancemove::parse(s))
        .collect::<Result<_, Error>>()?;

    let order = &mut ABC[..n];
    let n = n as i32;
    let mut offset = 0;

    let mut seen = vec![shift(offset, order)];
    let mut result = String::new();

    for i in 0..reps {
        for m in &routine {
            match *m {
                S(k) => {
                    offset = (offset + k) % n;
                }
                X(p1, p2) => {
                    let p1 = ((((p1 - offset) % n) + n) % n) as usize;
                    let p2 = ((((p2 - offset) % n) + n) % n) as usize;
                    order.swap(p1, p2);
                }
                P(c1, c2) => {
                    let p1 = order.iter().position(|c| *c == c1).unwrap();
                    let p2 = order.iter().position(|c| *c == c2).unwrap();
                    order.swap(p1, p2);
                }
            }
        }

        result = shift(offset, &order);

        if !seen.contains(&result) {
            seen.push(result.clone());
        } else {
            result = seen.remove(reps % (i + 1));
            break;
        }
    }

    Ok(result)
}

pub fn run(input: &str) -> Result<String, Error> {
    dance(input, 1_000_000_000, 16)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_dance(input: &str, rep: usize, expected: &str) {
        let result = dance(input, rep, expected.len()).unwrap();
        assert_eq!(result.as_str(), expected);
    }

    #[test]
    fn test_dance1() {
        let input = "s1,x3/4,pe/b";
        check_dance(input, 2, "ceadb");
    }

    #[test]
    fn test_dance2() {
        let input = include_str!("../../data/d16-test");
        check_dance(input, 1_000_000_000, "gnflbkojhicpmead")
    }
}
