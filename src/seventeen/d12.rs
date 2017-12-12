use std::collections::{HashSet, HashMap};
use failure::Error;

pub fn pipegraph(input: &str) -> Result<u32, Error> {
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
            .collect::<Result<_, Error>>()?;

        graph.insert(key, right);
    }

    let mut components = Vec::new();
    for i in 0..len {
        if let None = graph.get(&i) {
            continue;
        }

        let mut stack = vec![i];
        let mut connected = HashSet::new();

        while let Some(cur) = stack.pop() {
            if connected.contains(&cur) {
                continue;
            } else {
                connected.insert(cur);
                if let Some(ks) = graph.get(&cur) {
                    stack.extend(ks);
                }
            }
            graph.remove(&cur);
        }
        components.push(connected);
    }

    Ok(components.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipegraph() {
        let input = "0 <-> 2\n1 <-> 1\n2 <-> 0, 3, 4\n3 <-> 2, 4\n4 <-> 2, 3, 6\n5 <-> 6\n6 <-> 4, 5";

        assert_eq!(pipegraph(input).expect("failed"), 2);
    }
}