use super::Direction;
use nom::{self, types::CompleteStr as Input};

named!(up(Input) -> Direction, value!(Direction::Up, char!('U')));
named!(down(Input) -> Direction, value!(Direction::Down, char!('D')));
named!(left(Input) -> Direction, value!(Direction::Left, char!('L')));
named!(right(Input) -> Direction, value!(Direction::Right, char!('R')));

named!(direction(Input) -> Direction, alt!(up | down | left | right));

named!(line(Input) -> Vec<Direction>, many1!(direction));

pub fn parse_line(s: &str) -> Result<Vec<Direction>, nom::Err<Input>> {
    line(Input(s)).map(|(_rest, result)| result)
}

#[cfg(test)]
mod test {
    use super::Direction::*;
    use super::*;

    #[test]
    fn simple_directions() {
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
}
