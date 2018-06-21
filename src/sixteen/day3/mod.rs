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

fn count_valid_vertical(triangles: Vec<Triangle>) -> u32 {
    let mut swapped = Vec::with_capacity(triangles.len());
    triangles.exact_chunks(3).for_each(|ts| match ts {
        [t1, t2, t3] => izip!(t1.into_iter(), t2.into_iter(), t3.into_iter())
            .for_each(|(a, b, c)| swapped.push([*a, *b, *c])),
        _ => unreachable!(),
    });
    count_valid(&swapped)
}

pub fn solve() -> Result<()> {
    let input = ::get_input()?;
    let triangles = parse_triangles(&input)?;
    let part1 = count_valid(&triangles);
    let part2 = count_valid_vertical(triangles);

    ::print_output(3, part1, part2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_triangle() {
        assert!(!is_valid(&&[5, 10, 25]))
    }
}
