use std::collections::HashMap;

pub fn largest_register(input: &str) -> i32 {
    let mut env = HashMap::new();
    let mut max = 0;

    for l in input.lines() {
        let mut tokens = l.split_whitespace();

        let reg = tokens.next().unwrap();
        let op = tokens.next().unwrap();
        let val: i32 = tokens.next().unwrap().parse().unwrap();

        tokens.next();

        let regc = tokens.next().unwrap();
        let comp = tokens.next().unwrap();
        let valc: i32 = tokens.next().unwrap().parse().unwrap();

        let clone_env = env.clone();

        let &regval = clone_env.get(reg).unwrap_or(&0);
        env.insert(reg, regval);

        let &regvalc = clone_env.get(regc).unwrap_or(&0);
        env.insert(regc, regvalc);

        let resultc = match comp {
            ">" => regvalc > valc,
            "<" => regvalc < valc,
            ">=" => regvalc >= valc,
            "<=" => regvalc <= valc,
            "==" => regvalc == valc,
            "!=" => regvalc != valc,
            _ => panic!("faulty bool operator"),
        };

        if resultc {
            let result = match op {
                "inc" => regval + val,
                "dec" => regval - val,
                _ => panic!("faulty operator!"),
            };

            env.insert(reg, result);

            if result > max {
                max = result;
            }
        }
    }
    //*env.iter().map(|(k, v)| v).max().unwrap()
    max
}

#[cfg(test)]
mod tests {
    use seventeen::d8::*;
    const IN: &str = "b inc 5 if a > 1\na inc 1 if b < 5\nc dec -10 if a >= 1\nc inc -20 if c == 10";

    #[test]
    fn test_largest_register() {
        assert_eq!(largest_register(IN), 10);
    }
}