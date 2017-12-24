use failure::Error;

type Component = (usize, usize);

fn parse_connectors(s: &str) -> Vec<Component> {
    s.trim()
        .lines()
        .map(|s| {
            let mut it = s.split("/");
            let first = it.next()
                .expect("no connector present")
                .parse()
                .expect("invalid input");
            let second = it.next()
                .expect("no connector present")
                .parse()
                .expect("invalid input");
            (first, second)
        })
        .collect()
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
struct Bridge {
    len: usize,
    str: usize,
}

impl Bridge {
    fn new(len: usize, str: usize) -> Bridge {
        Bridge { len, str }
    }
}

fn backtrack(xs: &[Component], used: &mut Vec<Component>, pins: usize, bridge: Bridge) -> Bridge {
    let mut max = bridge;
    for &(pin, pout) in xs {
        let pins = if pin == pins {
            pout
        } else if pout == pins {
            pin
        } else {
            continue;
        };

        if !used.contains(&(pin, pout)) {
            used.push((pin, pout));
            let mut new = bridge;
            new.len += 1;
            new.str += pin + pout;
            max = max.max(backtrack(xs, used, pins, new));
            used.pop();
        }
    }

    max
}

fn setup(xs: Vec<Component>) -> usize {
    backtrack(&xs, &mut vec![], 0, Bridge::new(0, 0)).str
}

pub fn run(input: &str) -> Result<usize, Error> {
    let connectors = parse_connectors(input);
    let result = setup(connectors);
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
