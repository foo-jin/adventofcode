
use failure::Error;

pub fn run(input: &str) -> Result<usize, Error> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use seventeen::check;

    #[test]
    fn test_first() {
        let input = "p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>\np=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>";
        let result = run(input);
        let expected = 0;
        check(result, expected);
    }
}