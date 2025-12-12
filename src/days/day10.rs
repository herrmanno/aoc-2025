//! # Day 10

use std::{collections::{HashMap, HashSet}, fmt::Debug, io::stdin, iter::{once, repeat}, ops::{Add, Deref, DerefMut, Div, Mul, Sub}, time::{self, Duration}, usize};

use aoc_runner::Day;
use fxhash::{FxHashMap};
use itertools::{Itertools, repeat_n};
use num::integer::binomial;
use rayon::iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelBridge, ParallelIterator};

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
            // println!("{machine:?}");
            // let start = time::Instant::now();
            let mut buttons = MinMaxButton::from_buttons(&machine.joltages, &machine.buttons_index);
            let result = solve_machine4(machine.joltages.clone(), &mut buttons, Limit::max()).unwrap();
            // println!();
            // println!("{:?}\n{:?}", time::Instant::now().duration_since(start), machine);
            result
        })
        .sum()
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

#[derive(Debug)]
struct MinMaxButton {
    button: IndexButton,
    max: usize,
    min: usize,
    least: usize,
}

impl MinMaxButton {
    fn from_buttons(joltages: &Joltages, buttons: &[IndexButton]) -> Vec<Self> {
        let mut buttons = buttons.iter().cloned().map(|button| {
            Self { button, least: 0, min: 0, max: usize::MAX }
        }).collect::<Vec<_>>();

        Self::update(&mut buttons, joltages);

        buttons
    }

    fn update(buttons: &mut [Self], joltages: &Joltages) {
        Self::update_max(buttons, joltages);
        Self::update_min(buttons, joltages);
        // Self::update_least(buttons, joltages);
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

    fn update_least(buttons: &mut [Self], joltages: &Joltages) {
        for i in 0..buttons.len() {
            let least = {
                let others = buttons.iter().filter(|it| it.button != buttons[i].button).collect::<Vec<_>>();
                buttons[i].button.iter().map(|index| {
                    let sum_other = others.iter()
                        .filter(|other| other.button.contains(index))
                        .map(|other| other.max).sum::<usize>();

                    joltages[*index].saturating_sub(sum_other)
                })
                .min()
                .unwrap()
            };
            buttons[i].least = least;
        }
    }
}

fn solve_machine4(joltages: Joltages, buttons: &mut [MinMaxButton], mut limit: Limit) -> Option<usize> {
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

    // if buttons.iter().any(|it| it.least > 0) {
    //     let mut reduced_result = 0;
    //      let joltages = buttons.iter().fold(joltages.clone(), |acc, b| {
    //          reduced_result += b.least;
    //          acc.subtract_button(&b.button, b.least).unwrap()
    //      });

    //      return solve_machine4(joltages, buttons, limit.sub(reduced_result)).map(|it| it + reduced_result);
    // }

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

    // let mut iter = (button.min..=button.max).into_iter();
    // let iter = std::iter::from_fn(|| Some([iter.next()?, iter.next_back()?])).flatten();

    fn alternate<Iter>(mut iter: Iter) -> impl Iterator<Item = Iter::Item>
    where
        Iter: DoubleEndedIterator,
    {
        let mut from_front = false;

        std::iter::from_fn(move || {
            from_front = !from_front;
            if from_front {
                iter.next()
            } else {
                iter.next_back()
            }
        })
    }

    // (button.min..=button.max).rev().filter_map(|count| {
    alternate(button.min..=button.max).filter_map(|count| {
        let joltages = joltages.subtract_button(&button.button, count)?;
        let result = solve_machine4(joltages, rest, limit - count)?;
        limit.min_assign(result + count);
        Some(count + result)
    })
    .min()
}

#[deprecated]
fn solve_machine3(joltages: Joltages, buttons: &[IndexButton]) -> Option<usize> {
    let (result, joltages) = match reduce_machine(joltages.clone(), &buttons) {
        (result, None) => (result, joltages),
        (result, Some(joltages)) => (result, joltages)
    };

    // let indices_ascending = joltages.iter().enumerate().sorted_by_key(|(_, joltage)| **joltage);
    let Some((i, v)) = joltages.iter().enumerate().sorted_by_key(|(_, joltage)| **joltage).skip_while(|(_, v)| **v == 0).next() else {
        return Some(0)
    };

    let (buttons, rest): (Vec<_>, Vec<_>) = buttons.iter().partition(|it| it.contains(&i));
    let rest = rest.into_iter().cloned().collect::<Vec<_>>();
    // dbg!(&joltages, i, v, &buttons, &rest);
    let cs = repeat_n(0..=*v, buttons.len()).multi_cartesian_product().filter(|it| it.iter().sum::<usize>() == *v);
    cs.par_bridge().filter_map(|counts| {
        // println!("{v}: {counts:?}");
        let joltages = counts.into_iter().zip(buttons.iter()).try_fold(joltages.clone(), |joltages, (count, btn)| {
            joltages.subtract_button(btn, count)
        })?;

        solve_machine3(joltages, rest.as_slice()).map(|it| it + v + result)
    })
    .min()
}

#[deprecated]
fn reduce_machine(mut joltages: Joltages, buttons: &[IndexButton]) -> (usize, Option<Joltages>) {
    let mut result = 0;
    while !joltages.is_zero() {
        let mut progress = false;

        for idx in 0..joltages.len() {
            let value = joltages[idx];

            let num_presses = value;
            let buttons_with_max_presses = buttons.iter()
                .filter(|it| it.contains(&idx))
                .map(|btn| {
                    // let max_presses = btn.iter().map(|idx| joltages[*idx]).min().unwrap();
                    let max_presses_value = btn.iter().position_min_by_key(|idx| joltages[**idx]).unwrap();
                    let max_presses_index = btn[max_presses_value];
                    let max_presses = joltages[max_presses_index];
                    (btn, max_presses, max_presses_index)
                })
                .collect::<Vec<_>>();

            let sum_max_presses = buttons_with_max_presses.iter()
                .dedup_by(|a, b| a.2 == b.2)
                .map(|(_, num, _)| *num)
                .sum::<usize>();

            let buttons_with_min_presses = buttons_with_max_presses.iter()
                .into_group_map_by(|(_, _, max_presses_index)| max_presses_index)
                .into_values()
                .map(|btns| {
                    let max_btn_presses = btns[0].1;
                    let min_btn_presses = (num_presses + max_btn_presses).saturating_sub(sum_max_presses);
                    let btns = btns.into_iter().map(|it| it.0).collect::<Vec<_>>();
                    (btns, min_btn_presses)
                })
                // .map(|(btn, max_btn_presses, _)| {
                //     let min_btn_presses = (num_presses + max_btn_presses).saturating_sub(sum_max_presses);
                //     (btn, min_btn_presses)
                // })
                .filter_map(|(btns, min_presses)| match btns.as_slice() {
                    &[b] => Some((b, min_presses)),
                    _ => None,
                })
                .collect::<Vec<_>>();

            // dbg!(idx, &joltages, sum_max_presses, &buttons_with_max_presses, &buttons_with_min_presses);
            // stdin().read_line(&mut String::new());

            for (btn, num_presses) in buttons_with_min_presses.into_iter() {
                // let btn = match btns.as_slice() {
                //     &[btn] => btn,
                //     _ => {
                //         continue;
                //     }
                // };

                if num_presses == 0 {
                    continue;
                }
                // dbg!(&btn, num_presses);
                joltages = match joltages.subtract_button(&btn, num_presses) {
                    Some(new) => {
                        result += num_presses;
                        progress = true;
                        new
                    }
                    None => joltages,
                }
            }
        }

        if !progress {
            return (result, Some(joltages));
        }
    }

    (result, None)
}

#[deprecated]
type M = FxHashMap<Vec<usize>, Option<usize>>;

#[deprecated]
fn solve_machine2(mut joltages: Joltages, buttons: &[IndexButton], buttons_pressed: &mut Vec<usize>, memo: &mut M) -> Option<usize> {
    if let Some(result) = memo.get(buttons_pressed) {
        // println!("memo");
        return *result;
    }

    let mut result = 0;
    while !joltages.is_zero() {
        let mut progress = false;

        let mut button_groups: Vec<Vec<IndexButton>> = vec![];
        for idx in 0..joltages.len() {
            let value = joltages[idx];

            let num_presses = value;
            let buttons_with_max_presses = buttons.iter()
                .filter(|it| it.contains(&idx))
                .map(|btn| {
                    // let max_presses = btn.iter().map(|idx| joltages[*idx]).min().unwrap();
                    let max_presses_value = btn.iter().position_min_by_key(|idx| joltages[**idx]).unwrap();
                    let max_presses_index = btn[max_presses_value];
                    let max_presses = joltages[max_presses_index];
                    (btn, max_presses, max_presses_index)
                })
                .collect::<Vec<_>>();

            let sum_max_presses = buttons_with_max_presses.iter()
                .dedup_by(|a, b| a.2 == b.2)
                .map(|(_, num, _)| *num)
                .sum::<usize>();

            let buttons_with_min_presses = buttons_with_max_presses.iter()
                .into_group_map_by(|(_, _, max_presses_index)| max_presses_index)
                .into_values()
                .map(|btns| {
                    let max_btn_presses = btns[0].1;
                    let min_btn_presses = (num_presses + max_btn_presses).saturating_sub(sum_max_presses);
                    let btns = btns.into_iter().map(|it| it.0).collect::<Vec<_>>();
                    (btns, min_btn_presses)
                })
                // .map(|(btn, max_btn_presses, _)| {
                //     let min_btn_presses = (num_presses + max_btn_presses).saturating_sub(sum_max_presses);
                //     (btn, min_btn_presses)
                // })
                .filter_map(|(btns, min_presses)| match btns.as_slice() {
                    &[b] => Some((b, min_presses)),
                    &[] => None,
                    btns => {
                        if min_presses > 0 {
                            button_groups.push(btns.into_iter().cloned().cloned().collect::<Vec<_>>());
                        }
                       None
                    }
                })
                .collect::<Vec<_>>();

            // dbg!(idx, &joltages, sum_max_presses, &buttons_with_max_presses, &buttons_with_min_presses);
            // stdin().read_line(&mut String::new());

            for (btn, num_presses) in buttons_with_min_presses.into_iter() {
                // let btn = match btns.as_slice() {
                //     &[btn] => btn,
                //     _ => {
                //         continue;
                //     }
                // };

                if num_presses == 0 {
                    continue;
                }
                // dbg!(&btn, num_presses);
                joltages = match joltages.subtract_button(&btn, num_presses) {
                    Some(new) => {
                        result += num_presses;
                        buttons_pressed[btn.1] += 1;
                        progress = true;
                        new
                    }
                    None => joltages,
                }
            }
        }

        if progress {
            continue;
        }

        let Some(button_candidates) = button_groups.into_iter().next() else {
            memo.insert(buttons_pressed.clone(), None);
            return None;
        };

        // dbg!(&joltages, &button_candidates);
        // stdin().read_line(&mut String::new());

        result = result + 1 + button_candidates.into_iter().filter_map(|btn| {
            let joltages = joltages.subtract_button(&btn, 1)?;
            let mut buttons_pressed = buttons_pressed.clone();
            buttons_pressed[btn.1] += 1;
            solve_machine2(joltages, buttons, &mut buttons_pressed, memo)
        })
        .min()?;

        memo.insert(buttons_pressed.clone(), Some(result));

        return Some(result);

    }

    Some(result)
}

#[deprecated]
fn solve_machine(machine: &mut Machine) -> usize {
    println!("{machine:?}");

    let mut memo: Memo = Default::default();
    match solve_joltage(0, usize::MAX, &machine.joltages, &mut machine.buttons_index, &mut memo) {
        Value::Value(v) => v,
        Value::None => unreachable!(),
    }
}

#[deprecated]
#[derive(PartialEq, Eq, Hash)]
struct MemoKey(Joltages, Vec<IndexButton>);

#[deprecated]
impl MemoKey {
    fn new(target: &Joltages, buttons: &[IndexButton]) -> Self {
        Self(target.clone(), buttons.iter().cloned().sorted().collect::<Vec<_>>())
    }
}

#[deprecated]
#[derive(Debug, Copy, Clone)]
enum Value {
    Value(usize),
    None
}

#[deprecated]
type Memo = FxHashMap<MemoKey, Value>;

#[deprecated]
#[inline]
fn solve_joltage(num_presses: usize, limit: usize, target: &Joltages, buttons: &mut [IndexButton], memo: &mut Memo) -> Value {
    if num_presses > limit {
        // println!("exit");
        // return Value::None;
    }

    let button_indices = buttons.iter().flat_map(|it| it.iter()).unique().collect::<Vec<_>>();
    for (idx, t) in target.iter().enumerate() {
        if *t > 0 && !button_indices.contains(&&idx) {
            return Value::None;
        }
    }

    if target.is_zero() {
        // println!("found");
        return Value::Value(0);
    }

    let key = MemoKey::new(target, buttons);
    if let Some(result) = memo.get(&key) {
        return *result;
    }

    let min_idx = target.iter().enumerate().fold((usize::MAX, &usize::MAX), |acc, (idx, value)| {
        if *value > 0 && value < acc.1 {
            (idx, value)
        } else {
            acc
        }
    })
    .0;

    let min_joltage = target[min_idx];
    buttons.sort_by_key(|it| !it.contains(&min_idx));
    let partition_idx = buttons.partition_point(|it| it.contains(&min_idx));
    let (min_buttons, mut other_buttons) = buttons.split_at_mut(partition_idx);
    min_buttons.sort_by_key(|it| - (it.len() as isize));

    let child_result = combinations(min_joltage as usize, min_buttons.len())
        .try_fold((Value::None, limit), |(result, limit), combination| {
        // .filter_map(|combination| {
            let target = combination.into_iter().try_fold(target.clone(), |acc, btn_idx| {
                acc.subtract_button(&min_buttons[btn_idx], 1)
            })?;
            let num_presses_so_far = num_presses + min_joltage;
            match solve_joltage(num_presses_so_far, limit, &target, &mut other_buttons, memo) {
                Value::Value(v) => match result {
                    Value::Value(old_v) if v < old_v => Some((Value::Value(v), v + num_presses_so_far)),
                    Value::None => Some((Value::Value(v), v + num_presses_so_far)),
                    _ => Some((result, limit))
                }
                Value::None => Some((result, limit)),
            }
        })
        .and_then(|(value, _)| match value {
            Value::Value(v) => Some(v),
            _ => None
        });

    let result = if let Some(child_result) = child_result {
        Value::Value(min_joltage + child_result)
    } else {
        Value::None
    };

    memo.insert(key, result);

    result

}

#[deprecated]
fn combinations(target: usize, num_buttons: usize) -> impl Iterator<Item = Vec<usize>> {
    (0..num_buttons).into_iter().combinations_with_replacement(target)
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
