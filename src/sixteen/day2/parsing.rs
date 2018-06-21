use super::Direction;
use nom::types::CompleteStr as Input;

named!(up(Input) -> Direction, value!(Direction::Up, char!('U')));
named!(down(Input) -> Direction, value!(Direction::Down, char!('D')));
named!(left(Input) -> Direction, value!(Direction::Left, char!('L')));
named!(right(Input) -> Direction, value!(Direction::Right, char!('R')));

named!(direction(Input) -> Direction, alt!(up | down | left | right));

named!(line(Input) -> Vec<Direction>, many1!(direction));

named!(lines(Input) -> Vec<Vec<Direction>>, separated_list!(tag!("\n"), line));

pub fn parse_directions(s: &str) -> super::Result<Vec<Vec<Direction>>> {
    lines(Input(s))
        .map(|(_rest, result)| result)
        .map_err(|e| format_err!("failed to parse input: {}", e))
}

#[cfg(test)]
mod test {
    use super::Direction::*;
    use super::*;

    #[test]
    fn simple_directions<'a>() {
        assert_eq!(direction(Input("U")), Ok(("".into(), Up)));
        assert_eq!(direction(Input("D")), Ok(("".into(), Down)));
        assert_eq!(direction(Input("L")), Ok(("".into(), Left)));
        assert_eq!(direction(Input("R")), Ok(("".into(), Right)));
    }

    #[test]
    fn simple_line() {
        let input = Input("UDLLRRDU");
        assert_eq!(
            line(input),
            Ok((
                "".into(),
                vec![Up, Down, Left, Left, Right, Right, Down, Up]
            ))
        );
    }

    #[test]
    fn simple_lines() {
        let input = Input("UDLR\nUDLR\n");
        assert_eq!(
            lines(input),
            Ok(("\n".into(), vec![vec![Up, Down, Left, Right]; 2]))
        )
    }
}
