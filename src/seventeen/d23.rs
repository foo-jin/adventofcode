use failure::*;

use super::Result;
use self::Inst::{Jnz, Mul, Set, Sub};

type Memory = Vec<i64>;

#[derive(Debug, Clone)]
struct Reg(u8);

impl Reg {
    fn parse(input: &str) -> Reg {
        Reg(input.chars().next().expect("empty string") as u8)
    }
}

#[derive(Debug, Clone)]
enum RegVal {
    Reg(u8),
    Val(i64),
}

impl RegVal {
    fn parse(input: &str) -> RegVal {
        use self::RegVal::*;

        if let Ok(v) = input.parse() {
            Val(v)
        } else {
            let c = input.chars().next().expect("empty string");
            Reg(c as u8)
        }
    }

    fn eval(&self, mem: &[i64]) -> i64 {
        use self::RegVal::*;

        match *self {
            Reg(reg) => mem[reg as usize],
            Val(v) => v,
        }
    }
}

#[derive(Debug, Clone)]
enum Inst {
    Set(Reg, RegVal),
    Sub(Reg, RegVal),
    Mul(Reg, RegVal),
    Jnz(RegVal, RegVal),
}

impl Inst {
    fn parse(line: &str) -> Result<Inst> {
        let mut it = line.split_whitespace();
        let inst = it.next().ok_or(err_msg("no instruction"))?;
        let result = match inst {
            "jnz" => {
                let cond = RegVal::parse(it.next().ok_or(err_msg("no register"))?);
                let arg = RegVal::parse(it.next().ok_or(err_msg("no argument"))?);
                Jnz(cond, arg)
            }
            inst => {
                let reg = Reg::parse(it.next().ok_or(err_msg("no register"))?);
                let arg = RegVal::parse(it.next().ok_or(err_msg("no argument"))?);
                match inst {
                    "set" => Set(reg, arg),
                    "sub" => Sub(reg, arg),
                    "mul" => Mul(reg, arg),
                    _ => bail!("unkown instruction: {}", inst),
                }
            }
        };

        Ok(result)
    }
}

fn parse_inst(input: &str) -> Result<Vec<Inst>> {
    input
        .trim()
        .lines()
        .map(Inst::parse)
        .collect::<Result<Vec<Inst>>>()
}

#[derive(Debug, Clone)]
struct Counter {
    set: u32,
    sub: u32,
    mul: u32,
    jnz: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            set: 0,
            sub: 0,
            mul: 0,
            jnz: 0,
        }
    }

    fn set(&mut self) {
        self.set += 1;
    }
    fn sub(&mut self) {
        self.sub += 1;
    }

    fn mul(&mut self) {
        self.mul += 1;
    }

    fn jnz(&mut self) {
        self.jnz += 1;
    }
}

#[derive(Clone)]
struct Program {
    mem: Memory,
    inst: Vec<Inst>,
    ip: usize,
    count: Counter,
}

impl Program {
    fn from_inst(inst: Vec<Inst>) -> Program {
        let mem = vec![0; 256];
        Program {
            mem,
            inst: inst,
            ip: 0,
            count: Counter::new(),
        }
    }

    pub fn exec(&mut self) {
        while let Some(it) = self.inst.get(self.ip) {
            match *it {
                Set(Reg(reg), ref arg) => {
                    self.count.set();
                    self.mem[reg as usize] = arg.eval(&self.mem);
                }
                Sub(Reg(reg), ref arg) => {
                    self.count.sub();
                    self.mem[reg as usize] -= arg.eval(&self.mem);
                }
                Mul(Reg(reg), ref arg) => {
                    self.count.mul();
                    self.mem[reg as usize] *= arg.eval(&self.mem);
                }
                Jnz(ref cond, ref offset) => {
                    self.count.jnz();
                    let cond = cond.eval(&self.mem);
                    if cond != 0 {
                        let o = offset.eval(&self.mem);

                        if o < 0 {
                            self.ip = self.ip.checked_sub(-o as usize).expect("underflow");
                        } else {
                            self.ip = self.ip.checked_add(o as usize).expect("overflow");
                        }
                        continue;
                    }
                }
            }
            self.ip += 1;
        }
    }
}

fn first(input: &str) -> Result<u32> {
    let inst = parse_inst(input)?;
    let mut program = Program::from_inst(inst);
    program.exec();

    Ok(program.count.mul)
}

fn second(input: &str) -> Result<u64> {
    let mut b = input.split_whitespace().nth(2).unwrap().parse()?;
    b = b * 100 + 100_000;
    let mut h = 0;
    let end = b + 17_000 + 1;

    for x in (b..end).step_by(17) {
        for i in 2..x {
            if x % i == 0 {
                h += 1;
                break;
            }
        }
    }

    Ok(h)
}

pub fn run(input: &str) -> Result<u64> {
    second(input)
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    use seventeen::check;

    use test::Bencher;
    const FULL: &str = include_str!("../../data/d23-test");

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        b.iter(|| check(first(FULL), 5929))
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        b.iter(|| check(second(FULL), 907))
    }
}
