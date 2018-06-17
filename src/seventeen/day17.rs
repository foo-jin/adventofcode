use super::Result;

pub fn spinlock(steps: usize) -> u32 {
    let mut buf = Vec::with_capacity(2018);
    buf.push(0);
    let mut i = 0;

    for k in 1..2018 {
        i = ((i + steps) % k) + 1;

        buf.insert(i, k as u32);
    }

    buf[i + 1]
}


pub fn angry_spinlock(steps: u32, limit: u32) -> u32 {
    let mut i = 0;
    let mut result = 0;

    for k in 1..=limit {
        i = (i + steps) % k + 1;

        if i == 1 {
            result = k;
        }
    }

    result
}

pub fn solve() -> Result<()> {
    let input = super::get_input()?;
    let steps = input.parse()?;
    let first = spinlock(steps);
    let second = angry_spinlock(steps as u32, 50_000_000);

    println!("Day 17:\nPart 1: {}\nPart 2: {}\n", first, second);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(spinlock(3), 638);
    }

    #[test]
    fn test_second() {
        assert_eq!(angry_spinlock(3, 9), 9);
    }
}
