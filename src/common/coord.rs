use std::{ops::Add, process::Output};

use num::Integer;

trait Coord<const N: u8, T>: Sized
where
    T: Sized + Copy,
{
    fn from(components: [T; N]) -> Self;
    fn components(&self) -> [T; N];
}

struct Foo {}

impl<T: Foo> Add for T {
    type Output = T;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from([0, 0])
    }
}
