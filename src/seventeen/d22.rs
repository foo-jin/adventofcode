
use failure::Error;

pub fn run(input: &str) -> Result<usize, Error> {
    Ok(0)
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
}