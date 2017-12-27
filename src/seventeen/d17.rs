use failure::Error;

fn spinlock(steps: u32, limit: u32) -> u32 {
    let mut i = 0;
    let mut result = 0;

    for k in 1..=limit {
        let n = k;
        i = ((i + steps) % n) + 1;

        if i == 1 {
            result = k;
        }
    }

    result
}

pub fn run(input: &str) -> Result<u32, Error> {
    let steps: u32 = input.parse()?;
    Ok(spinlock(steps, 50_000_000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinlock1() {
        assert_eq!(spinlock(3, 9), 9);
    }
}