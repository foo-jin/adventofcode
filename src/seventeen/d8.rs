use std::collections::HashMap;

enum Operator {
    inc,
    dec
}

enum Comp {
    gt,
    lt,
    geq,
    leq,
    eq,
    neq,
}

pub fn largest_register(input: &str) -> i32 {
    let mut env = HashMap::new();
    let mut max = 0;

    for l in input.lines() {
        let mut tokens = l.split_whitespace();
        let reg = tokens.next().unwrap();
        let clone_env = env.clone();
        let regval = clone_env.get(reg);
        let regval = match regval {
            Some(val) => val,
            None => {
                env.insert(reg, 0);
                &0
            },
        };

        use seventeen::d8::Operator::*;
        let op = match tokens.next().unwrap() {
            "inc" => inc,
            "dec" => dec,
            _ => panic!("faulty operator!"),
        };

        let val: i32 = tokens.next().unwrap().parse().unwrap();

        tokens.next();

        let c_reg = tokens.next().unwrap();
        let clone_env = env.clone();
        let c_regval = clone_env.get(c_reg);
        let &c_regval = match c_regval {
            Some(val) => val,
            None => {
                env.insert(c_reg, 0);
                &0
            },
        };

        use seventeen::d8::Comp::*;
        let comp = match tokens.next().unwrap() {
            ">" => gt,
            "<" => lt,
            ">=" => geq,
            "<=" => leq,
            "==" => eq,
            "!=" => neq,
            _ => panic!("faulty bool operator")
        };

        let c_val: i32 = tokens.next().unwrap().parse().unwrap();

        let c_result = match comp {
            gt => c_regval > c_val,
            lt => c_regval < c_val,
            geq => c_regval >= c_val,
            leq => c_regval <= c_val,
            eq => c_regval == c_val,
            neq => c_regval != c_val,
        };

        if c_result {
            let result = match op {
                inc => regval + val,
                dec => regval - val,
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
    const input: &str = "b inc 5 if a > 1\na inc 1 if b < 5\nc dec -10 if a >= 1\nc inc -20 if c == 10";

    #[test]
    fn test_largest_register() {
        assert_eq!(largest_register(input), 10);
    }
}