use failure::Error;

pub fn run(input: &str) -> Result<u32, Error> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_dance(input: &str, rep: usize, expected: &str) {
        let result = dance(input, rep, expected.len()).unwrap();
        assert_eq!(result.as_str(), expected);
    }

    #[test]
    fn test_dance1() {
        let input = "s1,x3/4,pe/b";
        check_dance(input, 2, "ceadb");
    }
}