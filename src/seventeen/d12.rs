use std::collections::{HashMap, HashSet};

use super::Result;

pub fn run(input: &str) -> Result<(u32, u32)> {
    let mut graph = HashMap::new();
    let mut len = 0;

    for l in input.trim().lines() {
        len += 1;
        let mut it = l.split("<->");
        let key: u32 = it.next().expect("left").trim().parse()?;
        let right: Vec<u32> = it.next()
            .expect("right")
            .trim()
            .split(", ")
            .map(|s| s.parse().map_err(Into::into))
            .collect::<Result<_>>()?;

        graph.insert(key, right);
    }

    let mut size = 0;
    let mut count = 0;

    for i in 0..len {
        if !graph.contains_key(&i) {
            continue;
        }

        let mut stack = vec![i];
        let mut connected = HashSet::new();

        while let Some(cur) = stack.pop() {
            connected.insert(cur);

            if let Some(ks) = graph.remove(&cur) {
                stack.extend(ks);
            }
        }

        if connected.contains(&0) {
            size = connected.len() as u32;
        }

        count += 1;
    }

    Ok((size, count))
}

#[cfg(test)]
mod tests {
    use super::*;
    use seventeen::check;

    #[test]
    fn test_both() {
        let input =
            "0 <-> 2\n1 <-> 1\n2 <-> 0, 3, 4\n3 <-> 2, 4\n4 <-> 2, 3, 6\n5 <-> 6\n6 <-> 4, 5";
        check(run(input), (6, 2));
    }

    use test::Bencher;
    const FULL: &str = include_str!("../../data/d12-test");

    #[bench]
    fn bench_both(b: &mut Bencher) {
        b.iter(|| check(run(FULL), (128, 209)))
    }
}
