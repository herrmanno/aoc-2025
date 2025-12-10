//! # Day 03 Lobby

use aoc_runner::Day;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Default, Clone)]
pub struct Day03 {
    batteries: Vec<Vec<u8>>
}

impl Day for Day03 {
    type Result1 = u32;
    type Result2 = u64;

    fn parse(&mut self, input: &str) {
        self.batteries = input
            .lines()
            .map(|line| {
                line.chars().map(|c| c as u8 - '0' as u8).collect()
            })
            .collect()
    }

    fn part1(&mut self) -> Self::Result1 {
        self.batteries
            .iter()
            .map(|line| {
                let mut max_digit_so_far = 0;
                let mut max_value_so_far = 0;
                for i in line.iter() {
                    max_value_so_far = max_value_so_far.max(max_digit_so_far * 10 + i);
                    max_digit_so_far = max_digit_so_far.max(*i);
                }
                max_value_so_far as u32
            })
            .sum()
    }

    fn part2(&mut self) -> Self::Result2 {
        self.batteries
            .par_iter()
            .map(|line| {
                let mut digits = line[..12].to_vec();
                let mut max_value = to_num(&digits);
                for n in line.iter().skip(12) {
                    let (i, m) = (0..12).into_iter().map(|i| {
                        let m = to_num_skipping(&digits, i) * 10 + *n as u64;
                        (i, m)
                    })
                    .max_by_key(|(_, m)| *m)
                    .unwrap();

                    if m > max_value {
                        max_value = m;
                        digits.remove(i);
                        digits.push(*n);
                    }
                }

                max_value
            })
            .sum()
    }
}

#[inline]
fn to_num(digits: &[u8]) -> u64 {
    to_num_skipping(digits, digits.len())
}

#[inline]
fn to_num_skipping(digits: &[u8], index_to_skip: usize) -> u64 {
    digits.iter().enumerate().fold(0u64, |acc, (idx, el)| {
        if idx == index_to_skip {
            acc
        } else {
            acc * 10 + *el as u64
        }
    })
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::*;

    const INPUT: &str = indoc!{"
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    "};

    #[test]
    fn part_1() {
        let mut day = Day03::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 357);
    }

    #[test]
    fn part_2() {
        let mut day = Day03::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 3121910778619);
    }
}
