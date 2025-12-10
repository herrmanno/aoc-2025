//! # Day 08 Playground

use aoc_runner::Day;
use itertools::Itertools;

/// Coord Element
type I = u64;
/// Coord
type C = [I; 3];
/// Distanse
type D = I;

/// Maximal pair distance to consider
///
/// The value is choosen experimentally and might be adjusted upwards based on the used input!
const CUT_OFF: D = 500_000_000;

#[derive(Default, Clone)]
pub struct Day08<const N: usize = 1000> {
    coords: Vec<C>,
    tuples_by_distance: Vec<(D, usize, usize)>,
}

impl <const N: usize> Day for Day08<N> {
    type Result1 = usize;
    type Result2 = u64;

    fn parse(&mut self, input: &str) {
        self.coords = input.lines()
            .map(|line| {
                let mut nums = line.split(",").map(|it| it.parse::<I>().unwrap());
                let x = nums.next().unwrap();
                let y = nums.next().unwrap();
                let z = nums.next().unwrap();
                [x, y, z]
            })
            .collect();

        self.tuples_by_distance = {
            let len = self.coords.len();
            let mut distances: Vec<(D, usize, usize)> = Vec::with_capacity(len * len / 2);
            for i in 0..len {
                for j in (i+1)..len {
                    let [x1, y1, z1] = self.coords[i];
                    let [x2, y2, z2] = self.coords[j];
                    let d = x1.abs_diff(x2).pow(2) + y1.abs_diff(y2).pow(2) + z1.abs_diff(z2).pow(2);
                    if d < CUT_OFF {
                        distances.push((d, i, j));
                    }
                }
            }

            distances.sort_unstable_by_key(|it| it.0);
            distances
        };
    }

    fn part1(&mut self) -> Self::Result1 {
        let len = self.coords.len();
        let mut group_counter = 0;
        let mut node_to_group_mapping = vec![None as Option<usize>; len];
        let mut group_to_nodes_mapping = vec![vec![0usize; 0]; N];

        for &(_, i, j) in self.tuples_by_distance.iter().take(N) {
            let i_group = node_to_group_mapping[i];
            let j_group = node_to_group_mapping[j];

            match (i_group, j_group) {
                (None, None) => {
                    node_to_group_mapping[i] = Some(group_counter);
                    node_to_group_mapping[j] = Some(group_counter);
                    group_to_nodes_mapping[group_counter].extend_from_slice(&[i, j]);
                    group_counter += 1;
                }
                (None, Some(g)) => {
                    node_to_group_mapping[i] = Some(g);
                    group_to_nodes_mapping[g].push(i);
                }
                (Some(g), None) => {
                    node_to_group_mapping[j] = Some(g);
                    group_to_nodes_mapping[g].push(j);
                }
                (Some(g1), Some(g2)) => {
                    if g1 != g2 {
                        node_to_group_mapping[j] = Some(g1);
                        let mut nodes_g2 = vec![];
                        std::mem::swap(&mut group_to_nodes_mapping[g2], &mut nodes_g2);
                        for n in nodes_g2.iter() {
                            node_to_group_mapping[*n] = Some(g1);
                            group_to_nodes_mapping[g1].push(*n);
                        }
                    }
                }
            }
        }

        let group_sizes = group_to_nodes_mapping.into_iter().map(|it| it.len()).sorted().rev().collect::<Vec<_>>();
        group_sizes.into_iter().take(3).product()
    }

    fn part2(&mut self) -> Self::Result2 {
        let len = self.coords.len();
        let mut group_counter = 0;
        let mut node_to_group_mapping = vec![None as Option<usize>; len];
        let mut group_to_nodes_mapping = vec![vec![0usize; 0]; N];

        for &(_, i, j) in self.tuples_by_distance.iter() {
            let i_group = node_to_group_mapping[i];
            let j_group = node_to_group_mapping[j];

            let group_idx = match (i_group, j_group) {
                (None, None) => {
                    node_to_group_mapping[i] = Some(group_counter);
                    node_to_group_mapping[j] = Some(group_counter);
                    group_to_nodes_mapping[group_counter].extend_from_slice(&[i, j]);
                    group_counter += 1;
                    group_counter - 1
                }
                (None, Some(g)) => {
                    node_to_group_mapping[i] = Some(g);
                    group_to_nodes_mapping[g].push(i);
                    g
                }
                (Some(g), None) => {
                    node_to_group_mapping[j] = Some(g);
                    group_to_nodes_mapping[g].push(j);
                    g
                }
                (Some(g1), Some(g2)) => {
                    if g1 != g2 {
                        node_to_group_mapping[j] = Some(g1);
                        let nodes_g2 = group_to_nodes_mapping[g2].clone();
                        for n in nodes_g2.iter() {
                            node_to_group_mapping[*n] = Some(g1);
                        }
                        group_to_nodes_mapping[g1].extend(nodes_g2);
                        group_to_nodes_mapping[g2].clear();
                    }
                    g1
                }
            };

            let group = &group_to_nodes_mapping[group_idx];
            if group.len() == len {
                return self.coords[i][0] * self.coords[j][0];
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::*;

    const INPUT: &str = indoc!{"
        162,817,812
        57,618,57
        906,360,560
        592,479,940
        352,342,300
        466,668,158
        542,29,236
        431,825,988
        739,650,466
        52,470,668
        216,146,977
        819,987,18
        117,168,530
        805,96,715
        346,949,466
        970,615,88
        941,993,340
        862,61,35
        984,92,344
        425,690,689
    "};

    #[test]
    fn part_1() {
        let mut day = Day08::<10>::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 40);
    }

    #[test]
    fn part_2() {
        let mut day = Day08::<10>::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 25272);
    }
}
