use std::sync::Arc;

use crossbeam::scope;
use crossbeam_channel::{unbounded, Receiver, Sender};
use parking_lot::Mutex;

use self::Action::{Nothing, Store, Terminate};
use self::Inst::{Add, Jgz, Mod, Mul, Rcv, Set, Snd};
use super::Result;

type Memory = [i64; 256];

#[derive(Clone, Copy, Debug)]
pub struct Reg(u8);

impl Reg {
    fn parse(input: &str) -> Reg {
        Reg(input.chars().next().expect("empty string") as u8)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum RegVal {
    Reg(u8),
    Val(i64),
}

impl RegVal {
    fn parse(input: &str) -> RegVal {
        use self::RegVal::{Reg, Val};

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

#[derive(Clone, Copy, Debug)]
pub enum Inst {
    Set(Reg, RegVal),
    Mul(Reg, RegVal),
    Add(Reg, RegVal),
    Mod(Reg, RegVal),
    Jgz(RegVal, RegVal),
    Snd(RegVal),
    Rcv(Reg),
}

pub fn parse(input: &str) -> Result<Vec<Inst>> {
    let mut out = Vec::new();
    for mut words in input.trim().lines().map(str::split_whitespace) {
        let inst = words.next().expect("no instruction");
        let inst = match inst {
            "jgz" => {
                let cond = RegVal::parse(words.next().expect("no register"));
                let arg = RegVal::parse(words.next().expect("no argument"));
                Inst::Jgz(cond, arg)
            }
            "snd" => {
                let arg = RegVal::parse(words.next().expect("no argument"));
                Inst::Snd(arg)
            }
            "rcv" => {
                let reg = Reg::parse(words.next().expect("no argument"));
                Inst::Rcv(reg)
            }
            bin => {
                let reg = Reg::parse(words.next().expect("no register"));
                let arg = RegVal::parse(words.next().expect("no argument"));
                match bin {
                    "set" => Inst::Set(reg, arg),
                    "mul" => Inst::Mul(reg, arg),
                    "add" => Inst::Add(reg, arg),
                    "mod" => Inst::Mod(reg, arg),
                    _ => bail!("unkown instruction: {}", bin),
                }
            }
        };

        out.push(inst);
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
            mem: [0; 256],
            inst,
            ip: 0,
            channel,
        };

        if let Some(id) = program.channel.id() {
            program.mem[b'p' as usize] = i64::from(id);
        }

        program
    }

    pub fn exec(&mut self) {
        loop {
            let it = self.inst[self.ip];

            match it {
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
                        Terminate => return,
                    }
                }
            }
            self.ip += 1;
        }
    }
}

struct Duet {
    sent: i64,
}

impl Duet {
    fn new() -> Duet {
        Duet { sent: 0 }
    }
}

impl Channel for Duet {
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

pub fn duet(inst: &[Inst]) -> i64 {
    let mut p = Program::from_inst(inst, Duet::new());
    p.exec();
    p.channel.sent
}

struct ThreadDuet {
    id: u8,
    sent: u64,
    sender: Sender<Action>,
    receiver: Receiver<Action>,
    blocked: Arc<Mutex<bool>>,
}

impl ThreadDuet {
    fn new(
        id: u8,
        sender: Sender<Action>,
        receiver: Receiver<Action>,
        blocked: Arc<Mutex<bool>>,
    ) -> Self {
        ThreadDuet {
            id,
            sent: 0,
            sender,
            receiver,
            blocked,
        }
    }
}

impl Channel for ThreadDuet {
    fn id(&mut self) -> Option<u8> {
        Some(self.id)
    }

    fn snd(&mut self, val: i64) {
        self.sent += 1;
        let mut blocked = self.blocked.lock();
        *blocked = false;
        let _ = self.sender.send(Store(val));
    }

    fn rcv(&mut self, _: i64) -> Action {
        {
            let mut blocked = self.blocked.lock();
            if self.receiver.is_empty() {
                if *blocked {
                    let _ = self.sender.send(Terminate);
                    return Terminate;
                } else {
                    *blocked = true;
                }
            }
        }

        match self.receiver.recv() {
            Some(action) => action,
            _ => Terminate,
        }
    }
}

pub fn thread_duet(inst: &[Inst]) -> u64 {
    let (tx0, rx0) = unbounded();
    let (tx1, rx1) = unbounded();

    let s0 = Arc::new(Mutex::new(false));
    let s1 = Arc::clone(&s0);

    let mut p0 = Program::from_inst(inst, ThreadDuet::new(0, tx0, rx1, s0));
    let mut p1 = Program::from_inst(inst, ThreadDuet::new(1, tx1, rx0, s1));

    scope(|scope| {
        scope.spawn(|| p0.exec());
        scope.spawn(|| p1.exec());
    });

    p1.channel.sent
}

pub fn solve() -> Result<()> {
    let input = super::get_input()?;
    let inst = parse(&input)?;
    let first = duet(&inst);
    let second = thread_duet(&inst);

    println!("Day 18:\nPart 1: {}\nPart 2: {}\n", first, second);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let inst = parse(
            "set a 1\nadd a 2\nmul a a\nmod a 5\nsnd a\nset a 0\nrcv a\njgz a -1\nset a 1\njgz a \
             -2",
        ).unwrap();
        assert_eq!(duet(&inst), 4)
    }

    #[test]
    fn test_second() {
        let inst = parse("snd 1\nsnd 2\nsnd p\nrcv a\nrcv b\nrcv c\nrcv d").unwrap();
        assert_eq!(thread_duet(&inst), 3);
    }

    const FULL: &str = include_str!("../../data/d18-test");

    #[test]
    fn test_second_full() {
        let inst = parse(FULL).unwrap();
        assert_eq!(thread_duet(&inst), 7112)
    }
}
