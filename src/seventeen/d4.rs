use std::collections::HashSet;

pub fn check_password(input: &str) -> u32 {
    let mut result = 0;

    'outer: for l in input.lines() {
        let mut seen = HashSet::new();

        for s in l.split_whitespace() {
            if !seen.insert(s) {
                continue 'outer;
            }
        }

        result += 1;
    }

    result
}

pub fn check_anagram(input: &str) -> u32 {
    let mut result = 0;

    'outer: for l in input.lines() {
        let mut seen = HashSet::new();

        for s in l.split_whitespace() {
            let mut chars = s.chars().collect::<Vec<char>>();
            chars.sort_unstable();

            if !seen.insert(chars) {
                continue 'outer;
            }
        }
        
        result += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use seventeen::d4::*;

    #[test]
    fn test_check_password1() {
        assert_eq!(check_password("aa bb cc dd ee"), 1);
    }

    #[test]
    fn test_check_password2() {
        assert_eq!(check_password("aa bb cc dd aa"), 0);
    }

    #[test]
    fn test_check_password3() {
        assert_eq!(check_password("aa bb cc dd aaa"), 1);
    }

    #[test]
    fn test_check_anagram1() {
        assert_eq!(check_anagram("abcde fghij"), 1);
    }

    #[test]
    fn test_check_anagram2() {
        assert_eq!(check_anagram("abcde xyz ecdab"), 0);
    }

    #[test]
    fn test_check_anagram3() {
        assert_eq!(check_anagram("a ab abc abd abf abj"), 1);
    }

    #[test]
    fn test_check_anagram4() {
        assert_eq!(check_anagram("iiii oiii ooii oooi oooo"), 1);
    }

    #[test]
    fn test_check_anagram5() {
        assert_eq!(check_anagram("oiii ioii iioi iiio"), 0);
    }
}
