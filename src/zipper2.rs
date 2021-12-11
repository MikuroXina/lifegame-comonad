use crate::zipper::{Zipper, ZipperIter};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Zipper2<T>(Zipper<Zipper<T>>);

impl<I, T> FromIterator<I> for Zipper2<T>
where
    I: IntoIterator<Item = T>,
    T: Default,
{
    fn from_iter<I2: IntoIterator<Item = I>>(iter: I2) -> Self {
        Self(Zipper::from_iter(iter.into_iter().map(Zipper::from_iter)))
    }
}

impl<T> Zipper2<T> {
    pub fn map<U>(self, f: impl Fn(T) -> U) -> Zipper2<U> {
        Zipper2(self.0.map(move |inner| inner.map(&f)))
    }

    #[allow(clippy::type_complexity)]
    pub fn top(
        &self,
    ) -> (
        (Option<&T>, Option<&T>, Option<&T>),
        (Option<&T>, Option<&T>, Option<&T>),
        (Option<&T>, Option<&T>, Option<&T>),
    ) {
        fn unzip<'a, T>(
            opt: Option<(Option<&'a T>, Option<&'a T>, Option<&'a T>)>,
        ) -> (Option<&'a T>, Option<&'a T>, Option<&'a T>) {
            if let Some((l, c, r)) = opt {
                (l, c, r)
            } else {
                (None, None, None)
            }
        }
        let (left, current, right) = self.0.top();
        let left = unzip(left.map(Zipper::top));
        let current = unzip(current.map(Zipper::top));
        let right = unzip(right.map(Zipper::top));
        (left, current, right)
    }

    pub fn iter(&self) -> ZipperIter<Zipper<T>> {
        self.0.iter()
    }
}

impl<T: Clone> Zipper2<T> {
    pub fn new(width: usize, height: usize, init: T) -> Self {
        Self(Zipper::new(height / 2, Zipper::new(width / 2, init)))
    }

    pub fn extract(&self) -> T {
        self.0.extract().extract()
    }

    pub fn duplicate(&self) -> Zipper2<Self> {
        Zipper2(
            self.clone()
                .0
                .map(|inner| Self(inner.duplicate()))
                .duplicate(),
        )
    }

    pub fn extend<U>(&self, f: impl Fn(Self) -> U) -> Zipper2<U> {
        self.duplicate().map(f)
    }
}
