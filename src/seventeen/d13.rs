use failure::Error;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Layer {
    depth: u32,
    range: u32,
}

impl Layer {
    fn new(depth: u32, range: u32) -> Layer {
        Layer { depth, range }
    }
}

pub fn firewall(input: &str) -> Result<u32, Error> {
    let layers: Vec<Layer> = input
        .trim()
        .lines()
        .map(|s| {
            let mut it = s.split(": ");
            let depth = it.next().unwrap().parse()?;
            let range = it.next().unwrap().parse()?;
            Ok(Layer::new(depth, range))
        })
        .collect::<Result<_, Error>>()?;


    let wait = (0..)
        .find(|delay| {
            !layers.iter().any(|layer| {
                (delay + layer.depth) % (2 * (layer.range - 1)) == 0
            })
        })
        .unwrap();
    Ok(wait)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_firewall() {
        let input = "0: 3\n1: 2\n4: 4\n6: 4";
        assert_eq!(firewall(input).unwrap(), 10);
    }
}