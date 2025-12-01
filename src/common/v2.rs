use std::ops::Add;

use num::{Integer, One};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct V2<T>(pub T, pub T);

impl<T> V2<T>
where
    T: Integer + Add<Output = T> + Clone,
{
    pub fn neighbours(&self) -> [Self; 4] {
        [
            V2(self.0.clone().add(One::one()), self.1.clone()),
            V2(self.0.clone().sub(One::one()), self.1.clone()),
            V2(self.0.clone(), self.1.clone().add(One::one())),
            V2(self.0.clone(), self.1.clone().sub(One::one())),
        ]
    }
}

impl<T> std::ops::Add for V2<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = V2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        V2(self.0 + rhs.0, self.1 + rhs.1)
    }
}
