use std::collections::HashMap;

pub fn redistribute(input: &str) -> u32 {
    let mut input: Vec<u32> = input
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let n = input.len();
    let mut seen = HashMap::new();
    let mut result = 0;

    for i in 0.. {
        let mut clone = input.clone();
        {
            let (mut key, el) = input
                .iter()
                .cloned()
                .enumerate()
                .max_by_key(|&(i, v)| (v, -(i as i32)))
                .unwrap();

            input[key] = 0;
            key += 1;

            for _ in 0..el {
                input[key % n] += 1;
                key += 1;
            }
        }

        if let Some(x) = seen.insert(clone, i) {
            result = i - x;
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use seventeen::d6::*;

    #[test]
    fn test_redistribute() {
        assert_eq!(redistribute("0\t2\t7\t0"), 4);
    }

    #[test]
    fn test_redistribute_full() {
        let input = include_str!("../../data/d6-test");
        assert_eq!(redistribute(input), 8038);
    }
}
