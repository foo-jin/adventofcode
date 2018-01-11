use failure::*;

use super::Result;

type Connector = (u32, u32);

fn parse_connectors(s: &str) -> Result<Vec<Connector>> {
    s.trim()
        .lines()
        .map(|s| {
            let mut it = s.split('/');
            let first = it.next()
                .ok_or_else(|| err_msg("unexpected end of input"))?
                .parse()?;

            let second = it.next()
                .ok_or_else(|| err_msg("unexpected end of input"))?
                .parse()?;

            Ok((first, second))
        })
        .collect()
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
struct Bridge {
    len: u32,
    str: u32,
    pins: u32,
}

impl Bridge {
    fn new() -> Bridge {
        Bridge {
            len: 0,
            str: 0,
            pins: 0,
        }
    }

    fn stronger(b1: Bridge, b2: Bridge) -> Bridge {
        if b1.str >= b2.str {
            b1
        } else {
            b2
        }
    }

    fn extend(&self, (pin, pout): Connector) -> Option<Bridge> {
        let pins = match self.pins {
            p if p == pin => pout,
            p if p == pout => pin,
            _ => return None,
        };

        let result = Bridge {
            len: self.len + 1,
            str: self.str + pin + pout,
            pins,
        };

        Some(result)
    }
}

fn backtrack<F>(measure: &F, xs: &[Connector], used: &mut Vec<Connector>, bridge: Bridge) -> Bridge
where
    F: Fn(Bridge, Bridge) -> Bridge,
{
    let mut max = bridge;

    for &(pin, pout) in xs {
        if !used.contains(&(pin, pout)) {
            if let Some(bridge) = bridge.extend((pin, pout)) {
                used.push((pin, pout));
                max = measure(max, backtrack(measure, xs, used, bridge));
                used.pop();
            }
        }
    }

    max
}

fn setup<F>(measure: F, xs: &[Connector]) -> Bridge
where
    F: Fn(Bridge, Bridge) -> Bridge,
{
    backtrack(&measure, xs, &mut vec![], Bridge::new())
}

fn first(input: &str) -> Result<u32> {
    let connectors = parse_connectors(input)?;
    let bridge = setup(Bridge::stronger, &connectors);
    Ok(bridge.str)
}

fn second(input: &str) -> Result<u32> {
    let connectors = parse_connectors(input)?;
    let bridge = setup(Bridge::max, &connectors);
    Ok(bridge.str)
}

pub fn run(input: &str) -> Result<u32> {
    second(input)
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    use seventeen::check;

    const IN: &str = "0/2\n2/2\n2/3\n3/4\n3/5\n0/1\n10/1\n9/10";

    #[test]
    fn test_first() {
        check(first(IN), 31);
    }

    #[test]
    fn test_second() {
        check(second(IN), 19);
    }

    use test::Bencher;
    const FULL: &str = include_str!("../../data/d24-test");

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        b.iter(|| check(first(FULL), 1906))
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        b.iter(|| check(second(FULL), 1824))
    }
}
