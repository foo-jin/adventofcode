use nom::{self, alphanumeric, space, types::CompleteStr as Input};
use std::str;

named!(
    weight(Input) -> u32,
    map_res!(
            map!(delimited!(char!('('), is_not!(")"), char!(')')), |w| *w),
        str::parse
    )
);

named!(child_sep(Input) -> Input, complete!(tag!(" -> ")));

named!(
    children(Input) -> Vec<&str>,
    separated_list!(tag!(", "), map!(alphanumeric, |c| *c))
);

named!(
    line(Input) -> (&str, u32, Vec<&str>),
    do_parse!(n: alphanumeric >> opt!(space) >> w: weight >> opt!(child_sep) >> c: children >> (*n, w, c))
);

pub fn parse_line(s: &str) -> Result<(&str, u32, Vec<&str>), nom::Err<Input>> {
    line(Input(s)).map(|(_, r)| r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_sample() {
        assert_eq!(
            alphanumeric(Input("pbga (66)")),
            Ok((Input(" (66)"), Input("pbga")))
        );
    }

    #[test]
    fn weight_sample() {
        assert_eq!(weight(Input("(66)")), Ok((Input(""), 66u32)));
    }

    #[test]
    fn children_sample() {
        assert_eq!(
            children(Input("ktlj, cntj, xhth")),
            Ok((Input(""), vec!["ktlj", "cntj", "xhth"]))
        );
    }

    #[test]
    fn no_children_sample() {
        assert_eq!(children(Input("")), Ok((Input(""), vec![])));
    }

    #[test]
    fn line_with_children() {
        assert_eq!(
            line(Input("fwft (72) -> ktlj, cntj, xhth")),
            Ok(("".into(), ("fwft", 72, vec!["ktlj", "cntj", "xhth"])))
        );
    }

    #[test]
    fn line_without_children() {
        assert_eq!(
            line(Input("pbga (66)")),
            Ok(("".into(), ("pbga", 66, vec![])))
        );
    }
}
