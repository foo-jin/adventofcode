use failure::*;
use rayon::prelude::*;

use self::Pixel::*;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Pixel {
    On,
    Off,
}

impl Pixel {
    fn new(c: char) -> Result<Pixel, Error> {
        let result = match c {
            '#' => On,
            '.' => Off,
            _ => bail!("unexpected character: {}", c),
        };

        Ok(result)
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Pattern {
    pixels: Vec<Vec<Pixel>>,
    size: usize,
}

impl Pattern {
    fn new(pixels: Vec<Vec<Pixel>>) -> Pattern {
        let size = pixels.len();
        Pattern { pixels, size }
    }

    fn from_str(s: &str) -> Result<Pattern, Error> {
        let size = s.lines().count();
        let mut pixels = Vec::new();

        for line in s.lines() {
            let line: Vec<Pixel> = line.chars().map(Pixel::new).collect::<Result<_, Error>>()?;

            let len = line.len();
            ensure!(
                len == size,
                "incorrect pattern size: left={}, right={}",
                len,
                size
            );

            pixels.push(line);
        }

        Ok(Pattern::new(pixels))
    }

    fn split(&self) -> Result<Vec<Pattern>, Error> {
        let mut result = Vec::new();
        let pix = &self.pixels;

        let size = if self.size % 2 == 0 {
            2
        } else if self.size % 3 == 0 {
            3
        } else {
            bail!("unexpected size")
        };

        let n = self.size / size;

        for y in 0..n {
            let mut it = vec![];

            for i in 0..size {
                let offset = i + (y * size);
                it.push(pix[offset].iter());
            }

            for _ in 0..n {
                let mut temp: Vec<Vec<Pixel>> = Vec::with_capacity(size);

                for it in &mut it {
                    temp.push(it.take(size).cloned().collect());
                }

                result.push(Pattern::new(temp));
            }
        }
        Ok(result)
    }

    fn join(squares: &[Pattern]) -> Result<Pattern, Error> {
        let n = {
            let sq = (squares.len() as f64).sqrt();
            sq as usize
        };

        let size = squares.first().unwrap().size;
        let mut it = squares.iter();
        let mut result = vec![Vec::with_capacity(n * size); n * size];

        for y in 0..n {
            let offset = y * size;
            for _ in 0..n {
                let pat = it.next().unwrap();
                for (i, v) in pat.pixels.iter().enumerate() {
                    result[offset + i].extend(v.iter())
                }
            }
        }

        Ok(Pattern::new(result))
    }

    fn rotate(&self) -> Result<Pattern, Error> {
        let pix = &self.pixels;
        let n = self.size;
        let rot = match n {
            2 => vec![vec![pix[1][0], pix[0][0]], vec![pix[1][1], pix[0][1]]],
            3 => vec![
                vec![pix[2][0], pix[1][0], pix[0][0]],
                vec![pix[2][1], pix[1][1], pix[0][1]],
                vec![pix[2][2], pix[1][2], pix[0][2]],
            ],
            _ => bail!("rotate: unkown pattern size"),
        };

        Ok(Pattern::new(rot))
    }

    fn vflip(&self) -> Pattern {
        let mut pix = self.pixels.clone();
        let n = self.size;
        pix.swap(0, n - 1);

        Pattern::new(pix)
    }

    fn hflip(&self) -> Pattern {
        let mut pix = self.pixels.clone();
        let n = self.size;

        for v in &mut pix {
            v.swap(0, n - 1);
        }

        Pattern::new(pix)
    }

    fn rotations(self) -> Result<Vec<Pattern>, Error> {
        let r1 = self.rotate()?;
        let r2 = r1.rotate()?;
        let r3 = r2.rotate()?;

        Ok(vec![self, r1, r2, r3])
    }

    fn permute(self) -> Result<Vec<Pattern>, Error> {
        let mut rotations = Vec::new();

        let hor = self.hflip();
        let vert = self.vflip();

        rotations.extend(self.rotations()?);
        rotations.extend(hor.rotations()?);
        rotations.extend(vert.rotations()?);

        Ok(rotations)
    }

    fn count_on(&self) -> usize {
        let mut count = 0;
        for line in &self.pixels {
            for pix in line.iter() {
                match pix {
                    On => count += 1,
                    Off => (),
                }
            }
        }
        count
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Rule {
    variations: Vec<Pattern>,
    out: Pattern,
}

impl Rule {
    fn new(variations: Vec<Pattern>, out: Pattern) -> Rule {
        Rule { variations, out }
    }

    fn from_str(s: &str) -> Result<Rule, Error> {
        let mut it = s.split(" => ").map(|s| s.replace("/", "\n"));

        let variations = {
            let s = it.next()
                .ok_or(err_msg("no source pattern present"))?;
            Pattern::from_str(&s)?.permute()?
        };

        let out = {
            let s = it.next()
                .ok_or(err_msg("no target pattern present"))?;
            Pattern::from_str(&s)?
        };

        Ok(Rule::new(variations, out))
    }

    fn try(&self, p: &Pattern) -> Option<Pattern> {
        if self.variations.contains(p) {
            Some(self.out.clone())
        } else {
            None
        }
    }
}

struct RuleSet {
    rules: Vec<Rule>,
}

impl RuleSet {
    fn new(rules: Vec<Rule>) -> RuleSet {
        RuleSet { rules }
    }

    fn from_str(s: &str) -> Result<RuleSet, Error> {
        let rules = s.lines()
            .map(|s| Rule::from_str(s))
            .collect::<Result<_, Error>>()?;
        Ok(RuleSet::new(rules))
    }

    fn apply(&self, pat: &Pattern) -> Pattern {
        let mut result = None;
        for rule in &self.rules {
            let out = rule.try(pat);
            if out.is_some() {
                result = out;
                break;
            }
        }
        result.expect("no applicable rule present")
    }
}

struct Grid {
    pattern: Pattern,
    rules: RuleSet,
}

impl Grid {
    fn new(rules: RuleSet) -> Grid {
        let pixels = vec![vec![Off, On, Off], vec![Off, Off, On], vec![On, On, On]];
        let pattern = Pattern::new(pixels);
        Grid { pattern, rules }
    }

    fn from_str(s: &str) -> Result<Grid, Error> {
        let rules = RuleSet::from_str(s)?;
        Ok(Grid::new(rules))
    }

    fn enhance(&mut self) -> Result<(), Error> {
        let next: Vec<Pattern> = self.pattern
            .split()?
            .into_par_iter()
            .map(|p| self.rules.apply(&p))
            .collect();
        self.pattern = Pattern::join(next.as_slice())?;
        Ok(())
    }

    fn count_on(&self) -> usize {
        self.pattern.count_on()
    }
}

fn exec(input: &str, n: usize) -> Result<usize, Error> {
    let mut grid = Grid::from_str(input)?;
    for _ in 0..n {
        grid.enhance()?;
    }
    Ok(grid.count_on())
}

pub fn run(input: &str) -> Result<usize, Error> {
    exec(input, 18)
}

#[cfg(test)]
mod tests {
    use super::*;
    use seventeen::check;

    #[test]
    fn test_pattern() {
        let result = Pattern::from_str(".#.\n..#\n###");
        let expected = Pattern::new(vec![
            vec![Off, On, Off],
            vec![Off, Off, On],
            vec![On, On, On],
        ]);
        check(result, expected);
    }

    #[test]
    fn test_rotate() {
        let init = Pattern::from_str(".#.\n..#\n###").unwrap();
        let result = init.rotate();
        let expected = Pattern::new(vec![
            vec![On, Off, Off],
            vec![On, Off, On],
            vec![On, On, Off],
        ]);
        check(result, expected);
    }

    #[test]
    fn test_split() {
        let input = "#..#\n....\n....\n#..#";
        let init = Pattern::from_str(input).unwrap();
        let result = init.split();
        let expected = vec![
            Pattern::new(vec![vec![On, Off], vec![Off, Off]]),
            Pattern::new(vec![vec![Off, On], vec![Off, Off]]),
            Pattern::new(vec![vec![Off, Off], vec![On, Off]]),
            Pattern::new(vec![vec![Off, Off], vec![Off, On]]),
        ];
        check(result, expected);
    }

    #[test]
    fn test_join() {
        let input = vec![
            Pattern::new(vec![vec![On, Off], vec![Off, Off]]),
            Pattern::new(vec![vec![Off, On], vec![Off, Off]]),
            Pattern::new(vec![vec![Off, Off], vec![On, Off]]),
            Pattern::new(vec![vec![Off, Off], vec![Off, On]]),
        ];
        let result = Pattern::join(input.as_slice());
        let expected = Pattern::from_str("#..#\n....\n....\n#..#").unwrap();
        check(result, expected);
    }

    #[test]
    fn test_exec() {
        let input = "../.# => ##./#../...\n.#./..#/### => #..#/..../..../#..#";
        let result = exec(input, 2);
        let expected = 12;
        check(result, expected);
    }

    use test::Bencher;

    #[bench]
    fn bench_exec(b: &mut Bencher) {
        let input = include_str!("../../data/d21-test");
        b.iter(|| check(exec(input, 18), 3018423));
    }
}
