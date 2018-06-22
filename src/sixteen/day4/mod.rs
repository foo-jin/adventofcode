mod parsing;

use self::parsing::parse_rooms;
use std::collections::HashMap;

type Room = (HashMap<char, u32>, u32, [char; 5]);

fn is_real_room(r: &Room) -> bool {
    let (counts, _, checksum) = r;
    let mut counts = counts.into_iter().collect::<Vec<_>>();
    counts.sort_by(|(c1, x1), (c2, x2)| x2.cmp(x1).then(c1.cmp(c2)));
    let counts = counts.into_iter().map(|(c, _x)| *c).collect::<Vec<char>>();
    &counts[..5] == &checksum[..]
}

fn sector_sum(rooms: &[Room]) -> u32 {
    rooms
        .into_iter()
        .filter(|r| is_real_room(*r))
        .map(|r| r.1)
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
        .map(|r| r.1)
        .zip(names)
        .map(|(s, name)| (s, shift_name(name, s)))
        .find(|(_s, name)| name.contains("north"))
        .map(|(s, _name)| s)
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
        use self::parsing::room;

        assert!(is_real_room(
            &room("aaaaa-bbb-z-y-x-123[abxyz]".into()).unwrap().1
        ));
        assert!(is_real_room(
            &room("a-b-c-d-e-f-g-h-987[abcde]".into()).unwrap().1
        ));
        assert!(is_real_room(
            &room("not-a-real-room-404[oarel]".into()).unwrap().1
        ));
        assert!(!is_real_room(
            &room("totally-real-room-200[decoy]".into()).unwrap().1
        ))
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
