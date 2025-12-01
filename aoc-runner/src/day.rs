//! Defines a common interface for [Advent of Code](http://adventofcode.com) puzzles

/// A day's challenge
pub trait Day: Default {
    type Result1: std::fmt::Display + Sized;
    type Result2: std::fmt::Display + Sized;

    /// Part 1 of this day's challenge
    fn part1(&mut self) -> Self::Result1;

    /// Part 2 of this day's challenge
    fn part2(&mut self) -> Self::Result2;

    /// Print result of part 1
    fn print_part1(&self, result: Self::Result1) {
        println!(" - Part 1: {}", result)
    }

    /// Print result of part 2
    fn print_part2(&self, result: Self::Result2) {
        println!(" - Part 2: {}", result)
    }

    /// Optional: parse input to use later in part1/part2
    fn parse(&mut self, _input: &str) {}
}
