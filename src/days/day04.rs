//! # Day 04 Printing Department

use std::collections::VecDeque;

use fxhash::FxHashSet as HashSet;
use fxhash::FxHashMap as HashMap;
use aoc_runner::Day;

type I = i16;
type Maze = HashMap<(I, I), u8>;

#[derive(Default, Clone)]
pub struct Day04 {
    maze: Maze
}

impl Day for Day04 {
    type Result1 = usize;
    type Result2 = usize;

    fn parse(&mut self, input: &str) {
        let keys = input
            .lines().enumerate()
            .flat_map(|(y, line)|
                line.chars().enumerate().map(move |(x, c)| ((y,x), c))
            )
            .filter_map(|((y, x), c)| if c == '@' {
                Some((y as I, x as I))
            } else {
                None
            })
            .collect::<HashSet<_>>();

        self.maze = keys.iter().map(|it| {
            let value = neighbours8(*it).into_iter().filter(|n| keys.contains(n)).count();
            (*it, value as u8)
        })
        .collect();
    }

    fn part1(&mut self) -> Self::Result1 {
        self.maze.values().filter(|&&it| it < 4).count()
    }

    fn part2(&mut self) -> Self::Result2 {
        let mut visited: HashSet<_> = Default::default();
        let mut queue: VecDeque<_> = self.maze.iter().filter(|&(_, &it)| it < 4).map(|(coord, _)| coord).cloned().collect();
        while let Some(coord) = queue.pop_front() {
            if !visited.insert(coord) {
                continue;
            }

            for n in neighbours8(coord) {
                if let Some(v) = self.maze.get_mut(&n) {
                    *v = v.saturating_sub(1);
                    if *v < 4 {
                        queue.push_back(n);
                    }
                }
            }
        }

        visited.len()
    }
}

fn neighbours8((y, x): (I, I)) -> [(I, I); 8] {
    [
        (y - 1, x - 1),
        (y - 1, x),
        (y - 1, x + 1),
        (y, x - 1),
        (y, x + 1),
        (y + 1, x - 1),
        (y + 1, x),
        (y + 1, x + 1),
    ]
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::*;

    const INPUT: &str = indoc!{"
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.
    "};

    #[test]
    fn part_1() {
        let mut day = Day04::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 13);
    }

    #[test]
    fn part_2() {
        let mut day = Day04::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 43);
    }
}
