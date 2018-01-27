use fnv::FnvHashMap;

use super::Result;

fn eval(input: &str) -> (i32, i32) {
    let mut env = FnvHashMap::default();
    let mut max = 0;

    for l in input.trim().lines() {
        let mut tokens = l.split_whitespace();

        let reg = tokens.next().unwrap();
        let op = tokens.next().unwrap();
        let val: i32 = tokens.next().unwrap().parse().unwrap();

        tokens.next();

        let regc = tokens.next().unwrap();
        let cmp = tokens.next().unwrap();
        let valc: i32 = tokens.next().unwrap().parse().unwrap();

        let clone_env = env.clone();

        let &regval = clone_env.get(reg).unwrap_or(&0);
        env.insert(reg, regval);

        let &regvalc = clone_env.get(regc).unwrap_or(&0);
        env.insert(regc, regvalc);

        let cmp = match cmp {
            ">" => regvalc > valc,
            "<" => regvalc < valc,
            ">=" => regvalc >= valc,
            "<=" => regvalc <= valc,
            "==" => regvalc == valc,
            "!=" => regvalc != valc,
            op => panic!("unexpected operator: {}", op),
        };

        if cmp {
            let result = match op {
                "inc" => regval + val,
                "dec" => regval - val,
                op => panic!("unexpected operator: {}", op),
            };

            env.insert(reg, result);

            if result > max {
                max = result;
            }
        }
    }

    let first = env.values().max().unwrap();
    let second = max;
    (*first, second)
}

pub fn solve() -> Result<()> {
    let program = super::get_input()?;
    let (first, second) = eval(&program);

    println!(
        "Day 8:\n\
         Part 1: {}\n\
         Part 2: {}\n",
        first, second
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const IN: &str =
        "b inc 5 if a > 1\na inc 1 if b < 5\nc dec -10 if a >= 1\nc inc -20 if c == 10";

    #[test]
    fn test_both() {
        assert_eq!(eval(IN), (1, 10));
    }

    use test::Bencher;
    const FULL: &str = include_str!("../../data/d8-test");

    #[bench]
    fn bench_both(b: &mut Bencher) {
        b.iter(|| assert_eq!(eval(FULL), (4163, 5347)))
    }
}
