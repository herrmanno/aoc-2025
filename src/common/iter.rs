struct SkipNthIterator<T> {
    i: usize,
    n: usize,
    it: T,
}

impl<T, U: Iterator<Item = T>> Iterator for SkipNthIterator<U> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.n {
            self.it.next();
            self.i += 1;
        }
        self.i += 1;
        self.it.next()
    }
}

pub trait SkipNth {
    type Item;

    fn skip_nth(self, n: usize) -> impl Iterator<Item = Self::Item>;
}

impl<T, I: Iterator<Item = T>> SkipNth for I {
    type Item = I::Item;

    ///
    /// # Examples
    ///
    /// **Skip element from iterator**
    /// ```
    /// # use crate::aoc2025::common::iter::SkipNth;
    /// let v = [0, 1, 2, 3];
    /// let mut iter = v.iter().skip_nth(2);
    /// assert_eq!(Some(&0), iter.next());
    /// assert_eq!(Some(&1), iter.next());
    /// assert_eq!(Some(&3), iter.next());
    /// assert_eq!(None, iter.next());
    /// ```
    ///
    /// **Skip element past iterator (= skip none)**
    /// ```
    /// # use crate::aoc2025::common::iter::SkipNth;
    /// let v = [0, 1, 2];
    /// let mut iter = v.iter().skip_nth(3);
    /// assert_eq!(Some(&0), iter.next());
    /// assert_eq!(Some(&1), iter.next());
    /// assert_eq!(Some(&2), iter.next());
    /// assert_eq!(None, iter.next());
    /// ```
    fn skip_nth(self, n: usize) -> impl Iterator<Item = Self::Item> {
        SkipNthIterator { i: 0, n, it: self }
    }
}
