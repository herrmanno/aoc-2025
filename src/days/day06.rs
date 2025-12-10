//! # Day 06 Trash Compactor

use std::vec;

use aoc_runner::Day;

type I = u64;
type Matrix = Vec<Vec<I>>;
type Ops = Vec<Op>;

#[derive(Debug, Clone)]
enum Op {
    Add,
    Mul
}

#[derive(Default, Clone)]
pub struct Day06 {
    input: String,
}

impl Day06 {
    fn parse_matrix(&mut self) -> (Matrix, Ops) {
        let lines = self.input.lines().collect::<Vec<_>>();
        let nums = &lines[0..lines.len() - 1];
        let ops = &lines[lines.len() - 1];

        let matrix = nums.iter().map(|line| {
            line.split_whitespace().map(|n| n.parse().unwrap()).collect()
        })
        .collect();
        let ops = ops.split_whitespace().map(|op| match op {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => unreachable!()
        })
        .collect();

        (matrix, ops)
    }

    fn parse_matrix_transposed(&mut self) -> (Matrix, Ops) {
        let lines = self.input.lines().collect::<Vec<_>>();
        let nums = &lines[0..lines.len() - 1];
        let ops = &lines[lines.len() - 1];

        let matrix = {
            let width = nums.iter().map(|it| it.len()).max().unwrap();
            let mut result = vec![vec![]];
            for col in (0..width).rev() {
                let mut all_whitespace = true;
                let mut tmp: I = 0;

                for row in 0..nums.len() {
                    let c = &nums[row].chars().nth(col).unwrap_or(' ');
                    if !c.is_whitespace() {
                        all_whitespace = false;
                        tmp = 10 * tmp + c.to_digit(10).unwrap() as I;
                    }
                }

                if all_whitespace {
                    result.push(vec![]);
                } else {
                    result.last_mut().unwrap().push(tmp);
                }

            }
            result
        };

        let ops = ops.split_whitespace().map(|op| match op {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => unreachable!()
        })
        .rev()
        .collect();

        (matrix, ops)
    }

    fn sum_matrix(matrix: Matrix, ops: Ops) -> I {
        let len = matrix[0].len();
        let mut results = matrix[0].clone();
        for col in 0..len {
            for row in 1..matrix.len() {
                match ops[col] {
                    Op::Add => {
                        results[col] += matrix[row][col];
                    }
                    Op::Mul => {
                        results[col] *= matrix[row][col];
                    }
                }
            }
        }

        results.into_iter().sum()
    }

    fn sum_matrix_transposed(matrix: Matrix, ops: Ops) -> I {
        matrix.into_iter().enumerate()
            .map(|(idx, nums)| {
                match ops[idx] {
                  Op::Add => nums.into_iter().sum::<I>(),
                  Op::Mul => nums.into_iter().product::<I>(),
                }
            })
            .sum()
    }

}

impl Day for Day06 {
    type Result1 = I;
    type Result2 = I;

    fn parse(&mut self, input: &str) {
        self.input = input.to_string();
    }

    fn part1(&mut self) -> Self::Result1 {
        let (matrix, ops) = self.parse_matrix();
        Self::sum_matrix(matrix, ops)
    }

    fn part2(&mut self) -> Self::Result2 {
        let (matrix, ops) = self.parse_matrix_transposed();
        Self::sum_matrix_transposed(matrix, ops)
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::*;

    const INPUT: &str = indoc!{"
        123 328  51 64
         45 64  387 23
          6 98  215 314
        *   +   *   +
    "};

    #[test]
    fn part_1() {
        let mut day = Day06::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 4277556);
    }

    #[test]
    fn part_2() {
        let mut day = Day06::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 3263827);
    }
}
