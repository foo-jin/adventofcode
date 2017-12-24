use failure::Error;

type Connector = (usize, usize);

fn parse_connectors(s: &str) -> Result<Vec<Connector>, Error> {
    s.trim()
        .lines()
        .map(|s| {
            let mut it = s.split('/');
            let first = it.next().expect("no connector present").parse()?;
            let second = it.next().expect("no connector present").parse()?;
            Ok((first, second))
        })
        .collect()
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
struct Bridge {
    len: usize,
    str: usize,
    pins: usize,
}

impl Bridge {
    fn new() -> Bridge {
        Bridge {
            len: 0,
            str: 0,
            pins: 0,
        }
    }

    fn extend(&self, (pin, pout): Connector) -> Option<Bridge> {
        let pins = if pin == self.pins {
            pout
        } else if pout == self.pins {
            pin
        } else {
            return None;
        };

        let result = Bridge {
            len: self.len + 1,
            str: self.str + pin + pout,
            pins,
        };

        Some(result)
    }
}

fn backtrack(xs: &[Connector], used: &mut Vec<Connector>, bridge: Bridge) -> Bridge {
    let mut max = bridge;

    for &(pin, pout) in xs {
        if !used.contains(&(pin, pout)) {
            if let Some(bridge) = bridge.extend((pin, pout)) {
                used.push((pin, pout));
                max = max.max(backtrack(xs, used, bridge));
                used.pop();
            }
        }
    }

    max
}

fn setup(xs: &[Connector]) -> usize {
    backtrack(xs, &mut vec![], Bridge::new()).str
}

pub fn run(input: &str) -> Result<usize, Error> {
    let connectors = parse_connectors(input)?;
    let result = setup(&connectors);
    Ok(result)
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    use seventeen::check;

    const IN: &str = "0/2\n2/2\n2/3\n3/4\n3/5\n0/1\n10/1\n9/10";

    #[test]
    fn test_run() {
        let result = run(IN);
        let expected = 19;
        check(result, expected);
    }
}
