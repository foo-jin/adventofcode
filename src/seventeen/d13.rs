use failure::Error;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Layer {
    depth: u32,
    range: u32,
}

impl Layer {
    fn parse(s: &str) -> Result<Layer, Error> {
        let mut it = s.split(": ");
        let depth = it.next().unwrap().parse()?;
        let range = it.next().unwrap().parse()?;

        Ok(Layer { depth, range })
    }
}

fn parse_layers(s: &str) -> Result<Vec<Layer>, Error> {
    s.trim()
        .lines()
        .map(Layer::parse)
        .collect::<Result<_, Error>>()
}

fn first(input: &str) -> Result<u32, Error> {
    let layers = parse_layers(input)?;
    let severity = layers
        .iter()
        .map(|layer| {
            if layer.depth % (2 * (layer.range - 1)) == 0 {
                layer.depth * layer.range
            } else {
                0
            }
        })
        .sum();

    Ok(severity)
}

pub fn second(input: &str) -> Result<u32, Error> {
    let layers = parse_layers(input)?;
    let wait = (0..)
        .find(|delay| {
            !layers
                .iter()
                .any(|layer| (delay + layer.depth) % (2 * (layer.range - 1)) == 0)
        })
        .unwrap();

    Ok(wait)
}

#[cfg(test)]
mod tests {
    use super::*;
    use seventeen::check;

    const IN: &str = "0: 3\n1: 2\n4: 4\n6: 4";

    #[test]
    fn test_first() {
        check(first(IN), 24);
    }

    #[test]
    fn test_second() {
        check(second(IN), 10);
    }

    use test::Bencher;
    const FULL: &str = include_str!("../../data/d13-test");
    
    #[bench]
    fn bench_p1(b: &mut Bencher) {
        b.iter(|| check(first(FULL), 788))
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        b.iter(|| check(second(FULL), 3_905_748))
    }
}
