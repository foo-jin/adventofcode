use failure::Error;
use std::collections::HashMap;

fn exec(input: &str) -> Result<i64, Error> {
    let mut it = input.lines().enumerate();
    let mut freq = 0;
    let mut mem: HashMap<&str, i64> = HashMap::new();

    while let Some((i, line)) =  it.next() {
        let mut line = line.split_whitespace();
        let instruction = line.next().unwrap();
        match instruction {
            "snd" => {
                let reg = line.next().unwrap();
                freq = if let Some(val) = mem.get(&reg) {
                    *val
                } else {
                    0
                };
            },
            "set" => {
                let r1 = line.next().unwrap();
                let r2 = line.next().unwrap();
                let val = r2.parse().unwrap_or(*mem.get(&r2).unwrap_or(&0));
                mem.insert(r1, val);
            },
            "add" => {
                let r1 = line.next().unwrap();
                let r2 = line.next().unwrap();
                let &v1 = mem.get(&r1).unwrap_or(&0);
                let v2 = r2.parse().unwrap_or(*mem.get(&r2).unwrap_or(&0));
                mem.insert(r1, v1 + v2);
            },
            "mul" => {
                let r1 = line.next().unwrap();
                let r2 = line.next().unwrap();
                let &v1 = mem.get(&r1).unwrap_or(&0);
                let v2 = r2.parse().unwrap_or(*mem.get(&r2).unwrap_or(&0));
                mem.insert(r1, v1 * v2);
            },
            "mod" => {
                let r1 = line.next().unwrap();
                let r2 = line.next().unwrap();
                let &v1 = mem.get(&r1).unwrap_or(&0);
                let v2 = r2.parse().unwrap_or(*mem.get(&r2).unwrap_or(&0));
                let result = v1 % v2;
                mem.insert(r1, result);
            },
            "rcv" => {
                let reg = line.next().unwrap();
                if let Some(val) = mem.get(&reg) {
                    if *val != 0 {
                        break;
                    }
                }
            },
            "jgz" => {
                let r1 = line.next().unwrap();
                let r2 = line.next().unwrap();
                let &v1 = mem.get(&r1).unwrap_or(&0);
                let v2 = r2.parse().unwrap_or(*mem.get(&r2).unwrap_or(&0));
                if v1 > 0 {
                    if v2 > 1 {
                        for _ in 0..v2 {
                            it.next();
                        }
                    } else {
                        it = input.lines().enumerate();
                        let offset = (i as i64 + v2) as usize;
                        for _ in 0 .. offset {
                            it.next();
                        }
                    }
                }
            },
            _ => panic!("invalid input")
        }
    }

    Ok(freq)
}

pub fn run(input: &str) -> Result<i64, Error> {
    exec(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec() {
        let input = "set a 1\nadd a 2\nmul a a\nmod a 5\nsnd a\nset a 0\nrcv a\njgz a -1\nset a 1\njgz a -2";
        assert_eq!(exec(input).unwrap(), 4);
    }
}
