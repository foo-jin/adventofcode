use super::Result;

pub fn run(input: &str) -> Result<(u32, u32)> {
    let mut nesting = 0;
    let mut score = 0;
    let mut count = 0;
    let mut chars = input.trim().chars();

    while let Some(c) = chars.next() {
        match c {
            '!' => {
                chars.next();
            }
            '<' => while let Some(c) = chars.next() {
                match c {
                    '!' => {
                        chars.next();
                    }
                    '>' => break,
                    _ => count += 1,
                }
            },
            '{' => {
                nesting += 1;
                score += nesting;
            }
            '}' => nesting -= 1,
            ',' => (),
            other => bail!("unexpected input: {}", other),
        }
    }

    Ok((score, count))
}

#[cfg(test)]
mod tests {
    use super::*;
    use seventeen::check;

    #[test]
    fn test_both1() {
        check(run("{}<>"), (1, 0));
    }

    #[test]
    fn test_both2() {
        check(run("{{{}}}<random characters>"), (6, 17));
    }

    #[test]
    fn test_both3() {
        check(run("{{},{}}<{o\"i!a,<{i<a>"), (5, 10));
    }

    #[test]
    fn test_both4() {
        check(run("{{{},{},{{}}}}<<<<>"), (16, 3));
    }

    #[test]
    fn test_both5() {
        check(run("{<a>,<a>,<a>,<a>}<{!>}>"), (1, 6));
    }

    #[test]
    fn test_both6() {
        check(run("{{<ab>},{<ab>},{<ab>},{<ab>}}<!!>"), (9, 8));
    }

    #[test]
    fn test_both7() {
        check(run("{{<a!>},{<a!>},{<a!>},{<ab>}}<!!!>>"), (3, 17));
    }

    use test::Bencher;
    const FULL: &str = include_str!("../../data/d9-test");

    #[bench]
    fn bench_both(b: &mut Bencher) {
        b.iter(|| check(run(FULL), (12505, 6671)))
    }
}
