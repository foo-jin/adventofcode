
use failure::Error;

pub fn run(input: &str) -> Result<usize, Error> {
    Ok(0)
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    use seventeen::check;

    const IN: &str = include_str!("../../data/d23-test");

    #[test]
    fn test_first() {
        let result = run(IN);
        let expected = 5929;
        check(result, expected);
    }
}