mod parsing;

use super::Result;
use self::parsing::parse_triangles;

type Triangle = [u16; 3];

fn solve() -> Result<()> {
    let input = ::get_input()?;
    let triangles = parse_triangles(&input)?;

    ::print_output(3, 1, 2)
}
