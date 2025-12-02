//! # Day 02 Gift Shop

use std::{collections::HashSet, iter, ops::Range};

use aoc_runner::Day;

#[derive(Default, Clone)]
pub struct Day02 {
    ranges: Vec<Range<u64>>
}

impl Day for Day02 {
    type Result1 = u64;
    type Result2 = u64;

    fn parse(&mut self, input: &str) {
        self.ranges = input.lines().nth(0).unwrap()
            .split(",")
            .into_iter()
            .map(|pair| {
                let (lo, hi) = pair.split_once("-").unwrap();
                let (lo, hi) = (lo.parse().unwrap(), hi.parse::<u64>().unwrap());
                lo..(hi + 1)
            })
            .collect();
    }

    fn part1(&mut self) -> Self::Result1 {
        self.ranges.iter().flat_map(find_invalid_ids_v1).sum()
    }

    fn part2(&mut self) -> Self::Result2 {
        self.ranges.iter().flat_map(find_invalid_ids_v2).sum()
    }
}

fn find_invalid_ids_v1(range: &Range<u64>) -> Vec<u64> {
    let mut result = vec![];
    let (lo, hi) = (range.start, range.end);
    let mut n = lo;

    loop {
        n = next_invalid_id_v1(n);
        if n >= lo && n <= hi {
            result.push(n);
            n += 1;
        } else {
            break result
        }
    }
}

fn next_invalid_id_v1(num: u64) -> u64 {
    let mut n = num;
    let log = n.ilog10(); // xx -> 1; xxx -> 2; xxxx -> 3; xxxxx -> 4; xxxxxx -> 5
    let mut exp = log;
    if log % 2 == 0 {
        exp += 1;
        n = 10u64.pow(exp);
    }
    let div = 10u64.pow((exp / 2) + 1);
    let mut hi = n / div;

    loop {
        let result = hi * div + hi;

        if result >= num {
            break result;
        } else {
            hi += 1;
        }
    }
}
fn find_invalid_ids_v2(range: &Range<u64>) -> impl IntoIterator<Item = u64> {
    let mut result: HashSet<u64> = Default::default();
    let (lo, hi) = (range.start, range.end);
    let lo_str = lo.to_string();
    let hi_str = hi.to_string();
    let max_pattern_len = (hi_str.len() / 2).max(1);
    for pattern_len in 1..=max_pattern_len {
        let exp = 10u64.pow(pattern_len as u32);
        let test_range = if lo_str.len() == hi_str.len() {
          let lo: u64 = lo_str[..pattern_len].parse().unwrap();
          let hi: u64 = hi_str[..pattern_len].parse().unwrap();
          lo.min(hi)..=lo.max(hi)
        } else {
            // See test case for range 77..111:
            // If lo and hi have different length, we need to test for 1..9, not for 1..7
            10u64.pow(pattern_len as u32 - 1)..=(10u64.pow(pattern_len as u32) - 1)
        };

        for i in test_range {
            result.extend(repeat_until(i, exp, lo, hi));
        }
    }

    result
}

/// Repeats `i` indefinetely and returns all repetitions in `lo..=hi`
fn repeat_until(i: u64, exp: u64, lo: u64, hi: u64) -> impl Iterator<Item = u64> {
    iter::repeat(i)
        .scan(i, move |acc, el| {
            *acc = *acc * exp + el;
            if *acc <= hi {
                Some(*acc)
            } else {
                None
            }

        })
        .into_iter()
        .skip_while(move |it| *it < lo)
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::*;

    const INPUT: &str = indoc!{"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"};

    #[test]
    fn invalid_id_v1() {
        assert_eq!(next_invalid_id_v1(10), 11);
        assert_eq!(next_invalid_id_v1(100), 1010);
        assert_eq!(next_invalid_id_v1(1111), 1111);
    }

    #[test]
    fn invalid_id_v2() {
        assert_eq!(find_invalid_ids_v2(&(1..21)).into_iter().collect::<HashSet<_>>(), HashSet::from([11]));
        assert_eq!(find_invalid_ids_v2(&(77..116)).into_iter().collect::<HashSet<_>>(), HashSet::from([77, 88, 99, 111]));
        assert_eq!(find_invalid_ids_v2(&(95..115)).into_iter().collect::<HashSet<_>>(), HashSet::from([99, 111]));
        assert_eq!(find_invalid_ids_v2(&(45515..60929)).into_iter().collect::<HashSet<_>>(), HashSet::from([55555]));
    }

    #[test]
    fn part_1() {
        let mut day = Day02::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 1227775554);
    }

    #[test]
    fn part_2() {
        let mut day = Day02::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 4174379265);
    }
}
