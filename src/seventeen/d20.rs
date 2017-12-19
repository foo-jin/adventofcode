use failure::Error;

pub fn run(input: &str) -> Result<usize, Error> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, expected: usize) {
        match run(input) {
            Ok(result) => {
                assert_eq!(result, expected);
            }
            Err(e) => {
                for cause in e.causes() {
                    println!("{}", cause);
                }
            }
        };        
    }

    #[test]
    fn test_first() {
        let input = include_str!("../../data/d19-test");
        assert_eq!(first(input).unwrap(), "ABCDEF");
    }

    #[test]
    fn test_second() {
        let input = include_str!("../../data/d19-test");
        assert_eq!(second(input).unwrap(), 38);
    }
}