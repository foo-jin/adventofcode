use std::sync::mpsc::{channel, Receiver, Sender};
use failure::Error;
use self::Action::*;

type Memory = Vec<i64>;

#[derive(Debug, Clone)]
enum RegVal {
    Reg(u8),
    Val(i64),
}

impl RegVal {
    pub fn value(&self, mem: &Memory) -> i64 {
        use self::RegVal::*;

        match *self {
            Reg(reg) => mem[reg as usize],
            Val(v) => v,
        }
    }
}

#[derive(Debug, Clone)]
enum Inst {
    Set(u8, RegVal),
    Mul(u8, RegVal),
    Add(u8, RegVal),
    Mod(u8, RegVal),
    Jgz(RegVal, RegVal),
    Snd(RegVal),
    Rcv(u8),
}

fn reg(input: &str) -> u8 {
    input.chars().next().expect("empty string") as u8
}

fn parse_regval(input: &str) -> RegVal {
    if let Ok(v) = input.parse::<i64>() {
        RegVal::Val(v)
    } else {
        let c = input.chars().next().expect("empty string");
        RegVal::Reg(c as u8)
    }
}

fn parse(input: &str) -> Vec<Inst> {
    let mut out = Vec::new();

    for line in input.lines() {
        let mut it = line.split_whitespace();

        match it.next().expect("no instruction") {
            "set" => {
                let reg = reg(it.next().expect("no register"));
                let arg = parse_regval(it.next().expect("no argument"));
                out.push(Inst::Set(reg, arg));
            }
            "mul" => {
                let reg = reg(it.next().expect("no register"));
                let arg = parse_regval(it.next().expect("no argument"));
                out.push(Inst::Mul(reg, arg));
            }
            "add" => {
                let reg = reg(it.next().expect("no register"));
                let arg = parse_regval(it.next().expect("no argument"));
                out.push(Inst::Add(reg, arg));
            }
            "mod" => {
                let reg = reg(it.next().expect("no register"));
                let arg = parse_regval(it.next().expect("no argument"));
                out.push(Inst::Mod(reg, arg));
            }
            "jgz" => {
                let cond = parse_regval(it.next().expect("no register"));
                let arg = parse_regval(it.next().expect("no argument"));
                out.push(Inst::Jgz(cond, arg));
            }
            "snd" => {
                let arg = parse_regval(it.next().expect("no argument"));
                out.push(Inst::Snd(arg));
            }
            "rcv" => {
                let reg = reg(it.next().expect("no argument"));
                out.push(Inst::Rcv(reg));
            }
            inst => panic!("unkown instruction: {}", inst),
        }
    }

    out
}

#[derive(Clone)]
enum Action {
    Running,
    Blocked,
    Store(i64),
}

trait Channel {
    fn id(&mut self) -> Option<i64> {
        None
    }

    fn snd(&mut self, val: i64);

    fn rcv(&mut self, val: i64) -> Action;
}

#[derive(Clone)]
struct Program<C> {
    mem: Memory,
    state: Action,
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
            state: Running,
            inst: inst,
            ip: 0,
            channel: channel,
        };

        if let Some(id) = program.channel.id() {
            program.mem['p' as u8 as usize] = id;
        }

        program
    }

    fn blocked(&self) -> bool {
        match self.state {
            Blocked => true,
            _ => false,
        }
    }

    pub fn exec(&mut self) {
        use self::Inst::*;

        let it = self.inst.get(self.ip).expect("ip overflow");

        match *it {
            Set(ref reg, ref arg) => {
                self.mem[*reg as usize] = arg.value(&self.mem);
            }
            Mul(ref reg, ref arg) => {
                self.mem[*reg as usize] *= arg.value(&self.mem);
            }
            Add(ref reg, ref arg) => {
                self.mem[*reg as usize] += arg.value(&self.mem);
            }
            Mod(ref reg, ref arg) => {
                self.mem[*reg as usize] %= arg.value(&self.mem);
            }
            Jgz(ref cond, ref offset) => {
                let cond = cond.value(&self.mem);
                if cond > 0 {
                    let o = offset.value(&self.mem);

                    if o < 0 {
                        self.ip = self.ip.checked_sub(-o as usize).expect("underflow");
                    } else {
                        self.ip = self.ip.checked_add(o as usize).expect("overflow");
                    }
                    return;
                }
            }
            Snd(ref arg) => {
                let val = arg.value(&self.mem);
                self.channel.snd(val);
            }
            Rcv(ref reg) => {
                let val = self.mem[*reg as usize];
                self.state = self.channel.rcv(val);
                match self.state {
                    Blocked => return,
                    Store(val) => self.mem[*reg as usize] = val,
                    Running => (),
                }
            }
        }
        self.ip += 1;
    }
}



struct Part1 {
    sent: i64,
}

impl Channel for Part1 {
    fn snd(&mut self, val: i64) {
        self.sent = val;
    }

    fn rcv(&mut self, val: i64) -> Action {
        if val != 0 && self.sent > 0 {
            Blocked
        } else {
            Running
        }
    }
}

fn part1(input: &str) -> i64 {
    let inst = parse(input);

    let mut program = Program::from_inst(inst, Part1 { sent: 0 });
    program.exec();

    program.channel.sent
}

#[derive(Debug)]
struct Part2 {
    id: i64,
    send: u64,
    recv: u64,
    sender: Sender<i64>,
    receiver: Receiver<i64>,
}

impl Part2 {
    fn new(id: i64, sender: Sender<i64>, receiver: Receiver<i64>) -> Part2 {
        Part2 {
            id,
            send: 0,
            recv: 0,
            sender,
            receiver,
        }
    }
}

impl Channel for Part2 {
    fn id(&mut self) -> Option<i64> {
        Some(self.id)
    }

    fn snd(&mut self, val: i64) {
        self.send += 1;
        self.sender.send(val).expect("no receiver");
    }

    fn rcv(&mut self, _: i64) -> Action {
        use std::sync::mpsc::TryRecvError;

        match self.receiver.try_recv() {
            Ok(val) => {
                self.recv += 1;
                Store(val)
            }
            Err(TryRecvError::Empty) => Blocked,
            Err(e) => panic!("unexpected error: {}", e),
        }
    }
}

fn part2(input: &str) -> u64 {
    let inst = parse(input);
    let (tx0, rx0) = channel();
    let (tx1, rx1) = channel();

    let mut p0 = Program::from_inst(inst.clone(), Part2::new(0, tx0, rx1));
    let mut p1 = Program::from_inst(inst.clone(), Part2::new(1, tx1, rx0));

    while !(p0.blocked() && p1.blocked()) {
        p0.exec();
        p1.exec();
    }

    p1.channel.send
}

pub fn run(input: &str) -> u64 {
    part2(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec() {
        let input = "snd 1\nsnd 2\nsnd p\nrcv a\nrcv b\nrcv c\nrcv d";
        assert_eq!(part2(input), 3);
    }
}
