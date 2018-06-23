use failure;
use itertools::Itertools;
use std::{
    collections::HashMap, str::{self, FromStr},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Room {
    name: String,
    counts: HashMap<char, u32>,
    sector: u32,
    checksum: [char; 5],
}

impl Room {
    fn new(encoded_name: &str, sector: u32, checksum: [char; 5]) -> Self {
        let name = shift_name(encoded_name, sector);
        let counts = count_chars(encoded_name.chars());
        Room {
            name,
            counts,
            sector,
            checksum,
        }
    }

    fn is_real(&self) -> bool {
        let counts = self
            .counts
            .iter()
            .sorted_by(|(char1, val1), (char2, val2)| val2.cmp(val1).then(char1.cmp(char2)))
            .into_iter()
            .map(|(c, _x)| *c)
            .collect::<Vec<char>>();

        counts[..5] == self.checksum[..]
    }

    fn parse_many(s: &str) -> ::Result<Vec<Room>> {
        s.trim()
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()
            .map_err(Into::into)
    }
}

fn count_chars<T: IntoIterator<Item = char>>(input: T) -> HashMap<char, u32> {
    let mut counter = HashMap::new();
    input
        .into_iter()
        .filter(|c| *c != '-')
        .for_each(|c| *counter.entry(c).or_insert(0u32) += 1);
    counter
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

fn sector_sum(rooms: &[Room]) -> u32 {
    rooms
        .into_iter()
        .filter(|r| r.is_real())
        .map(|r| r.sector)
        .sum()
}

fn find_storage(rooms: &[Room]) -> Option<u32> {
    rooms
        .into_iter()
        .find(|r| r.name.contains("north"))
        .map(|r| r.sector)
}

pub fn solve() -> ::Result<()> {
    let input = ::get_input()?;
    let rooms = Room::parse_many(&input)?;
    let part1 = sector_sum(&rooms);
    let part2 = find_storage(&rooms).unwrap();

    ::print_output(4, part1, part2)
}

impl FromStr for Room {
    type Err = failure::Error;

    fn from_str(s: &str) -> ::Result<Self> {
        use nom::{anychar, types::CompleteStr as Input};
        use regex::Regex;

        lazy_static! {
            static ref RE: Regex = Regex::new(r#"^([a-z\-]*)-(\d+)\[([a-z]{5})\]$"#).unwrap();
        }

        named!(checksum(Input) -> [char; 5], count_fixed!(char, anychar, 5));

        let caps = RE
            .captures(s)
            .ok_or_else(|| format_err!("\"{}\" is not a valid room", s))?;
        let encoded = &caps[1];
        let sector = caps[2].parse()?;
        let checksum = checksum(Input(&caps[3]))
            .map(|(_rest, result)| result)
            .unwrap();

        Ok(Room::new(encoded, sector, checksum))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn check_realness(s: &str, expected: bool) {
        assert_eq!(s.parse::<Room>().unwrap().is_real(), expected)
    }

    #[test]
    fn simple_room() {
        let input = "aaaaa-bbb-z-y-x-123[abxyz]";
        let expected = Room::new("aaaaa-bbb-z-y-x", 123, ['a', 'b', 'x', 'y', 'z']);
        ::check(input.parse(), expected)
    }

    #[test]
    fn simple_real_room() {
        check_realness("aaaaa-bbb-z-y-x-123[abxyz]", true);
        check_realness("a-b-c-d-e-f-g-h-987[abcde]", true);
        check_realness("totally-real-room-200[decoy]", false);
        check_realness("not-a-real-room-404[oarel]", true);
    }

    #[test]
    fn simple_sector_sum() {
        let input = Room::parse_many("aaaaa-bbb-z-y-x-123[abxyz]\n\
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
