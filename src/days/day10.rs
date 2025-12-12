//! # Day 10 Factory

use std::{fmt::Debug,  ops::{Add, Deref, DerefMut, Div, Mul, Sub}, usize};

use aoc_runner::Day;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Default, Clone)]
pub struct Day10 {
    machines: Vec<Machine>
}

impl Day for Day10 {
    type Result1 = u32;
    type Result2 = usize;

    fn parse(&mut self, input: &str) {
        self.machines = input.lines().map(Machine::from).collect();
    }

    fn part1(&mut self) -> Self::Result1 {
        self.machines.iter().map(|machine| {
            let mut states = vec![0; machine.buttons_binary.len()];
            let mut n = 1;
            'l: loop {
                states = states
                    .iter()
                    .flat_map(|&s| {
                        machine.buttons_binary.iter().map(move |&b| s ^ b)
                    })
                    .collect();

                for &s in states.iter() {
                    if s == machine.pattern {
                        break 'l n;
                    }
                }

                n += 1;
            }
        })
        .sum()
    }

    fn part2(&mut self) -> Self::Result2 {
        self.machines.par_iter().map(|machine| {
            let mut buttons = MinMaxButton::from_buttons(&machine.joltages, &machine.buttons_index);
            solve_machine(machine.joltages.clone(), &mut buttons, Limit::max()).unwrap()
        })
        .sum()
    }
}

fn solve_machine(joltages: Joltages, buttons: &mut [MinMaxButton], mut limit: Limit) -> Option<usize> {
    if joltages.is_zero() {
        return Some(0);
    }

    if buttons.len() == 0 {
        return None;
    }

    if limit.0 == 0 || joltages.iter().any(|value| *value > limit.0) {
        return None;
    }

    MinMaxButton::update(buttons, &joltages);

    if buttons.iter().any(|button| button.min > button.max) {
        return None;
    }

    if joltages.iter().enumerate().any(|(idx, value)| {
        buttons.iter()
            .filter(|it| it.button.contains(&idx))
            .map(|it| it.max)
            .sum::<usize>()
            .lt(value)
    }) {
        return None;
    }

    buttons.sort_unstable_by_key(|it| it.max - it.min);
    let (&mut [ref button], rest) = buttons.split_at_mut(1) else {
        unreachable!();
    };

    (button.min..=button.max).rev().filter_map(|count| {
        let joltages = joltages.subtract_button(&button.button, count)?;
        let result = solve_machine(joltages, rest, limit - count)?;
        limit.min_assign(result + count);
        Some(count + result)
    })
    .min()
}

type Pattern = u16;
type Button = u16;
type Joltage = usize;

#[derive(Clone, Default, PartialEq, Eq, Hash)]
struct Joltages(Vec<Joltage>);

impl Joltages {
    fn is_zero(&self) -> bool {
        self.0.iter().all(|it| *it == 0)
    }

    fn subtract_button(&self, button: &IndexButton, times: usize) -> Option<Self> {
        let mut this = self.clone();
        for idx in button.iter() {
            this[*idx] = this[*idx].checked_sub(times)?;
        }
        Some(this)
    }
}

impl Debug for Joltages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Deref for Joltages {
    type Target = [Joltage];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl DerefMut for Joltages {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_slice()
    }
}

impl Add for Joltages {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut r = self.0.clone();
        for (i, n) in rhs.into_iter().enumerate() {
            r[i] += n;
        }
        Joltages(r)
    }
}

impl Div<usize> for Joltages {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        let mut r = self.0.clone();
        for n in r.iter_mut() {
            *n /= rhs;
        }
        Joltages(r)
    }
}

impl Mul<usize> for Joltages {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        let mut r = self.0.clone();
        for n in r.iter_mut() {
            *n += rhs;
        }
        Joltages(r)
    }
}

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct IndexButton(Vec<usize>, usize);

impl Debug for IndexButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Deref for IndexButton {
    type Target = Vec<usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Default)]
struct Machine {
    pattern: Pattern,
    buttons_binary: Vec<Button>,
    buttons_index: Vec<IndexButton>,
    joltages: Joltages,
}

impl <S> From<S> for Machine where S: AsRef<str> {
    fn from(value: S) -> Self {
        let mut this = Self::default();
        let mut parts = value.as_ref().split_whitespace();
        this.pattern = {
            let s = parts.next().unwrap();
            s.trim_matches(&['[', ']']).chars().enumerate().fold(0 as Pattern, |acc, (idx, char)| {
                acc + match char {
                    '#' => (2 as Pattern).pow(idx as u32),
                    _ => 0
                }
            })
        };
        this.buttons_binary = {
            let btns = parts.clone().take_while(|it| it .starts_with('('));
            btns.map(|s| {
                s.trim_matches(&['(', ')'])
                    .split(',')
                    .map(|n| n.parse().unwrap())
                    .fold(0 as Button, |acc, el| {
                        acc + (2 as Button).pow(el)
                    })
            }).collect()
        };
        this.buttons_index = {
            let btns = parts.clone().take_while(|it| it .starts_with('('));
            btns.enumerate().map(|(i, s)| {
                let v = s.trim_matches(&['(', ')'])
                    .split(',')
                    .map(|n| n.parse().unwrap())
                    .collect();
                IndexButton(v, i)
            }).collect()
        };
        this.joltages = {
            let v = parts.skip_while(|it| it .starts_with('(')).next().unwrap()
                .trim_matches(&['{', '}'])
                .split(',')
                .map(|it| it.parse().unwrap())
                .collect();

            Joltages(v)
        };
        this
    }
}

#[derive(Debug)]
struct MinMaxButton {
    button: IndexButton,
    max: usize,
    min: usize,
}

impl MinMaxButton {
    fn from_buttons(joltages: &Joltages, buttons: &[IndexButton]) -> Vec<Self> {
        let mut buttons = buttons.iter().cloned().map(|button| {
            Self { button, min: 0, max: usize::MAX }
        }).collect::<Vec<_>>();

        Self::update(&mut buttons, joltages);

        buttons
    }

    fn update(buttons: &mut [Self], joltages: &Joltages) {
        Self::update_max(buttons, joltages);
        Self::update_min(buttons, joltages);
    }

    fn update_max(buttons: &mut [Self], joltages: &Joltages) {
        for b in buttons.iter_mut() {
            b.max = b.button.iter().map(|index| {
                joltages[*index]
            })
            .min()
            .unwrap();
        }
    }

    fn update_min(buttons: &mut [Self], joltages: &Joltages) {
        for i in 0..buttons.len() {
            let min = {
                let others = buttons.iter().filter(|it| it.button != buttons[i].button).collect::<Vec<_>>();
                buttons[i].button.iter().map(|index| {
                    let sum_other = others.iter()
                        .filter(|other| other.button.contains(index))
                        .map(|other| other.max).sum::<usize>();

                    joltages[*index].saturating_sub(sum_other)
                })
                .max()
                .unwrap()
            };
            buttons[i].min = min;
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Limit(usize);

impl Limit {
    fn max() -> Self {
        Self(usize::MAX)
    }

    fn min_assign(&mut self, other: usize) {
        self.0 = self.0.min(other);
    }
}

impl Sub<usize> for Limit {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0.saturating_sub(rhs))
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::*;

    const INPUT: &str = indoc!{"
        [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
    "};

    #[test]
    fn part_1() {
        let mut day = Day10::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 7);
    }

    #[test]
    fn part_2() {
        let mut day = Day10::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 33);
    }
}
