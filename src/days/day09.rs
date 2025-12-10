//! # Day 09 Movie Theater

use std::{collections::BTreeMap, iter::once, ops::Range};

use aoc_runner::Day;
use itertools::Itertools;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

type I = i32;
type Area = u64;

#[derive(Default, Clone)]
pub struct Day09 {
    polygon: Polygon,
}

impl Day for Day09 {
    type Result1 = Area;
    type Result2 = Area;

    fn parse(&mut self, input: &str) {
        let points = input.lines()
            .map(|line| {
                let (y, x) = line.split_once(",").unwrap();
                Point {
                    y: y.parse().unwrap(),
                    x: x.parse().unwrap(),
                }
            })
            .collect();

        self.polygon = Polygon::new(points);
    }

    fn part1(&mut self) -> Self::Result1 {
        self.polygon.points_pairwise()
            .map(|(p1, p2)| {
                p1.area(p2)
            })
            .max()
            .unwrap()
    }

    fn part2(&mut self) -> Self::Result2 {
        let rects_by_area_desc = self.polygon.points_pairwise()
            .map(|(p1, p2)| {
                (p1.area(p2), p1, p2)
            })
            .sorted_by_key(|(area, _, _)| *area)
            .rev()
            .collect::<Vec<_>>();

        rects_by_area_desc
            .into_par_iter()
            .by_exponential_blocks()
            .find_first(|&(_, p1, p2)| {
                let rect = Points::min_max(p1.rect_points(p2));

                // A line is valid, is not end point of it lies strictly inside the rectangle
                let line_valid = |line: &Line| {
                    let line = Points::min_max(line.points());
                    line.x_max <= rect.x_min || line.x_min >= rect.x_max || line.y_max <= rect.y_min || line.y_min >= rect.y_max
                };

                // std::fs::write(std::env::current_dir().unwrap().join("09.svg"), create_svg(&self.polygon.points_slice(), p1.rect_lines(p2))).unwrap();
                self.polygon.lines_by_y(rect.y_min + 1..rect.y_max).all(line_valid) &&
                self.polygon.lines_by_x(rect.x_min + 1..rect.x_max).all(line_valid)
            })
            .map(|(area, _, _)| area)
            .unwrap()
    }
}

#[derive(Debug, Clone, Default)]
struct Polygon {
    points: Vec<Point>,
    lines_by_y: BTreeMap<I, Line>,
    lines_by_x: BTreeMap<I, Line>,
}

impl Polygon {
    fn new(points: Vec<Point>) -> Self {
        let lines = points.iter().zip(points.iter().skip(1).chain(once(points.first().unwrap()))).map(|(a, b)| {
            Line::new(*a, *b)
        });

        let mut lines_by_y: BTreeMap<I, Line> = Default::default();
        let mut lines_by_x: BTreeMap<I, Line> = Default::default();

        for line in lines {
            match line.direction() {
                Direction::Vertical => {
                    lines_by_x.insert(line.0.x, line.clone());
                }
                Direction::Horizontal => {
                    lines_by_y.insert(line.0.y, line.clone());
                }
            }
        }

        Self {
            points,
            lines_by_y,
            lines_by_x,
        }
    }

    /// All points of this polygon
    fn points(&self) -> impl Iterator<Item = &Point> {
        self.points.iter()
    }

    /// All distinct pairs of points of this polygon
    fn points_pairwise(&self) -> impl Iterator<Item = (&Point, &Point)> {
        self.points().enumerate()
            .flat_map(|(idx, c)| {
                std::iter::zip(
                    std::iter::repeat(c),
                    self.points.iter().skip(idx + 1)
                )
            })
    }

    /// All horizontal lines of this polygon whose y-value is inside `y_range`
    fn lines_by_y(&self, y_range: Range<I>) -> impl Iterator<Item = &Line> {
        self.lines_by_y.range(y_range).map(|(_, line)| line)
    }

    /// All vertical lines of this polygon whose x-value is inside *x_range*
    fn lines_by_x(&self, x_range: Range<I>) -> impl Iterator<Item = &Line> {
        self.lines_by_x.range(x_range).map(|(_, line)| line)
    }
}

#[derive(Debug, Copy, Clone)]
struct MinMax {
    x_min: I,
    x_max: I,
    y_min: I,
    y_max: I,
}

struct Points;

impl Points {
    #[inline]
    fn min_max<T, P>(points: T) -> MinMax where T: IntoIterator<Item = P>, P: AsRef<Point> {
        let mut x_min = I::MAX;
        let mut x_max = I::MIN;
        let mut y_min = I::MAX;
        let mut y_max = I::MIN;

        for p in points {
            let p = p.as_ref();
            x_min = x_min.min(p.x);
            x_max = x_max.max(p.x);
            y_min = y_min.min(p.y);
            y_max = y_max.max(p.y);
        }

        MinMax { x_min, x_max, y_min, y_max }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Point {
    y: I,
    x: I,
}

impl AsRef<Point> for Point {
    #[inline]
    fn as_ref(&self) -> &Point {
        self
    }
}

impl Point {
    /// Area of the rectangle formed by *self* and *other*
    #[inline]
    fn area(&self, other: &Point) -> Area {
        (self.y.abs_diff(other.y) + 1) as Area * (self.x.abs_diff(other.x) + 1) as Area
    }

    /// All four vertices of the rectangle formed by *self* and *other*
    #[inline]
    fn rect_points(&self, other: &Point) -> [Point; 4] {
        let &Point { y: y1, x: x1 } = self;
        let &Point { y: y2, x: x2 } = other;
        [
            Point {y: y1, x: x1},
            Point {y: y1, x: x2},
            Point {y: y2, x: x2},
            Point {y: y2, x: x1},
        ]
    }
}

#[derive(Debug, Copy, Clone)]
struct Line(Point, Point);

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Vertical,
    Horizontal,
}

impl Line {
    #[inline]
    fn new(a: Point, b: Point) -> Self {
        Self(a, b)
    }

    #[inline]
    fn direction(&self) -> Direction {
        let Line(a, b) = self;
        if a.y == b.y {
            Direction::Horizontal
        } else {
            Direction::Vertical
        }
    }

    #[inline]
    fn points(&self) -> impl Iterator<Item = &Point> {
        [&self.0, &self.1].into_iter()
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::*;

    const INPUT: &str = indoc!{"
        7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3
    "};

    #[test]
    fn part_1() {
        let mut day = Day09::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 50);
    }

    #[test]
    fn part_2() {
        let mut day = Day09::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 24);
    }
}

#[allow(unused)]
fn create_svg(polygon: &[Point], rectangle: [Line; 4]) -> String {
    const SCALE: f32 = 100.0;
    // const SCALE: f32 = 0.1;

    macro_rules! scale {
        ($v: expr) => {
            ($v as f32 / SCALE) as I
        };
    }

    let mut min_x = I::MAX;
    let mut max_x = I::MIN;
    let mut min_y = I::MAX;
    let mut max_y = I::MIN;

    polygon.iter().for_each(|Point { x, y }| {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        min_y = min_y.min(*y);
        max_y = max_y.max(*y);
    });

    let points = polygon.iter()
        .map(|Point {y, x}| format!("{},{}", scale!(*x), scale!(*y)))
        .join(" ");

    let rect = rectangle.iter()
        .map(|Line(p1, p2)| format!("{},{} {},{}", scale!(p1.x), scale!(p1.y), scale!(p2.x), scale!(p2.y)))
        .join(" ");

    let mut s = String::new();
    s.push_str(&format!(r#"<svg height="{}" width="{}" xmlns="http://www.w3.org/2000/svg">"#, scale!(max_y), scale!(max_x)));
    s.push_str(&format!(r#"<polygon points="{points}" style="fill:lime;fill-opacity:.5;stroke:red;stroke-width:2"/>"#));
    s.push_str(&format!(r#"<polygon points="{rect}" style="fill:red;fill-opacity:.5;stroke:blue;stroke-opacity:.5;stroke-width:2"/>"#));
    s.push_str("</svg>");

    s
}
