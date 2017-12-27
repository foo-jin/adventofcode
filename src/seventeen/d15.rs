use failure::Error;

const A: u64 = 16_807;
const B: u64 = 48_271;
const DIV: u64 = 2_147_483_647;

pub fn genmatch(input: &str) -> Result<u32, Error> {
    let mut line = input.trim().split(", ");
    let mut a: u64 = line.next().unwrap().parse()?;
    let mut b: u64 = line.next().unwrap().parse()?;
    let mut matches = 0;

    for _ in 0..5_000_000 {
        a = (a * A) % DIV;

        while a % 4 != 0 {
            a = (a * A) % DIV;
        }

        b = (b * B) % DIV;

        while b % 8 != 0 {
            b = (b * B) % DIV;
        }

        if a as u16 == b as u16 {
            matches += 1;
        }
    }

    Ok(matches)
}

#[cfg(test)]
mod tests {
    use super::*;
    use seventeen::check;

    #[test]
    fn test_genmatch() {
        let input = "65, 8921";
        check(genmatch(input), 309);
    }
}
