//! Collection of all puzzles and utilities to run a day's puzzle

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;

use aoc_runner::{Analyzer, Day};
use derive_aoc_runner::{Analyzer, AoC};

#[derive(Analyzer, AoC)]
pub struct Days(
    day01::Day01,
    day02::Day02,
    day03::Day03,
    day04::Day04,
    day05::Day05,
    day06::Day06,
    day07::Day07,
    day08::Day08,
    day09::Day09,
    day10::Day10,
    day11::Day11,
);
