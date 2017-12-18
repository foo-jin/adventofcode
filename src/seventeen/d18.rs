use std::sync::mpsc::{Receiver, Sender, channel};
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
enum Instr {
    Set(u8, RegVal),
    Mul(u8, RegVal),
    Add(u8, RegVal),
    Mod(u8, RegVal),
    Jgz(RegVal, RegVal),
    Snd(RegVal),
    Rcv(u8)
}

enum Action {
    Nothing,
    Halt,
    Store(i64),
}

trait Fragment {
    fn init(&mut self) -> Option<i64> {
        None
    }

    fn snd(&mut self, val: i64);

    fn rcv(&mut self, val: i64) -> Action;
}

struct Program<F> {
    mem: Memory,
    instr: Vec<Instr>,
    ip: usize,
    fragment: F,
}

impl<F> Program<F>
where
    F: Fragment,
{
    fn from_instr(instr: Vec<Instr>, fragment: F) -> Program<F> {
        let mut program = Program {
            mem: vec![0; 256],
            instr: instr,
            ip: 0,
            fragment: fragment,
        };

        if let Some(id) = program.fragment.init() {
            program.mem['p' as u8 as usize] = id;
        }

        program
    }

    pub fn run(&mut self) {
        use self::Instr::*;

        loop {
            let it = self.instr.get(self.ip).expect("ip overflow");

            match *it {
                Set(ref reg, ref arg) => {
                    self.mem[*reg as usize] = arg.value(&self.mem);
                },
                Mul(ref reg, ref arg) => {
                    self.mem[*reg as usize] *= arg.value(&self.mem);
                },
                Add(ref reg, ref arg) => {
                    self.mem[*reg as usize] += arg.value(&self.mem);
                },
                Mod(ref reg, ref arg) => {
                    self.mem[*reg as usize] %= arg.value(&self.mem);
                },
                Jgz(ref cond, ref offset) => {
                    let cond = cond.value(&self.mem);
                    if cond > 0 {
                        let o = offset.value(&self.mem);

                        if o < 0 {
                            self.ip = self.ip.checked_sub(-o as usize).expect("underflow");
                        } else {
                            self.ip = self.ip.checked_sub(o as usize).expect("overflow");
                        }

                        continue;
                    }
                },
                Snd(ref arg) => {
                    let val = arg.value(&self.mem);
                    self.fragment.snd(val);
                }
                Rcv(ref reg) => {
                    let val = self.mem[*reg as usize];

                    match self.fragment.rcv(val) {
                        Halt => return,
                        Store(val) => self.mem[*reg as usize] = val,
                        Nothing => ()
                    }
                }
            }
        }
    }
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

fn parse(input: &str) -> Vec<Instr> {
    let mut out = Vec::new();

    for line in input.lines() {
        let mut it = line.split_whitespace();

        match it.next().expect("no instruction") {
            "set" => {
                let reg = reg(it.next().expect("no register"));
                let arg = parse_regval(it.next().expect("no argument"));
                out.push(Instr::Set(reg, arg));
            }
            "mul" => {
                let reg = reg(it.next().expect("no register"));
                let arg = parse_regval(it.next().expect("no argument"));
                out.push(Instr::Mul(reg, arg));
            }
            "add" => {
                let reg = reg(it.next().expect("no register"));
                let arg = parse_regval(it.next().expect("no argument"));
                out.push(Instr::Add(reg, arg));
            }
            "mod" => {
                let reg = reg(it.next().expect("no register"));
                let arg = parse_regval(it.next().expect("no argument"));
                out.push(Instr::Mod(reg, arg));
            }
            "jgz" => {
                let cond = parse_regval(it.next().expect("no register"));
                let arg = parse_regval(it.next().expect("no argument"));
                out.push(Instr::Jgz(cond, arg));
            }
            "snd" => {
                let arg = parse_regval(it.next().expect("no argument"));
                out.push(Instr::Snd(arg));
            }
            "rcv" => {
                let reg = reg(it.next().expect("no argument"));
                out.push(Instr::Rcv(reg));
            }
            inst => panic!("unkown instruction: {}", inst),
        }
    }

    out
}

struct Part1 {
    sent: i64,
}

impl Fragment for Part1 {
    fn snd(&mut self, val: i64) {
        self.sent = val;
    }

    fn rcv(&mut self, val: i64) -> Action {
        if val != 0 && self.sent > 0 {
            Halt
        } else {
            Nothing
        }
    }
}

fn part1(input: &str) -> i64 {
    let instr = parse(input);

    let mut program = Program::from_instr(instr, Part1 {sent: 0});
    program.run();

    program.fragment.sent
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
            receiver
        }
    }
}

impl Fragment for Part2 {
    fn init(&mut self) -> Option<i64> {
        Some(self.id)
    }

    fn snd(&mut self, val: i64) {
        self.send += 1;
        println!("({}) send: {}", self.id, self.send);
        self.sender.send(val).expect("no receiver");
    }

    fn rcv(&mut self, _: i64) -> Action {
        use std::sync::mpsc::TryRecvError;

        match self.receiver.try_recv() {
            Ok(val) => {
                self.recv += 1;
                println!("({}) receive: {}", self.id, self.recv);
                Store(val)
            }
            Err(TryRecvError::Empty) => Halt,
            Err(e) => panic!("unexpected error: {}", e),
        }
    }
}

fn part2(input: &str) -> u64 {
    let inst = parse(input);
    let (tx0, rx0) = channel();
    let (tx1, rx1) = channel();

    let mut p0 = Program::from_instr(inst.clone(), Part2::new(0, tx0, rx1));
    let mut p1 = Program::from_instr(inst.clone(), Part2::new(1, tx1, rx0));

    let mut attempts = 0;

    loop {
        println!("run");
        p0.run();
        p1.run();
        println!("ran");
        if p0.fragment.send == p1.fragment.recv && p1.fragment.send == p0.fragment.recv {
            if attempts > 3 {
                return p1.fragment.send;
            }

            attempts += 1;
        } else {
            attempts = 0;
        }
    }
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
