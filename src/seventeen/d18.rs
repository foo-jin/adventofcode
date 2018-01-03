use std::sync::{Arc, Mutex};

use crossbeam::scope;
use crossbeam_channel::{unbounded, Receiver, Sender};

use super::Result;
use self::Action::*;
use self::Inst::{Add, Jgz, Mod, Mul, Rcv, Set, Snd};

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

        if let Ok(v) = input.parse::<i64>() {
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
    Mul(Reg, RegVal),
    Add(Reg, RegVal),
    Mod(Reg, RegVal),
    Jgz(RegVal, RegVal),
    Snd(RegVal),
    Rcv(Reg),
}

fn parse(input: &str) -> Result<Vec<Inst>> {
    let mut out = Vec::new();

    for mut words in input.trim().lines().map(str::split_whitespace) {
        let inst = words.next().expect("no instruction");
        match inst {
            "jgz" => {
                let cond = RegVal::parse(words.next().expect("no register"));
                let arg = RegVal::parse(words.next().expect("no argument"));
                out.push(Inst::Jgz(cond, arg));
            }
            "snd" => {
                let arg = RegVal::parse(words.next().expect("no argument"));
                out.push(Inst::Snd(arg));
            }
            "rcv" => {
                let reg = Reg::parse(words.next().expect("no argument"));
                out.push(Inst::Rcv(reg));
            }
            bin => {
                let reg = Reg::parse(words.next().expect("no register"));
                let arg = RegVal::parse(words.next().expect("no argument"));

                match bin {
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
                    _ => bail!("unkown instruction: {}", bin),
                }
            }
        }
    }

    Ok(out)
}

#[derive(Clone)]
enum Action {
    Store(i64),
    Nothing,
    Terminate,
}

impl Action {
    fn is_store(&self) -> bool {
        match self {
            Store(_) => true,
            _ => false,
        }
    }
}

trait Channel {
    fn id(&mut self) -> Option<u8> {
        None
    }

    fn snd(&mut self, val: i64);

    fn rcv(&mut self, cond: i64) -> Action;
}

#[derive(Clone)]
struct Program<'a, C> {
    mem: Memory,
    inst: &'a [Inst],
    ip: usize,
    channel: C,
}

impl<'a, C> Program<'a, C>
where
    C: Channel,
{
    fn from_inst(inst: &'a [Inst], channel: C) -> Self {
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

    pub fn exec(&mut self) {
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
                    use self::{Nothing, Store, Terminate};
                    let val = self.mem[reg as usize];
                    match self.channel.rcv(val) {
                        Store(val) => self.mem[reg as usize] = val,
                        Nothing => (),
                        Terminate => break,
                    }
                }
            }
            self.ip += 1;
        }
    }
}

struct First {
    sent: i64,
}

impl First {
    fn new() -> First {
        First { sent: 0 }
    }
}

impl Channel for First {
    fn id(&mut self) -> Option<u8> {
        None
    }

    fn snd(&mut self, val: i64) {
        self.sent = val
    }

    fn rcv(&mut self, cond: i64) -> Action {
        if cond == 0 {
            Nothing
        } else {
            Terminate
        }
    }
}

fn first(input: &str) -> Result<i64> {
    let inst = parse(input)?;
    let mut p = Program::from_inst(&inst, First::new());
    p.exec();
    Ok(p.channel.sent)
}

#[derive(Debug)]
struct Second {
    id: u8,
    sent: u64,
    recv: u64,
    sender: Sender<Action>,
    receiver: Receiver<Action>,
    blocked: Arc<Mutex<bool>>,
}

impl Second {
    fn new(
        id: u8,
        sender: Sender<Action>,
        receiver: Receiver<Action>,
        blocked: Arc<Mutex<bool>>,
    ) -> Second {
        Second {
            id,
            sent: 0,
            recv: 0,
            sender,
            receiver,
            blocked,
        }
    }

    fn snd_term(&self) {
        let _ = self.sender.send(Terminate);
    }
}

impl Channel for Second {
    fn id(&mut self) -> Option<u8> {
        Some(self.id)
    }

    fn snd(&mut self, val: i64) {
        self.sent += 1;
        let mut blocked = self.blocked.lock().unwrap();
        *blocked = false;
        let _ = self.sender.send(Store(val));
    }

    fn rcv(&mut self, _: i64) -> Action {
        {
            let mut blocked = self.blocked.lock().unwrap();

            if self.receiver.is_empty() {
                if *blocked {
                    self.snd_term();
                    return Terminate;
                } else {
                    *blocked = true;
                }
            }
        }

        match self.receiver.recv() {
            Ok(action) => {
                if action.is_store() {
                    self.recv += 1;
                }
                action
            }
            _ => Terminate,
        }
    }
}

fn second(input: &str) -> Result<u64> {
    let inst = parse(input)?;

    let (tx0, rx0) = unbounded();
    let (tx1, rx1) = unbounded();

    let s0 = Arc::new(Mutex::new(false));
    let s1 = s0.clone();

    let mut p0 = Program::from_inst(&inst, Second::new(0, tx0, rx1, s0));
    let mut p1 = Program::from_inst(&inst, Second::new(1, tx1, rx0, s1));

    scope(|scope| {
        scope.spawn(|| p0.exec());
        scope.spawn(|| p1.exec());
    });

    Ok(p1.channel.sent)
}

pub fn run(input: &str) -> Result<u64> {
    second(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use seventeen::check;

    #[test]
    fn test_first() {
        let input = "set a 1\nadd a 2\nmul a a\nmod a 5\nsnd a\nset a 0\nrcv a\njgz a -1\nset a 1\njgz a -2";
        check(first(input), 4)
    }

    #[test]
    fn test_second() {
        let input = "snd 1\nsnd 2\nsnd p\nrcv a\nrcv b\nrcv c\nrcv d";
        check(second(input), 3);
    }

    use test::Bencher;
    const FULL: &str = include_str!("../../data/d18-test");

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        b.iter(|| check(first(FULL), 3188))
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        b.iter(|| check(second(FULL), 7112))
    }
}
