//! # Day 05 Cafeteria

use aoc_runner::Day;
use itertools::Itertools;

type I = u128;
type Range = (I, I);

#[derive(Default, Clone)]
pub struct Day05 {
    ranges: Vec<Range>,
    ids: Vec<I>,
}

impl Day for Day05 {
    type Result1 = usize;
    type Result2 = u128;

    fn parse(&mut self, input: &str) {
        let (ranges, ids) = input.split_once("\n\n").unwrap();
        self.ranges = ranges.lines().map(|line| {
            let (lo, hi) = line.split_once("-").unwrap();
            (lo.parse::<I>().unwrap(), hi.parse::<I>().unwrap())
        })
        .sorted_by_key(|r| (r.0, r.1))
        .fold(vec![], |mut acc, el| {
            if acc.is_empty() {
                acc.push(el);
            } else {
                // merge ranges
                let last = acc.last_mut().unwrap();
                if el.0 <= last.1 {
                    last.1 = last.1.max(el.1);
                } else {
                    acc.push(el);
                }
            }

            acc
        });
        self.ids = ids.lines().map(|it| it.parse().unwrap()).collect();
    }

    fn part1(&mut self) -> Self::Result1 {
        self.ids.iter().filter(|it| {
            self.ranges.iter().any(|r| contains(r, **it))
        }).count()
    }

    fn part2(&mut self) -> Self::Result2 {
        self.ranges.iter().map(|(lo, hi)| hi - lo + 1).sum::<I>() as Self::Result2
    }
}

#[inline]
fn contains(r: &Range, i: I) -> bool {
    i >= r.0 && i <= r.1
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::*;

    const INPUT: &str = indoc!{"
        3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32
    "};

    #[test]
    fn part_1() {
        let mut day = Day05::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 3);
    }

    #[test]
    fn part_2() {
        let mut day = Day05::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 14);
    }
}
