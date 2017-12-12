 
 use std::collections::{HashSet, HashMap};

pub fn pipegraph(input: &str) -> u32 {
    let mut graph = HashMap::new();
    let mut len = 0;
    for l in input.trim().lines() {
        len += 1;
        let mut it = l.split_whitespace();
        let key: u32 = it.next().unwrap().parse().unwrap();
        let _ = it.next();

        let mut neigh = Vec::new();
        for s in it {
            let k: u32 = s.chars().filter(|&c| c != ',').collect::<String>().parse().unwrap();
            neigh.push(k);
        }
        graph.insert(key, neigh);
    }

    let mut components = Vec::new();
    for i in 0 .. len {
        if let None = graph.get(&i) {
            continue;
        }

        let mut stack = Vec::new();
        stack.push(i);
        let mut connected = HashSet::new();
        while !stack.is_empty() {
            let cur = stack.pop().unwrap();
            if connected.contains(&cur) {
                continue
            } else {
                connected.insert(cur);
                if let Some(ks) = graph.get(&cur) {
                    for &k in ks {
                        stack.push(k);
                    }
                }
                
            }
            graph.remove(&cur);
        }
        components.push(connected);
    }    
    components.len() as u32
}

 #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipegraph1() {
        let input = "0 <-> 2\n1 <-> 1\n2 <-> 0, 3, 4\n3 <-> 2, 4\n4 <-> 2, 3, 6\n5 <-> 6\n6 <-> 4, 5";

        assert_eq!(pipegraph(input), 2);
    }
}