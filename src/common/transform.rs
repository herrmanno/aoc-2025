/// A value that can be transformed
#[allow(unused)]
pub(crate) trait Transform: Sized {
    /// Transform `self` by passing it to `f`
    fn transform<T, F: Fn(Self) -> T>(self, f: F) -> T;
}

impl<U> Transform for U {
    fn transform<T, F: Fn(Self) -> T>(self, f: F) -> T {
        f(self)
    }
}
