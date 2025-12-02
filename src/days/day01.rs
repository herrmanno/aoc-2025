//! # Day 01 Secret Entrance

use aoc_runner::Day;

#[derive(Default, Clone)]
pub struct Day01 {
    instructions: Vec<i16>,
}

impl Day for Day01 {
    type Result1 = u16;
    type Result2 = u16;

    fn parse(&mut self, input: &str) {
        self.instructions = input
            .lines()
            .into_iter()
            .filter(|it| !it.is_empty())
            .map(|line| {
                let (dir, num) = line.split_at(1);
                let int: i16 = num.parse().unwrap();
                if dir == "L" { -int } else { int }
            })
            .collect();
    }

    fn part1(&mut self) -> Self::Result1 {
        let mut start = 50i16;
        let mut count = 0;
        for i in self.instructions.iter() {
            start += *i as i16;
            start %= 100;
            count += (start == 0) as Self::Result2;
        }

        count
    }

    fn part2(&mut self) -> Self::Result2 {
        let mut num = 50i16;
        let mut count = 0;
        for i in self.instructions.iter() {
            let new = num + *i as i16;
            if new <= 0 {
                count += if num == 0 { 0 } else { 1 };
                count += (new / 100).abs() as Self::Result2;
            }
            if new >= 100 {
                count += (new / 100).abs() as Self::Result2;
            }
            num = new.rem_euclid(100);
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82
    "};

    #[test]
    fn part_1() {
        let mut day = Day01::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 3);
    }

    #[test]
    fn part_2() {
        let mut day = Day01::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 6);
    }
}
