use nom::{line_ending, not_line_ending, types::CompleteStr as Input};
use std::collections::HashMap;

fn parse_messages(s: &str) -> ::Result<Vec<Vec<char>>> {
    named!(message(Input) -> Vec<char>, map!(not_line_ending, |w| w.chars().collect()));
    named!(lines(Input) -> Vec<Vec<char>>, separated_list!(line_ending, message));

    lines(Input(s))
        .map(|(_rest, result)| result)
        .map_err(|e| format_err!("failed to parse input: {}", e))
}

fn recover_message(msg: &[Vec<char>], use_modified_code: bool) -> String {
    let n = msg[0].len();
    let mut counts = vec![HashMap::new(); n];
    msg.into_iter()
        .flat_map(|msg| msg.iter().enumerate())
        .for_each(|(i, c)| *counts[i].entry(c).or_insert(0) += 1);

    let sign = if use_modified_code { 1 } else { -1 };
    counts
        .into_iter()
        .flat_map(|occurrences| {
            occurrences
                .into_iter()
                .max_by_key(|(_c, v)| *v * sign)
                .map(|(c, _v)| c)
        })
        .collect()
}

pub fn solve() -> ::Result<()> {
    let input = ::get_input()?;
    let messages = parse_messages(&input)?;

    info!("Solving part 1");
    let part1 = recover_message(&messages, true);

    info!("Solving part 2");
    let part2 = recover_message(&messages, false);

    ::print_output(6, part1, part2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn message_parsing() {
        let input = "eedadn\ndrvtee";
        let expected = vec![
            vec!['e', 'e', 'd', 'a', 'd', 'n'],
            vec!['d', 'r', 'v', 't', 'e', 'e'],
        ];

        ::check(parse_messages(input), expected);
    }

    const INPUT: &str = "eedadn\n\
                    drvtee\n\
                    eandsr\n\
                    raavrd\n\
                    atevrs\n\
                    tsrnev\n\
                    sdttsa\n\
                    rasrtv\n\
                    nssdts\n\
                    ntnada\n\
                    svetve\n\
                    tesnvt\n\
                    vntsnd\n\
                    vrdear\n\
                    dvrsen\n\
                    enarar";
    #[test]
    fn message_recovery() {
        let messages = parse_messages(INPUT).unwrap();
        let expected = "easter";
        assert_eq!(&recover_message(&messages, true), expected)
    }

    #[test]
    fn flipped_message_recovery() {
        let messages = parse_messages(INPUT).unwrap();
        let expected = "advent";
        assert_eq!(&recover_message(&messages, false), expected)
    }
}
