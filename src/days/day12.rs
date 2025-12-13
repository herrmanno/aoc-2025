//! # Day 12 Christmas Tree Farm

use aoc_runner::Day;

#[derive(Default, Clone)]
pub struct Day12 {
    // shapes: Vec<Shape>,
    problems: Vec<Problem>,
}

impl Day for Day12 {
    type Result1 = usize;
    type Result2 = u32;

    fn parse(&mut self, input: &str) {
        let parts = input.split("\n\n");
        let len = parts.clone().count();
        // self.shapes = parts.clone().take(len - 1).map(|s| {
        //     Shape::from(&s[3..])
        // }).collect();
        self.problems = parts.last().unwrap().lines().map(Problem::from).collect();
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


// #[derive(Default, Clone)]
// struct Shape {
//     width: usize,
//     height: usize,
//     points: usize,
// }

// impl From<&str> for Shape {
//     fn from(value: &str) -> Self {
//         let points = value.lines().skip(1).fold(0, |acc, line| acc + line.chars().filter(|it| *it == '#').count());
//         Self {
//             width: 9,
//             height: 9,
//             points
//         }
//     }
// }

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
