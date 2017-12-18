use failure::Error;
use std::collections::{HashMap, VecDeque};

enum Status<'a> {
    Run,
    Block(&'a str),
    Term,
}

fn exec(input: &str) -> Result<i64, Error> {
    use super::d18::Status::*;

    let mut it = vec![input.lines().enumerate(), input.lines().enumerate()];
    let mut mem: Vec<HashMap<&str, i64>> = vec![HashMap::new(), HashMap::new()];
    let mut qs = vec![VecDeque::new(), VecDeque::new()];
    let mut status = vec![Run, Run];
    let mut count = vec![0, 0];
    let mut id = 0;

    for i in 0..2 {
        mem[i].insert("p", i as i64);
    }

    loop {
        //println!("id: {}, q0: {}, q1: {}", id, qs[0].len(), qs[1].len());
        match status[id] {
            Block(reg) => {
                if let Some(val) = qs[id].pop_front() {
                    mem[id].insert(reg, val);
                    status[id] = Run
                } else {
                    id = (id + 1) % 2
                }
            },
            Term => id = (id + 1) % 2,
            Run => ()
        }

        match (&status[0], &status[1]) {
            (&Block(_), &Block(_)) => {
                println!("bb");
                break
            },
            (&Term, &Term) => {
                println!("tt");
                break
            },
            (&Term, &Block(_)) => {
                println!("tb");
                break
            },
            (&Block(_), &Term) => {
                println!("bt");
                break
            },
            _ => {
                let (i, line) = match it[id].next() {
                    Some((i, line)) => (i, line),
                    None => {
                        status[id] = Term;
                        id = (id + 1) % 2;
                        continue
                    }
                };

                let mut line = line.split_whitespace();
                let instruction = line.next().unwrap();
                if id == 1 {
                    println!("{}", instruction);
                }
                match instruction {
                    "snd" => {
                        let reg = line.next().unwrap();
                        let val = *mem[id].get(&reg).unwrap_or(&0);
                        qs[(id + 1) % 2].push_back(val);
                        count[id] += 1;
                        if id == 1 {
                            println!("1 sending");
                        }
                    },
                    "set" => {
                        let r1 = line.next().unwrap();
                        let r2 = line.next().unwrap();
                        let val = r2.parse().unwrap_or(*mem[id].get(&r2).unwrap_or(&0));
                        mem[id].insert(r1, val);
                    },
                    "add" => {
                        let r1 = line.next().unwrap();
                        let r2 = line.next().unwrap();
                        let &v1 = mem[id].get(&r1).unwrap_or(&0);
                        let v2 = r2.parse().unwrap_or(*mem[id].get(&r2).unwrap_or(&0));
                        mem[id].insert(r1, v1 + v2);
                    },
                    "mul" => {
                        let r1 = line.next().unwrap();
                        let r2 = line.next().unwrap();
                        let &v1 = mem[id].get(&r1).unwrap_or(&0);
                        let v2 = r2.parse().unwrap_or(*mem[id].get(&r2).unwrap_or(&0));
                        mem[id].insert(r1, v1 * v2);
                    },
                    "mod" => {
                        let r1 = line.next().unwrap();
                        let r2 = line.next().unwrap();
                        let &v1 = mem[id].get(&r1).unwrap_or(&0);
                        let v2 = r2.parse().unwrap_or(*mem[id].get(&r2).unwrap_or(&0));
                        let result = v1 % v2;
                        mem[id].insert(r1, result);
                    },
                    "rcv" => {
                        let reg = line.next().unwrap();
                        if let Some(val) = qs[id].pop_front() {
                            mem[id].insert(reg, val);
                        } else {
                            status[id] = Block(reg);
                            id = (id + 1) % 2;
                        }
                    },
                    "jgz" => {
                        let r1 = line.next().unwrap();
                        let r2 = line.next().unwrap();
                        let &v1 = mem[id].get(&r1).unwrap_or(&0);
                        let v2 = r2.parse().unwrap_or(*mem[id].get(&r2).unwrap_or(&0));
                        if v1 > 0 {
                            if v2 > 1 {
                                for _ in 0..v2 {
                                    it[id].next();
                                }
                            } else {
                                it[id] = input.lines().enumerate();
                                let offset = (i as i64 + v2) as usize;
                                for _ in 0 .. offset {
                                    it[id].next();
                                }
                            }
                        }
                    },
                    _ => panic!("invalid input")
                }
            }
        }
    }

    Ok(count[1])
}

pub fn run(input: &str) -> Result<i64, Error> {
    exec(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec() {
        let input = "snd 1\nsnd 2\nsnd p\nrcv a\nrcv b\nrcv c\nrcv d";
        assert_eq!(exec(input).unwrap(), 3);
    }
}
