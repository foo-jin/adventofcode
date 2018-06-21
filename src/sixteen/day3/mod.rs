mod parsing;

use self::parsing::parse_triangles;
use super::Result;

type Triangle = [u16; 3];

fn is_valid(t: &&Triangle) -> bool {
    izip!(t.iter(), t.iter().cycle().skip(1), t.iter().cycle().skip(2)).all(|(a, b, c)| a + b > *c)
}

fn count_valid(triangles: &[Triangle]) -> u32 {
    triangles.into_iter().filter(is_valid).count() as u32
}

pub fn solve() -> Result<()> {
    let input = ::get_input()?;
    let triangles = parse_triangles(&input)?;
    let part1 = count_valid(&triangles);

    ::print_output(3, part1, 2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_triangle() {
        assert!(!is_valid(&&[5,10,25]))
    }
}
