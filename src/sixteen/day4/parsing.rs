use super::Room;
use nom::{anychar, digit, line_ending, types::CompleteStr as Input};
use std::collections::HashMap;

fn count_chars<T: IntoIterator<Item = char>>(input: T) -> HashMap<char, u32> {
    let mut counter = HashMap::new();
    input
        .into_iter()
        .filter(|c| *c != '-')
        .for_each(|c| *counter.entry(c).or_insert(0u32) += 1);
    counter
}

named!(char_count_and_sector(Input) -> (HashMap<char, u32>, u32),
       map!(
           many_till!(
               anychar,
                 map_res!(digit,|d: Input| d.parse())
               ), |(chars, sector)| (count_chars(chars), sector))
       );

named!(checksum(Input) -> [char; 5], delimited!(char!('['), count_fixed!(char, anychar, 5), char!(']')));

named!(pub room(Input) -> Room, do_parse!(estring: char_count_and_sector >> check: checksum >> (estring.0, estring.1, check)));

named!(lines(Input) -> Vec<Room>, separated_list!(line_ending, room));

pub fn parse_rooms(s: &str) -> ::Result<Vec<Room>> {
    lines(Input(s))
        .map(|(_rest, result)| result)
        .map_err(|e| format_err!("failed to parse input: {}", e))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_room() {
        let input = Input("aaaaa-bbb-z-y-x-123[abxyz]");
        let expected = Ok((
            "".into(),
            (
                hashmap!{
                    'a' => 5,
                    'b' => 3,
                    'z' => 1,
                    'y' => 1,
                    'x' => 1,
                },
                123,
                ['a', 'b', 'x', 'y', 'z'],
            ),
        ));
        assert_eq!(room(input), expected);
    }
}
