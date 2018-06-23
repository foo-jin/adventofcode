mod parsing;

use self::parsing::parse_rooms;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Room {
    counts: HashMap<char, u32>,
    sector: u32,
    checksum: [char; 5],
}

impl Room {
    fn is_real(&self) -> bool {
        let counts = self
            .counts
            .iter()
            .sorted_by(|(char1, val1), (char2, val2)| val2.cmp(val1).then(char1.cmp(char2)))
            .into_iter()
            .map(|(c, _x)| *c)
            .collect::<Vec<char>>();

        &counts[..5] == &self.checksum[..]
    }
}

fn sector_sum(rooms: &[Room]) -> u32 {
    rooms
        .into_iter()
        .filter(|r| r.is_real())
        .map(|r| r.sector)
        .sum()
}

fn shift_name(name: &str, s: u32) -> String {
    const OFFSET: u8 = 97;
    let parts = name.split('-').collect::<Vec<&str>>();
    let name = parts[..parts.len() - 1].join("-");
    name.chars()
        .map(|c| match c {
            '-' => ' ',
            c => {
                let c = c as u8 - OFFSET;
                let mut c = c as u32 + s;
                c %= 26;
                (c as u8 + OFFSET) as char
            }
        })
        .collect::<String>()
}

fn find_storage(input: &str, rooms: &[Room]) -> Option<u32> {
    let names = input.lines().map(|l| &l[..l.len() - 7]);
    rooms
        .into_iter()
        .zip(names)
        .map(|(r, name)| (r, shift_name(name, r.sector)))
        .find(|(_r, name)| name.contains("north"))
        .map(|(r, _name)| r.sector)
}

pub fn solve() -> ::Result<()> {
    let input = ::get_input()?;
    let rooms = parse_rooms(&input)?;
    let part1 = sector_sum(&rooms);
    let part2 = find_storage(&input, &rooms).unwrap();

    ::print_output(4, part1, part2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_real_room() {
        use self::parsing::parse_room;

        assert!(parse_room("aaaaa-bbb-z-y-x-123[abxyz]").unwrap().is_real());
        assert!(parse_room("a-b-c-d-e-f-g-h-987[abcde]").unwrap().is_real());
        assert!(
            !parse_room("totally-real-room-200[decoy]")
                .unwrap()
                .is_real()
        );
        assert!(parse_room("not-a-real-room-404[oarel]").unwrap().is_real());
    }

    #[test]
    fn simple_sector_sum() {
        let input = parse_rooms("aaaaa-bbb-z-y-x-123[abxyz]\n\
                     a-b-c-d-e-f-g-h-987[abcde]\n\
                     not-a-real-room-404[oarel]\n\
                     totally-real-room-200[decoy]\n").unwrap();
        let result = sector_sum(&input);
        assert_eq!(result, 1514)
    }

    #[test]
    fn simple_name_shift() {
        assert_eq!(
            shift_name("qzmt-zixmtkozy-ivhz-343", 343),
            "very encrypted name"
        )
    }
}
