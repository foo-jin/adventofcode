use md5;
use std::fmt;

const ZEROS: &str = "00000";

struct HexSlice<'a>(&'a [u8]);

impl<'a> HexSlice<'a> {
    fn new<T>(data: &'a T) -> HexSlice<'a>
    where
        T: ?Sized + AsRef<[u8]> + 'a,
    {
        HexSlice(data.as_ref())
    }
}

impl<'a> fmt::Display for HexSlice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}


fn bruteforce(door_id: &str) -> String {
    (0..)
        .map(|i| format!("{}{}", door_id, i))
        .map(md5::compute)
        .map(|digest| HexSlice::new(&*digest).to_string())
        .filter(|digest| &digest[..5] == ZEROS)
        .map(|digest| digest.chars().nth(5).unwrap())
        .take(8)
        .collect()
}

fn ordered_bruteforce(door_id: &str) -> String {
    use itertools::Itertools;
    let mut positions = hashset! { '0', '1', '2', '3', '4', '5', '6', '7'};

    (0..)
        .map(|i| format!("{}{}", door_id, i))
        .map(md5::compute)
        .map(|digest| HexSlice::new(&*digest).to_string())
        .filter_map(|digest| {
            let mut chars = digest.chars();
            let position = chars.nth(5)?;
            if &digest[..5] == ZEROS && positions.remove(&position) {
                let c = chars.next()?;
                Some((position, c))
            } else {
                None
            }
        })
        .take(8)
        .sorted_by_key(|(p, _c)| *p)
        .into_iter()
        .map(|(_p, c)| c)
        .collect()
}


pub fn solve() -> ::Result<()> {
    let input = ::get_input()?;
    let door_id = input.trim();

    let part1 = bruteforce(door_id);
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

    #[test]
    fn complex_bruteforce() {
        let door_id = "abc";
        assert_eq!(ordered_bruteforce(door_id), "05ace8e3")
    }
}
