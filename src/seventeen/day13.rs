use super::Result;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Layer {
    depth: u32,
    range: u32,
}

impl Layer {
    fn from_str(s: &str) -> Result<Layer> {
        let mut it = s.split(": ");
        let depth = it.next().unwrap().parse()?;
        let range = it.next().unwrap().parse()?;

        Ok(Layer { depth, range })
    }
}

pub fn parse_layers(s: &str) -> Result<Vec<Layer>> {
    s.trim().lines().map(Layer::from_str).collect()
}

pub fn default_severity(layers: &[Layer]) -> u32 {
    layers
        .iter()
        .map(|layer| {
            if layer.depth % (2 * (layer.range - 1)) == 0 {
                layer.depth * layer.range
            } else {
                0
            }
        })
        .sum()
}

pub fn delay(layers: &[Layer]) -> u32 {
    (0..)
        .find(|delay| {
            !layers
                .iter()
                .any(|layer| (delay + layer.depth) % (2 * (layer.range - 1)) == 0)
        })
        .unwrap()
}

pub fn solve() -> Result<()> {
    let input = super::get_input()?;
    let layers = parse_layers(&input)?;
    let first = default_severity(&layers);
    let second = delay(&layers);

    println!("Day 13:\nPart 1: {}\nPart 2: {}\n", first, second);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const IN: &str = "0: 3\n1: 2\n4: 4\n6: 4";

    #[test]
    fn test_first() {
        let layers = parse_layers(IN).unwrap();
        assert_eq!(default_severity(&layers), 24);
    }

    #[test]
    fn test_second() {
        let layers = parse_layers(IN).unwrap();
        assert_eq!(delay(&layers), 10);
    }
}
