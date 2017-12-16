use failure::Error;

const A: u64 = 16807;
const B: u64 = 48271;
const DIV: u64 = 2147483647;

pub fn genmatch(input: &str) -> Result<u32, Error> {
    let mut line = input.trim().split(", ");
    let mut a: u64 = line.next().unwrap().parse()?;
    let mut b: u64 = line.next().unwrap().parse()?;
    let mut matches = 0;

    for _ in 0..5000000 {
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

    fn check_smth(input: &str, expected: u32) {
        let result = genmatch(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_genmatch() {
        let input = "65, 8921";
        check_smth(input, 309);
    }
}
