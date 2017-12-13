use failure::Error;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Layer {
    depth: u32,
    range: u32,
    dir: i8,
    sc: u32,
}

impl Layer {
    fn new(depth: u32, range: u32) -> Layer {
        Layer {
            depth,
            range,
            dir: 1,
            sc: 1,
        }
    }

    fn move_sc(&mut self) {
        let sc = self.sc;
        let range = self.range;
        if sc == range {
            self.dir = -1;
        } else if sc == 1 {
            self.dir = 1;
        }
        self.sc = (sc as i32 + self.dir as i32) as u32;
    }

    fn update(&mut self, n: u32) {
        for _ in 0..n {
            self.move_sc();
        }
    }
}

pub fn firewall(input: &str) -> Result<u32, Error> {
    let layers: Vec<_> = input
        .trim()
        .lines()
        .map(|s| {
            let mut it = s.split(": ");
            let depth = it.next().unwrap().parse().unwrap();
            let range = it.next().unwrap().parse().unwrap();
            Layer::new(depth, range)
        })
        .collect();


    let wait = (0..).filter(|delay| {
        !layers.iter().any(|layer| {
            (delay + layer.depth) % (2 * (layer.range - 1)) == 0
        })
    }).next().unwrap();
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