use fnv::{FnvHashMap, FnvHashSet};

use super::Result;

type Graph = FnvHashMap<u32, Vec<u32>>;

pub fn parse_graph(input: &str) -> Result<Graph> {
    input
        .trim()
        .lines()
        .map(|l| {
            let mut it = l.split("<->");
            let program = it.next().expect("left").trim().parse()?;
            let neighbors: Vec<u32> = it.next()
                .expect("right")
                .trim()
                .split(", ")
                .map(|s| s.parse().map_err(Into::into))
                .collect::<Result<_>>()?;

            Ok((program, neighbors))
        })
        .collect()
}

pub fn process_pipegraph(mut graph: Graph) -> (u32, u32) {
    let mut size = 0;
    let mut count = 0;
    let n = graph.keys().len() as u32;

    for i in 0..n {
        if !graph.contains_key(&i) {
            continue;
        }

        let mut stack = vec![i];
        let mut connected = FnvHashSet::default();
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

    (size, count)
}

pub fn solve() -> Result<()> {
    let input = super::get_input()?;
    let graph = parse_graph(&input)?;
    let (first, second) = process_pipegraph(graph);

    println!(
        "Day 12:\n\
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
    fn test_both() {
        let graph = parse_graph(
            "0 <-> 2\n\
             1 <-> 1\n\
             2 <-> 0, 3, 4\n\
             3 <-> 2, 4\n\
             4 <-> 2, 3, 6\n\
             5 <-> 6\n\
             6 <-> 4, 5",
        ).unwrap();
        assert_eq!(process_pipegraph(graph), (6, 2));
    }
}
