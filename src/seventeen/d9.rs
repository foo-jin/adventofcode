pub fn count_groups(input: &str) -> u32 {
    let mut nesting = 0;
    let mut count = 0;
    let mut chars = input.chars();

    while let Some(c) = chars.next() {
        match c {
            '!' => {
                chars.next();
            }
            '<' => while let Some(c) = chars.next() {
                match c {
                    '!' => {
                        chars.next();
                    }
                    '>' => break,
                    _ => count += 1,
                }
            },
            '{' => {
                nesting += 1;
                count += nesting;
            }
            '}' => nesting -= 1,
            ',' => (),
            c => panic!("unexpected input: {}", c),
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use seventeen::d9::*;


    #[test]
    fn test_count_groups1() {
        assert_eq!(count_groups("<>"), 0);
    }

    #[test]
    fn test_count_groups2() {
        assert_eq!(count_groups("<random characters>"), 17);
    }

    #[test]
    fn test_count_groups3() {
        assert_eq!(count_groups("<{o\"i!a,<{i<a>"), 10);
    }

    #[test]
    fn test_count_groups4() {
        assert_eq!(count_groups("<<<<>"), 3);
    }

    #[test]
    fn test_count_groups5() {
        assert_eq!(count_groups("<{!>}>"), 2);
    }

    #[test]
    fn test_count_groups6() {
        assert_eq!(count_groups("<!!>"), 0);
    }

    #[test]
    fn test_count_groups7() {
        assert_eq!(count_groups("<!!!>>"), 0);
    }
}
