use failure::Error;

fn spinlock(steps: usize, limit: usize) -> Result<usize, Error> {
    let mut i = 0;
    let mut result = 0;

    for k in 1..=limit {
        let n = k;
        i = ((i + steps) % n) + 1;

        if i == 1 {
            result = k;
        }
    }
    Ok(result)
}

pub fn run(input: &str) -> Result<usize, Error> {
    let steps: usize = input.parse()?;
    spinlock(steps, 50_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinlock1() {
        assert_eq!(spinlock(3, 9).unwrap(), 9);
    }
}