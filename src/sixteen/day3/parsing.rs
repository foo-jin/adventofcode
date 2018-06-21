use super::Triangle;
use nom::{alphanumeric, digit, line_ending, space, types::CompleteStr as Input};
use std::str;

named!(triangle(Input) -> Triangle, count_fixed!(u16, map_res!(preceded!(space, digit), |d: Input| d.parse()), 3));
named!(lines(Input) -> Vec<Triangle>, separated_list!(line_ending, triangle));

pub fn parse_triangles(s: &str) -> super::Result<Vec<Triangle>> {
    lines(Input(s))
        .map(|(_rest, result)| result)
        .map_err(|e| format_err!("failed to parse input: {}", e))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_triangle() {
        let input = Input("  541  588  421");
        let result = triangle(input);
        assert_eq!(result, Ok(("".into(), [541, 588, 421])));
    }

    #[test]
    fn simple_list() {
        let input = Input("  541  588  421\n  827  272  126");
        let result = lines(input);
        assert_eq!(
            result,
            Ok(("".into(), vec![[541, 588, 421], [827, 272, 126]]))
        );
    }
}
