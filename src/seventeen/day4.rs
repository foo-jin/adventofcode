use fnv::FnvHashSet;

use super::Result;

pub fn check_password(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .filter(|l| {
            let mut seen = FnvHashSet::default();
            l.split_whitespace().all(|s| seen.insert(s))
        })
        .count() as u32
}

pub fn check_anagram(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .filter(|l| {
            let mut seen = FnvHashSet::default();

            l.split_whitespace().all(|s| {
                let mut chars: Vec<char> = s.chars().collect();
                chars.sort_unstable();
                seen.insert(chars)
            })
        })
        .count() as u32
}

pub fn solve() -> Result<()> {
    let passphrases = super::get_input()?;
    let first = check_password(&passphrases);
    let second = check_anagram(&passphrases);

    println!(
        "Day 4:\n\
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
