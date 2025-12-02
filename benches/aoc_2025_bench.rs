use std::time::Duration;

const WARM_UP_TIME: Duration = Duration::from_secs(1);
const MEASUREMENT_TIME: Duration = Duration::from_secs(1);
const SAMPLE_SIZE: usize = 100;

macro_rules! get_input {
    ($day: expr) => {{
        let input_file_path = format!("./input/{}.txt", $day);
        let input = std::fs::read_to_string(input_file_path).expect("Could not read input file");
        input
    }};
}

macro_rules! bench_day {
    ($day: expr, $name: ident, $Day: ty) => {
    mod $name {
        use criterion::{black_box, criterion_group, Criterion};
        use aoc_runner::Day;

        fn parse(c: &mut Criterion) {
            let input = get_input!($day);
            let mut day = <$Day>::default();
            let name = format!("day {} - parse", $day);
            c.bench_function(&name, |b| b.iter(|| day.parse(black_box(&input))));
        }

        fn part1(c: &mut Criterion) {
            let input = get_input!($day);
            let mut day = <$Day>::default();
            day.parse(&input);
            let name = format!("day {} - part 1", $day);
            c.bench_function(&name, |b| b.iter(|| black_box(day.part1())));
        }

        fn part2(c: &mut Criterion) {
            let input = get_input!($day);
            let mut day = <$Day>::default();
            day.parse(&input);
            let name = format!("day {} - part 2", $day);
            c.bench_function(&name, |b| b.iter(|| black_box(day.part2())));
        }

        criterion_group!(
            name = bench;
            config = Criterion::default().sample_size(super::SAMPLE_SIZE).warm_up_time(super::WARM_UP_TIME).measurement_time(super::MEASUREMENT_TIME);
            targets = parse, part1, part2
        );
    }
    };
}

bench_day!("01", day_01, aoc2025::days::day01::Day01);
bench_day!("02", day_02, aoc2025::days::day02::Day02);

criterion::criterion_main!(
    day_01::bench,
    day_02::bench,
);
