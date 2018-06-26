fn is_palindrome(cs: &[char]) -> bool {
    let mid = (cs.len() + 1) / 2;
    cs.iter().take(mid).eq(cs.iter().rev().take(mid))
}

fn is_abba(cs: &[char]) -> bool {
    cs[0] != cs[1] && is_palindrome(cs)
}

fn verify_tls(address: &str) -> bool {
    let chars = address.chars().collect::<Vec<char>>();
    let mut hypernet = false;
    let mut valid = false;
    for window in chars.windows(4) {
        if window.contains(&'[') {
            hypernet = true;
        } else if window.contains(&']') {
            hypernet = false;
        } else if is_abba(window) {
            match hypernet {
                true => return false,
                false => valid = true,
            }
        }
    }
    valid
}

fn count_tls(addresses: &str) -> usize {
    addresses.trim().lines().filter(|a| verify_tls(a)).count()
}

fn mirrors(a: &str, b: &str) -> bool {
    let ac = a.chars().collect::<Vec<char>>();
    let bc = b.chars().collect::<Vec<char>>();
    ac[0] == bc[1] && bc[0] == ac[1]
}

fn verify_ssl(addr: &str) -> bool {
    let chars = addr.chars().collect::<Vec<char>>();
    let mut hypernet = false;
    let mut aba: Vec<String> = Vec::new();
    let mut bab: Vec<String> = Vec::new();

    for window in chars.windows(3) {
        if window.contains(&'[') {
            hypernet = true;
        } else if window.contains(&']') {
            hypernet = false;
        } else if is_abba(window) {
            let s = window.iter().collect::<String>();
            let (same, other) = match hypernet {
                true => (&mut bab, &aba),
                false => (&mut aba, &bab),
            };

            if other.iter().any(|abab| mirrors(&s, &abab)) {
                return true;
            }

            same.push(s);
        }
    }

    false
}

fn count_ssl(addresses: &str) -> usize {
    addresses.trim().lines().filter(|a| verify_ssl(a)).count()
}

pub fn solve() -> ::Result<()> {
    let input = ::get_input()?;

    info!("Solving part 1");
    let part1 = count_tls(&input);

    info!("Solving part 2");
    let part2 = count_ssl(&input);

    ::print_output(6, part1, part2)
}

#[cfg(test)]
mod test {
    use super::*;

    fn check_tls(addr: &str, expected: bool) {
        assert_eq!(verify_tls(addr), expected);
    }

    #[test]
    fn tls_verification() {
        check_tls("abba[mnop]qrst", true);
        check_tls("abcd[bddb]xyyx", false);
        check_tls("aaaa[qwer]tyui", false);
        check_tls("ioxxoj[asdfgh]zxcvbn", true);
    }

    fn check_ssl(addr: &str, expected: bool) {
        assert_eq!(verify_ssl(addr), expected)
    }

    #[test]
    fn ssl_verification() {
        check_ssl("aba[bab]xyz", true);
        check_ssl("xyx[xyx]xyx", false);
        check_ssl("aaa[kek]eke", true);
        check_ssl("zazbz[bzb]cdb", true);
    }
}
