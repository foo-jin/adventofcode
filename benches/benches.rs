extern crate adventofcode;
#[macro_use]
extern crate criterion;

use criterion::Criterion;

mod day1 {
    use adventofcode::seventeen::day1::*;
    use criterion::Criterion;

    const FULL: &str = include_str!("../data/d1-test");

    pub fn bench_p1(c: &mut Criterion) {
        let input = parse(FULL).unwrap();
        c.bench_function("d1 p1", move |b| {
            b.iter(|| assert_eq!(reverse_captcha(&input), 1049))
        });
    }

    pub fn bench_p2(c: &mut Criterion) {
        let input = parse(FULL).unwrap();
        c.bench_function("d1 p2", move |b| {
            b.iter(|| assert_eq!(reverse_captcha_half(&input), 1508))
        });
    }
}

mod day2 {
    use adventofcode::seventeen::day2::*;
    use criterion::Criterion;

    const FULL: &str = include_str!("../data/d2-test");

    pub fn bench_p1(c: &mut Criterion) {
        let input = parse(FULL).unwrap();
        c.bench_function("d2 p1", move |b| {
            b.iter(|| assert_eq!(checksum(&input), 45_351))
        });
    }

    pub fn bench_p2(c: &mut Criterion) {
        let input = parse(FULL).unwrap();
        c.bench_function("d2 p2", move |b| b.iter(|| assert_eq!(divsum(&input), 275)));
    }
}

mod day3 {
    use adventofcode::seventeen::day3::*;
    use criterion::Criterion;

    pub fn bench_p1(c: &mut Criterion) {
        c.bench_function("d3 p1", |b| b.iter(|| assert_eq!(nthspiral(265_149), 438)));
    }

    pub fn bench_p2(c: &mut Criterion) {
        c.bench_function("d3 p2", |b| {
            b.iter(|| assert_eq!(firstlarger(265_149), 266_330))
        });
    }
}

mod day4 {
    use adventofcode::seventeen::day4::*;
    use criterion::Criterion;

    const FULL: &str = include_str!("../data/d4-test");

    pub fn bench_p1(c: &mut Criterion) {
        c.bench_function("d4 p1", |b| {
            b.iter(|| assert_eq!(check_password(FULL), 466))
        });
    }

    pub fn bench_p2(c: &mut Criterion) {
        c.bench_function("d4 p2", |b| b.iter(|| assert_eq!(check_anagram(FULL), 251)));
    }
}

mod day5 {
    use adventofcode::seventeen::day5::*;
    use criterion::Criterion;

    const FULL: &str = include_str!("../data/d5-test");

    pub fn bench_p1(c: &mut Criterion) {
        c.bench_function("d5 p1", |b| {
            b.iter(|| {
                let mut input = parse_buffer(FULL).unwrap();
                assert_eq!(buffer_jump(&mut input), 373_543)
            })
        });
    }

    pub fn bench_p2(c: &mut Criterion) {
        c.bench_function("d5 p2", |b| {
            b.iter(|| {
                let mut input = parse_buffer(FULL).unwrap();
                assert_eq!(buffer_jump_extreme(&mut input), 27_502_966)
            })
        });
    }
}

mod day6 {
    use adventofcode::seventeen::day6::*;
    use criterion::Criterion;

    const FULL: &str = include_str!("../data/d6-test");

    pub fn bench_both(c: &mut Criterion) {
        let mut input = parse_memory(FULL).unwrap();
        c.bench_function("d6 both", move |b| {
            b.iter(|| assert_eq!(redistribute(&mut input), (12_841, 8038)))
        });
    }
}

mod day7 {
    use adventofcode::seventeen::day7::*;
    use criterion::Criterion;

    const FULL: &str = include_str!("../data/d7-test");

    pub fn bench_p1(c: &mut Criterion) {
        let tree = Tree::from_str(FULL).unwrap();
        c.bench_function("d7 p1", move |b| b.iter(|| assert_eq!(tree.root, "fbgguv")));
    }

    pub fn bench_p2(c: &mut Criterion) {
        let tree = Tree::from_str(FULL).unwrap();
        c.bench_function("d7 p2", move |b| b.iter(|| assert_eq!(tree.solve(), 1864)));
    }
}

mod day8 {
    use adventofcode::seventeen::day8::*;
    use criterion::Criterion;

    const FULL: &str = include_str!("../data/d8-test");

    pub fn bench_both(c: &mut Criterion) {
        c.bench_function("d8 both", |b| {
            b.iter(|| assert_eq!(eval(FULL), (4163, 5347)))
        });
    }
}

mod day9 {
    use adventofcode::seventeen::day9::*;
    use criterion::Criterion;

    const FULL: &str = include_str!("../data/d9-test");

    pub fn bench_both(c: &mut Criterion) {
        c.bench_function("d9 both", |b| {
            b.iter(|| assert_eq!(process_stream(FULL).unwrap(), (12_505, 6671)))
        });
    }
}

mod day10 {
    use adventofcode::seventeen::day10::*;
    use criterion::Criterion;

    const FULL: &str = "199,0,255,136,174,254,227,16,51,85,1,2,22,17,7,192";

    pub fn bench_p2(c: &mut Criterion) {
        c.bench_function("d10 p2", |b| {
            b.iter(|| check_knothash(FULL, "a9d0e68649d0174c8756a59ba21d4dc6"))
        });
    }
}

mod day11 {
    use adventofcode::seventeen::day11::*;
    use criterion::Criterion;

    const FULL: &str = include_str!("../data/d11-test");

    pub fn bench_both(c: &mut Criterion) {
        c.bench_function("d11 both", |b| {
            b.iter(|| assert_eq!(hexgrid(FULL), (824, 1548)))
        });
    }
}

mod day12 {
    use adventofcode::seventeen::day12::*;
    use criterion::Criterion;

    const FULL: &str = include_str!("../data/d12-test");

    pub fn bench_both(c: &mut Criterion) {
        c.bench_function("d12 both", |b| {
            let graph = parse_graph(FULL).unwrap();
            b.iter(|| assert_eq!(process_pipegraph(graph.clone()), (128, 209)))
        });
    }
}

mod day13 {
    use adventofcode::seventeen::day13::*;
    use criterion::Criterion;

    const FULL: &str = include_str!("../data/d13-test");

    pub fn bench_p1(c: &mut Criterion) {
        c.bench_function("d13 p1", |b| {
            let layers = parse_layers(FULL).unwrap();
            b.iter(|| assert_eq!(default_severity(&layers), 788))
        });
    }

    pub fn bench_p2(c: &mut Criterion) {
        c.bench_function("d13 p2", |b| {
            let layers = parse_layers(FULL).unwrap();
            b.iter(|| assert_eq!(delay(&layers), 3_905_748))
        });
    }
}

mod day14 {
    use adventofcode::seventeen::day14::*;
    use criterion::Criterion;

    const FULL: &str = "oundnydw";

    pub fn bench_p1(c: &mut Criterion) {
        c.bench_function("d14 p1", |b| {
            b.iter(|| assert_eq!(squares_used(FULL), 8106))
        });
    }

    pub fn bench_p2(c: &mut Criterion) {
        c.bench_function("d14 p2", |b| {
            let grid = parse_grid(FULL);
            b.iter(|| assert_eq!(regions(grid.clone()), 1164))
        });
    }
}

mod day15 {
    use adventofcode::seventeen::day15::*;
    use criterion::Criterion;

    pub fn bench_p1(c: &mut Criterion) {
        c.bench_function("d15 p1", |b| b.iter(|| assert_eq!(first(634, 301), 573)));
    }

    pub fn bench_p2(c: &mut Criterion) {
        c.bench_function("d15 p2", |b| b.iter(|| assert_eq!(second(634, 301), 294)));
    }
}

mod day16 {
    use adventofcode::seventeen::day16::*;
    use criterion::Criterion;

    const FULL: &str = include_str!("../data/d16-test");

    pub fn bench_p2(c: &mut Criterion) {
        c.bench_function("d16 p2", |b| {
            let routine = parse_routine(FULL).unwrap();
            b.iter(|| {
                assert_eq!(
                    dance(&routine, 1_000_000_000),
                    "gnflbkojhicpmead".to_owned()
                )
            })
        });
    }
}

mod day17 {
    use adventofcode::seventeen::day17::*;
    use criterion::Criterion;

    pub fn bench_p1(c: &mut Criterion) {
        let input = 354;
        c.bench_function("d17 p1", move |b| {
            b.iter(|| assert_eq!(spinlock(input), 2000))
        });
    }

    pub fn bench_p2(c: &mut Criterion) {
        let input = 354;
        c.bench_function("d17 p2", move |b| {
            b.iter(|| assert_eq!(angry_spinlock(input, 50_000_000), 10_242_889))
        });
    }
}

mod day18 {
    use adventofcode::seventeen::day18::*;
    use criterion::Criterion;

    const FULL: &str = include_str!("../data/d18-test");

    pub fn bench_p1(c: &mut Criterion) {
        c.bench_function("d18 p1", |b| {
            let inst = parse(FULL).unwrap();
            b.iter(|| assert_eq!(duet(&inst), 3188))
        });
    }

    pub fn bench_p2(c: &mut Criterion) {
        c.bench_function("d18 p2", |b| {
            let inst = parse(FULL).unwrap();
            b.iter(|| assert_eq!(thread_duet(&inst), 7112))
        });
    }
}

mod day19 {
    use adventofcode::seventeen::day19::*;
    use criterion::Criterion;

    const FULL: &str = include_str!("../data/d19-test");

    pub fn bench_p1(c: &mut Criterion) {
        c.bench_function("d19 p1", |b| {
            b.iter(|| {
                assert_eq!(
                    get_letters(Path::from_str(FULL).unwrap()),
                    "ABCDEF".to_owned()
                )
            })
        });
    }
}

mod day20 {
    use adventofcode::seventeen::day20::*;
    use criterion::Criterion;

    const FULL: &str = include_str!("../data/d20-test");

    pub fn bench_p1(c: &mut Criterion) {
        c.bench_function("d20 p1", |b| {
            let particles = parse(FULL).unwrap();
            b.iter(|| assert_eq!(first(&particles), 119))
        });
    }

    pub fn bench_p2(c: &mut Criterion) {
        c.bench_function("d20 p2", |b| {
            let particles = parse(FULL).unwrap();
            b.iter(|| assert_eq!(second(particles.clone()), 471))
        });
    }
}

mod day21 {
    use adventofcode::seventeen::day21::*;
    use criterion::Criterion;

    const FULL: &str = include_str!("../data/d21-test");

    pub fn bench_both(c: &mut Criterion) {
        c.bench_function("d21 both", |b| {
            let grid = Grid::from_str(FULL).unwrap();
            b.iter(|| assert_eq!(evolve(grid.clone(), 18), 3_018_423));
        });
    }
}

mod day22 {
    use adventofcode::seventeen::day22::*;
    use criterion::Criterion;

    const FULL: &str = include_str!("../data/d22-test");

    pub fn bench_p1(c: &mut Criterion) {
        c.bench_function("d22 p1", |b| {
            let grid = parse_grid(FULL).unwrap();
            b.iter(|| assert_eq!(infection(grid.clone(), 10_000), 5433))
        });
    }

    pub fn bench_p2(c: &mut Criterion) {
        c.bench_function("d22 p2", |b| {
            let grid = parse_grid(FULL).unwrap();
            b.iter(|| assert_eq!(evolved_infection(grid.clone(), 10_000_000), 2_512_599))
        });
    }
}

mod day23 {
    use adventofcode::seventeen::day23::*;
    use criterion::Criterion;

    const FULL: &str = include_str!("../data/d23-test");

    pub fn bench_p1(c: &mut Criterion) {
        c.bench_function("d23 p1", |b| {
            b.iter(|| assert_eq!(debug_processor(FULL).unwrap(), 5929))
        });
    }

    pub fn bench_p2(c: &mut Criterion) {
        c.bench_function("d23 p2", |b| {
            b.iter(|| assert_eq!(optimize_processor(FULL).unwrap(), 907))
        });
    }
}

mod day24 {
    use adventofcode::seventeen::day24::*;
    use criterion::Criterion;

    const FULL: &str = include_str!("../data/d24-test");

    pub fn bench_p1(c: &mut Criterion) {
        c.bench_function("d24 p1", |b| {
            let mut connectors = parse_connectors(FULL).unwrap();
            b.iter(|| assert_eq!(strongest_bridge(&mut connectors), 1906))
        });
    }

    pub fn bench_p2(c: &mut Criterion) {
        c.bench_function("d24 p2", |b| {
            let mut connectors = parse_connectors(FULL).unwrap();
            b.iter(|| assert_eq!(longest_bridge(&mut connectors), 1824))
        });
    }
}

mod day25 {
    use adventofcode::seventeen::day25::*;
    use criterion::Criterion;

    const FULL: &str = include_str!("../data/d25-full");

    pub fn bench_p1(c: &mut Criterion) {
        c.bench_function("d25", |b| b.iter(|| assert_eq!(first(FULL).unwrap(), 2870)));
    }
}

macro_rules! bench_both {
    ($name:ident, $($module:tt),*) => {
        criterion_group!(
            $name
            $(, $module::bench_both)*
        );
    };
}

macro_rules! bench {
    ($name:ident, $($module:tt),*) => {
        criterion_group!(
            $name
            $(, $module::bench_p1, $module::bench_p2)*
        );
    };
}

bench!(
    benches, day1, day2, day3, day4, day5, day7, day13, day14, day15, day17, day18, day20, day22,
    day23, day24
);

bench_both!(boths, day6, day8, day9, day11, day12, day21);

criterion_group!(
    rest,
    day10::bench_p2,
    day16::bench_p2,
    day19::bench_p1,
    day25::bench_p1
);

criterion_main!(benches, boths, rest);
