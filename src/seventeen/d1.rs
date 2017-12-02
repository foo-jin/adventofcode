

fn parse(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

pub fn reverse_captcha(input: &str) -> u32 {
    let input = parse(input);
    let mut result = 0;
    let n = input.len();
    for i in 0..n {
        if input[i] == input[(i + 1) % n] {
            result += input[i];
        }
    }
    result
}

pub fn reverse_captcha_half(input: &str) -> u32 {
    let input = parse(input);
    let mut result = 0;
    let n = input.len();
    for i in 0..n {
        if input[i] == input[(i + (n / 2)) % n] {
            result += input[i];
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use seventeen::d1::*;

    #[test]
    fn test_reverse_captcha1122() {
        assert_eq!(reverse_captcha("1122"), 3);
    }

    #[test]
    fn test_reverse_captcha1111() {
        assert_eq!(reverse_captcha("1111"), 4);
    }

    #[test]
    fn test_reverse_captcha1234() {
        assert_eq!(reverse_captcha("1234"), 0);
    }

    #[test]
    fn test_reverse_captcha_long() {
        assert_eq!(reverse_captcha("91212129"), 9);
    }

    #[test]
    fn test_rev_captcha_half1212() {
        assert_eq!(reverse_captcha_half("1212"), 6);
    }

    #[test]
    fn test_rev_captcha_half1221() {
        assert_eq!(reverse_captcha_half("1221"), 0);
    }

    #[test]
    fn test_rev_captcha_half_long1() {
        assert_eq!(reverse_captcha_half("123123"), 12);
    }

    #[test]
    fn test_rev_captcha_half_long2() {
        assert_eq!(reverse_captcha_half("12131415"), 4);
    }
}
