use std::str;

use nom::{alphanumeric, space};

named!(name<&str>, map_res!(alphanumeric, str::from_utf8));

named!(
    weight<u32>,
    map_res!(
        map_res!(
            delimited!(char!('('), is_not!(")"), char!(')')),
            str::from_utf8
        ),
        str::parse
    )
);

named!(child_sep, complete!(tag!(" -> ")));

named!(
    children<Vec<&str>>,
    separated_list_complete!(tag!(", "), name)
);

named!(
    pub line<(&str, u32, Vec<&str>)>,
    do_parse!(n: name >> opt!(space) >> w: weight >> opt!(child_sep) >> c: children >> (n, w, c))
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_sample() {
        assert_eq!(name(&b"pbga (66)"[..]), Ok((&b" (66)"[..], "pbga")));
    }

    #[test]
    fn weight_sample() {
        assert_eq!(weight(&b"(66)"[..]), Ok((&b""[..], 66u32)));
    }

    #[test]
    fn children_sample() {
        assert_eq!(
            children(&b"ktlj, cntj, xhth"[..]),
            Ok((&b""[..], vec!["ktlj", "cntj", "xhth"]))
        );
    }

    #[test]
    fn no_children_sample() {
        assert_eq!(children(&b""[..]), Ok((&b""[..], vec![])));
    }

    #[test]
    fn line_with_children() {
        assert_eq!(
            line(&b"fwft (72) -> ktlj, cntj, xhth"[..]),
            Ok((&b""[..], ("fwft", 72, vec!["ktlj", "cntj", "xhth"])))
        );
    }

    #[test]
    fn line_without_children() {
        assert_eq!(
            line(&b"pbga (66)"[..]),
            Ok((&b""[..], ("pbga", 66, vec![])))
        );
    }
}
