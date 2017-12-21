
pub fn escape_maze(input: &str) -> u32 {
    let mut lines: Vec<i32> = input.lines().map(|s| s.parse::<i32>().unwrap()).collect();

    let n = lines.len();
    let mut i = 0;
    let mut j = 0;
    while i >= 0 && i < n as i32 {
        let el = &mut lines[i as usize];
        i += *el;
        if (*el) >= 3 {
            *el -= 1;
        } else {
            *el += 1;
        }
        j += 1;
    }
    j
}

#[cfg(test)]
mod tests {
    use seventeen::d5::*;

    #[test]
    fn test_escape_maze1() {
        assert_eq!(escape_maze("0\n3\n0\n1\n-3"), 10);
    }
}
