 
 use std::collections::{HashSet, HashMap};

pub fn pipegraph(input: &str) -> u32 {
    let mut graph = HashMap::new();
    let mut connected = HashSet::new();

    for l in input.trim().lines() {
        let mut it = l.split_whitespace();
        let first: u32 = it.next().unwrap().parse().unwrap();
        let _ = it.next();

        let mut isconn = connected.contains(&first);
        let mut last = Vec::new();
        for s in it {
            let k: u32 = s.chars().filter(|&c| c != ',').collect::<String>().parse().unwrap();
            if isconn {
                connected.insert(k);
                println!("inserting {}", k);
            } else if connected.contains(&k) {
                isconn = true;

                for n in last.clone() {
                    println!("inserting {}", n);
                    connected.insert(n);
                }
            } else {
                last.push(k);
            }
        }
    }

    connected.len() as u32
}

 #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipegraph1() {
        let input = "0 <-> 2\n1 <-> 1\n2 <-> 0, 3, 4\n3 <-> 2, 4\n4 <-> 2, 3, 6\n5 <-> 6\n6 <-> 4, 5";

        assert_eq!(pipegraph(input), 6);
    }
}