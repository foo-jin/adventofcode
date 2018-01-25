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
    length: u32,
    strength: u32,
    pins: u32,
}

impl Bridge {
    fn new() -> Bridge {
        Bridge {
            length: 0,
            strength: 0,
            pins: 0,
        }
    }

    fn extend(&self, (pin, pout): Connector) -> Option<Bridge> {
        let pins = match self.pins {
            p if p == pin => pout,
            p if p == pout => pin,
            _ => return None,
        };

        Some(Bridge {
            length: self.length + 1,
            strength: self.strength + pin + pout,
            pins,
        })
    }
}

fn optimal_bridge<F>(quality: F, xs: &mut [Connector]) -> Bridge
where
    F: Fn(Bridge, Bridge) -> Bridge,
{
    fn backtrack<G>(quality: &G, xs: &mut [Connector], i: usize, mut max: Bridge) -> Bridge
    where
        G: Fn(Bridge, Bridge) -> Bridge,
    {
        let bridge = max;
        for j in i..xs.len() {
            xs.swap(i, j);
            if let Some(bridge) = bridge.extend(xs[i]) {
                max = quality(max, backtrack(quality, xs, i + 1, bridge));
            }
            xs.swap(j, i);
        }

        max
    }

    backtrack(&quality, xs, 0, Bridge::new())
}

fn first(input: &str) -> Result<u32> {
    let mut connectors = parse_connectors(input)?;
    let stronger = |b1: Bridge, b2: Bridge| if b1.strength > b2.strength { b1 } else { b2 };
    let bridge = optimal_bridge(stronger, &mut connectors);
    Ok(bridge.strength)
}

fn second(input: &str) -> Result<u32> {
    let mut connectors = parse_connectors(input)?;
    let bridge = optimal_bridge(Bridge::max, &mut connectors);
    Ok(bridge.strength)
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
