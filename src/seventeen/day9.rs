use super::Result;

pub fn process_stream(input: &str) -> Result<(u32, u32)> {
    let mut chars = input.trim().chars();
    let mut nesting = 0;
    let mut score = 0;
    let mut count = 0;

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

pub fn solve() -> Result<()> {
    let stream = super::get_input()?;
    let (first, second) = process_stream(&stream)?;

    println!("Day 9:\nPart 1: {}\nPart 2: {}\n", first, second);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use seventeen::check;

    #[test]
    fn test_both1() {
        check(process_stream("{}<>"), (1, 0));
    }

    #[test]
    fn test_both2() {
        check(process_stream("{{{}}}<random characters>"), (6, 17));
    }

    #[test]
    fn test_both3() {
        check(process_stream("{{},{}}<{o\"i!a,<{i<a>"), (5, 10));
    }

    #[test]
    fn test_both4() {
        check(process_stream("{{{},{},{{}}}}<<<<>"), (16, 3));
    }

    #[test]
    fn test_both5() {
        check(process_stream("{<a>,<a>,<a>,<a>}<{!>}>"), (1, 6));
    }

    #[test]
    fn test_both6() {
        check(process_stream("{{<ab>},{<ab>},{<ab>},{<ab>}}<!!>"), (9, 8));
    }

    #[test]
    fn test_both7() {
        check(
            process_stream("{{<a!>},{<a!>},{<a!>},{<ab>}}<!!!>>"),
            (3, 17),
        );
    }
}
