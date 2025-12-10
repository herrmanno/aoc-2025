//! # Day 07 Laboratories

use std::collections::VecDeque;

use fxhash::FxHashSet as HashSet;
use fxhash::FxHashMap as HashMap;
use aoc_runner::Day;

type I = i16;
type C = (I, I);
type Splitters = HashSet<C>;

#[derive(Default, Clone)]
pub struct Day07 {
    width: I,
    height: I,
    start: C,
    splitters: Splitters,
}

impl Day for Day07 {
    type Result1 = u32;
    type Result2 = u64;

    fn parse(&mut self, input: &str) {
        let tiles = input.lines().enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, c)| {
                    ((y as I, x as I), c)
                })
            });

        for (coord@(y, x), char) in tiles {
            if char == 'S' {
                self.start = coord;
            } else if char == '^' {
                self.splitters.insert(coord);
            }

            self.width = self.width.max(x);
            self.height = self.height.max(y);
        }
    }

    fn part1(&mut self) -> Self::Result1 {
        let mut result = 0;
        let mut queue: VecDeque<C> = Default::default();
        queue.push_front(self.start);
        let mut visited: HashSet<C> = Default::default();

        while let Some((y, x)) = queue.pop_front() {
            if y >= self.height {
                continue;
            }
            if !visited.insert((y, x)) {
                continue;
            }

            if self.splitters.contains(&(y, x)) {
                queue.push_back((y, x - 1));
                queue.push_back((y, x + 1));
                result += 1;
            } else {
                queue.push_back((y + 1, x));
            }
        }

        result
    }

    fn part2(&mut self) -> Self::Result2 {
        type ResultType = <crate::days::day07::Day07 as Day>::Result2;
        type Cache = HashMap<C, ResultType>;
        let mut cache: Cache = Default::default();

        fn get_value(start: C, splitters: &Splitters, cache: &mut Cache, coord@(y, x): C) -> ResultType {
            macro_rules! recurse { ($coord: expr) => { get_value(start, splitters, cache, $coord) }; }

            if y == 0 {
                return (coord == start) as ResultType
            }

            if let Some(value) = cache.get(&coord) {
                return *value;
            }

            let value = {
                let left = if splitters.contains(&(y, x - 1)) {
                    recurse!((y - 1, x - 1))
                } else {
                    0
                };
                let right = if splitters.contains(&(y, x + 1)) {
                    recurse!((y - 1, x + 1))
                } else {
                    0
                };
                let top = if splitters.contains(&(y - 1, x)) {
                    0
                } else {
                    recurse!((y - 1, x))
                };
                left + right + top
            };

            cache.insert(coord, value);

            value
        }

        (0..=self.width).map(|x| {
            get_value(self.start, &self.splitters, &mut cache, (self.height, x))
        })
        .sum()
    }

}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::*;

    const INPUT: &str = indoc!{"
        .......S.......
        ...............
        .......^.......
        ...............
        ......^.^......
        ...............
        .....^.^.^.....
        ...............
        ....^.^...^....
        ...............
        ...^.^...^.^...
        ...............
        ..^...^.....^..
        ...............
        .^.^.^.^.^...^.
        ...............
    "};

    #[test]
    fn part_1() {
        let mut day = Day07::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 21);
    }

    #[test]
    fn part_2() {
        let mut day = Day07::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 40);
    }
}
