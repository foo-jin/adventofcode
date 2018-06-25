use crypto::{digest::Digest, md5::Md5};

const OFFSET: usize = '0' as usize;

fn bruteforce(door_id: &str) -> String {
    let mut hasher = Md5::new();
    hasher.input_str(door_id);

    (0..)
        .map(|i| {
            let mut hs = hasher.clone();
            hs.input_str(&i.to_string());
            hs.result_str()
        })
        .filter(|hash| hash.starts_with("00000"))
        .take(8)
        .map(|hash| hash.as_bytes()[5] as char)
        .collect()
}

fn ordered_bruteforce(door_id: &str) -> String {
    let mut password = vec!['_'; 8];
    let mut hasher = Md5::new();
    hasher.input_str(door_id);

    for hash in (0..)
        .map(|i| {
            let mut hs = hasher.clone();
            hs.input_str(&i.to_string());
            hs.result_str()
        })
        .filter(|hash| hash.starts_with("00000"))
    {
        let bytes = hash.as_bytes();
        let i = bytes[5] as usize - OFFSET;
        if password[i] == '_' {
            password[i] = bytes[6] as char;
        }

        if !password.contains(&'_') {
            break;
        }
    }

    password.into_iter().collect()
}


pub fn solve() -> ::Result<()> {
    let input = ::get_input()?;
    let door_id = input.trim();

    info!("Solving part 1");
    let part1 = bruteforce(door_id);

    info!("Solving part 2");
    let part2 = ordered_bruteforce(door_id);

    ::print_output(5, part1, part2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_bruteforce() {
        let door_id = "abc";
        assert_eq!(bruteforce(door_id), "18f47a30")
    }

    // #[test]
    // fn complex_bruteforce() {
    //     let door_id = "abc";
    //     assert_eq!(ordered_bruteforce(door_id), "05ace8e3")
    // }
}
