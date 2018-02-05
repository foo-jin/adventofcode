use super::Result;

pub fn parse_buffer(s: &str) -> Result<Vec<i32>> {
    s.trim()
        .lines()
        .map(|s| s.parse().map_err(Into::into))
        .collect::<Result<_>>()
}

pub fn buffer_jump(lines: &mut [i32]) -> u32 {
    let n = lines.len() as i32;
    let (mut i, mut j) = (0, 0);

    while (0..n).contains(i) {
        let inst = &mut lines[i as usize];
        i += *inst;
        *inst += 1;
        j += 1;
    }

    j
}

pub fn buffer_jump_extreme(lines: &mut [i32]) -> u32 {
    let n = lines.len() as i32;
    let (mut i, mut j) = (0, 0);

    while (0..n).contains(i) {
        let el = &mut lines[i as usize];
        i += *el;
        *el += if *el >= 3 { -1 } else { 1 };
        j += 1;
    }

    j
}

pub fn solve() -> Result<()> {
    let input = super::get_input()?;
    let mut buffer = parse_buffer(&input)?;
    let first = buffer_jump(&mut buffer.clone());
    let second = buffer_jump_extreme(&mut buffer);

    println!(
        "Day 5:\n\
         Part 1: {}\n\
         Part 2: {}\n",
        first, second
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let mut input = [0, 3, 0, 1, -3];
        assert_eq!(buffer_jump(&mut input), 5);
    }

    #[test]
    fn test_second() {
        let mut input = [0, 3, 0, 1, -3];
        assert_eq!(buffer_jump_extreme(&mut input), 10);
    }
}
