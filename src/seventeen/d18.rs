use failure::Error;

pub fn run(input: &str) -> Result<usize, Error> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinlock1() {
        assert_eq!(spinlock(3, 9).unwrap(), 9);
    }
}