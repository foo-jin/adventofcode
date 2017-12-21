use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;
use failure::*;

use self::Action::*;
use self::Inst::*;

type Memory = Vec<i64>;

#[derive(Debug, Clone)]
struct Reg(u8);

impl Reg {
    fn from_str(input: &str) -> Reg {
        Reg(input.chars().next().expect("empty string") as u8)
    }
}

#[derive(Debug, Clone)]
enum RegVal {
    Reg(u8),
    Val(i64),
}

impl RegVal {
    fn from_str(input: &str) -> RegVal {
        if let Ok(v) = input.parse::<i64>() {
            RegVal::Val(v)
        } else {
            let c = input.chars().next().expect("empty string");
            RegVal::Reg(c as u8)
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
    Mul(Reg, RegVal),
    Add(Reg, RegVal),
    Mod(Reg, RegVal),
    Jgz(RegVal, RegVal),
    Snd(RegVal),
    Rcv(Reg),
}

fn parse(input: &str) -> Result<Vec<Inst>, Error> {
    let mut out = Vec::new();

    for line in input.lines() {
        let mut it = line.split_whitespace();
        let inst = it.next().expect("no instruction");
        match inst {
            "jgz" => {
                let cond = RegVal::from_str(it.next().expect("no register"));
                let arg = RegVal::from_str(it.next().expect("no argument"));
                out.push(Inst::Jgz(cond, arg));
            }
            "snd" => {
                let arg = RegVal::from_str(it.next().expect("no argument"));
                out.push(Inst::Snd(arg));
            }
            "rcv" => {
                let reg = Reg::from_str(it.next().expect("no argument"));
                out.push(Inst::Rcv(reg));
            }
            inst => {
                let reg = Reg::from_str(it.next().expect("no register"));
                let arg = RegVal::from_str(it.next().expect("no argument"));
                match inst {
                    "set" => {
                        out.push(Inst::Set(reg, arg));
                    }
                    "mul" => {
                        out.push(Inst::Mul(reg, arg));
                    }
                    "add" => {
                        out.push(Inst::Add(reg, arg));
                    }
                    "mod" => {
                        out.push(Inst::Mod(reg, arg));
                    }
                    _ => bail!("unkown instruction: {}", inst),
                }
            }
        }
    }

    Ok(out)
}

#[derive(Clone)]
enum Action {
    Store(i64),
    Terminate,
}

trait Channel {
    fn id(&mut self) -> Option<u8> {
        None
    }

    fn snd(&mut self, val: i64);

    fn rcv(&mut self, val: i64) -> Action;
}

#[derive(Clone)]
struct Program<C> {
    mem: Memory,
    inst: Vec<Inst>,
    ip: usize,
    channel: C,
}

impl<C> Program<C>
where
    C: Channel,
{
    fn from_inst(inst: Vec<Inst>, channel: C) -> Program<C> {
        let mut program = Program {
            mem: vec![0; 256],
            inst: inst,
            ip: 0,
            channel: channel,
        };

        if let Some(id) = program.channel.id() {
            program.mem[b'p' as usize] = i64::from(id);
        }

        program
    }

    pub fn exec(mut self) -> Program<C> {
        loop {
            let it = self.inst.get(self.ip).expect("ip overflow");

            match *it {
                Set(Reg(reg), ref arg) => {
                    self.mem[reg as usize] = arg.eval(&self.mem);
                }
                Mul(Reg(reg), ref arg) => {
                    self.mem[reg as usize] *= arg.eval(&self.mem);
                }
                Add(Reg(reg), ref arg) => {
                    self.mem[reg as usize] += arg.eval(&self.mem);
                }
                Mod(Reg(reg), ref arg) => {
                    self.mem[reg as usize] %= arg.eval(&self.mem);
                }
                Jgz(ref cond, ref offset) => {
                    let cond = cond.eval(&self.mem);
                    if cond > 0 {
                        let o = offset.eval(&self.mem);

                        if o < 0 {
                            self.ip = self.ip.checked_sub(-o as usize).expect("underflow");
                        } else {
                            self.ip = self.ip.checked_add(o as usize).expect("overflow");
                        }
                        continue;
                    }
                }
                Snd(ref arg) => {
                    let val = arg.eval(&self.mem);
                    self.channel.snd(val);
                }
                Rcv(Reg(reg)) => {
                    let val = self.mem[reg as usize];
                    let state = self.channel.rcv(val);
                    match state {
                        Store(val) => self.mem[reg as usize] = val,
                        Terminate => break,
                    }
                }
            }
            self.ip += 1;
        }
        self
    }
}

#[derive(Debug)]
struct Second {
    id: u8,
    sent: u64,
    recv: u64,
    sender: Sender<i64>,
    receiver: Receiver<i64>,
}

impl Second {
    fn new(id: u8, sender: Sender<i64>, receiver: Receiver<i64>) -> Second {
        Second {
            id,
            sent: 0,
            recv: 0,
            sender,
            receiver,
        }
    }
}

impl Channel for Second {
    fn id(&mut self) -> Option<u8> {
        Some(self.id)
    }

    fn snd(&mut self, val: i64) {
        self.sent += 1;
        self.sender.send(val).expect("no receiver");
    }

    fn rcv(&mut self, _: i64) -> Action {
        match self.receiver.recv_timeout(Duration::new(2, 0)) {
            Ok(val) => {
                self.recv += 1;
                Store(val)
            }
            _ => Terminate,
        }
    }
}

fn part2(input: &str) -> Result<u64, Error> {
    let inst = parse(input)?;
    
    let (tx0, rx0) = channel();
    let (tx1, rx1) = channel();

    let p0 = Program::from_inst(inst.clone(), Second::new(0, tx0, rx1));
    let p1 = Program::from_inst(inst.clone(), Second::new(1, tx1, rx0));

    let h0 = thread::spawn(move || {
        p0.exec()
    });

    let h1 = thread::spawn(move || {
        p1.exec()
    });

    let _ = h0.join();
    let p1 = h1.join().expect("error in thread p1");

    Ok(p1.channel.sent)
}

pub fn run(input: &str) -> Result<u64, Error> {
    part2(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec1() {
        let input = "snd 1\nsnd 2\nsnd p\nrcv a\nrcv b\nrcv c\nrcv d";
        assert_eq!(part2(input).unwrap(), 3);
    }

    #[test]
    fn test_exec2() {
        let input = include_str!("../../data/d18-test");
        assert_eq!(part2(input).unwrap(), 7112)
    }
}
