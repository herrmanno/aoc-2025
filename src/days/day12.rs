//! # Day 12 Christmas Tree Farm

use aoc_runner::Day;

#[derive(Default, Clone)]
pub struct Day12 {
    problems: Vec<Problem>,
}

impl Day for Day12 {
    type Result1 = usize;
    type Result2 = u32;

    fn parse(&mut self, input: &str) {
        self.problems = input.split("\n\n").last().unwrap().lines().map(Problem::from).collect();
    }

    fn part1(&mut self) -> Self::Result1 {
        self.problems.iter().filter(|it| {
            let num_shapes = it.shapes.iter().sum::<usize>();
            let num_shapse_trivially_packable = (it.width / 3) * (it.height / 3);
            num_shapse_trivially_packable >= num_shapes

        }).count()
    }

    fn part2(&mut self) -> Self::Result2 {
        0
    }
}

#[derive(Default, Clone)]
struct Problem {
    width: usize,
    height: usize,
    shapes: Vec<usize>
}

impl From<&str> for Problem {
    fn from(value: &str) -> Self {
        let (dimensions, shapes) = value.split_once(":").unwrap();
        let (width, height) = dimensions.split_once('x').unwrap();

        Self {
            width: width.parse().unwrap(),
            height: height.parse().unwrap(),
            shapes: shapes.split_whitespace().map(|it| it.parse().unwrap()).collect(),
        }
    }
}
