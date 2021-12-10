use crate::zipper::Zipper;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Zipper2<T>(Zipper<Zipper<T>>);

impl<T> Zipper2<T> {
    pub fn map<U>(self, f: impl Fn(T) -> U) -> Zipper2<U> {
        Zipper2(self.0.map(move |inner| inner.map(&f)))
    }

    pub fn top(
        &self,
    ) -> (
        (Option<&T>, Option<&T>, Option<&T>),
        (Option<&T>, &T, Option<&T>),
        (Option<&T>, Option<&T>, Option<&T>),
    ) {
        let (left, current, right) = self.0.top();
        let left = if let Some((l, c, r)) = left.map(Zipper::top) {
            (l, Some(c), r)
        } else {
            (None, None, None)
        };
        let right = if let Some((l, c, r)) = right.map(Zipper::top) {
            (l, Some(c), r)
        } else {
            (None, None, None)
        };
        (left, current.top(), right)
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