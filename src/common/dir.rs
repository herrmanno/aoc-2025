use num::{Integer, One, Zero};
use std::ops::Neg;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Dir {
    #[default]
    N,
    S,
    W,
    E,
}

impl Dir {
    pub const ALL: [Self; 4] = [Self::N, Self::E, Self::S, Self::W];

    pub fn go<N: Integer + Neg<Output = N>>(&self, (y, x): (N, N)) -> (N, N) {
        let dir: (N, N) = <Dir as Into<(N, N)>>::into(*self) as (N, N);
        (y + dir.0, x + dir.1)
    }

    pub fn go_n<N: Integer + Neg<Output = N> + Copy>(&self, (y, x): (N, N), n: N) -> (N, N) {
        let dir: (N, N) = <Dir as Into<(N, N)>>::into(*self) as (N, N);
        (y + dir.0 * n, x + dir.1 * n)
    }

    pub fn turn_left(&self) -> Self {
        match self {
            Dir::N => Dir::W,
            Dir::S => Dir::E,
            Dir::W => Dir::S,
            Dir::E => Dir::N,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Dir::N => Dir::E,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
            Dir::E => Dir::S,
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::W => Dir::E,
            Dir::E => Dir::W,
        }
    }
}

impl<T: Integer + Neg<Output = T>> From<Dir> for (T, T) {
    fn from(dir: Dir) -> (T, T) {
        match dir {
            Dir::N => (Neg::neg(<T as One>::one()), Zero::zero()),
            Dir::S => (One::one(), Zero::zero()),
            Dir::W => (Zero::zero(), Neg::neg(<T as One>::one())),
            Dir::E => (Zero::zero(), One::one()),
        }
    }
}

impl TryFrom<char> for Dir {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'N' | 'U' | '^' => Ok(Dir::N),
            'S' | 'D' | 'v' => Ok(Dir::S),
            'W' | 'L' | '<' => Ok(Dir::W),
            'E' | 'R' | '>' => Ok(Dir::E),
            _ => Err(value),
        }
    }
}

impl<'a> TryFrom<&'a str> for Dir {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let char = value.chars().next().ok_or("")?;
        let result = Dir::try_from(char);
        result.map_err(|_| value)
    }
}
