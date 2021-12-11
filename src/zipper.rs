#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Zipper<T> {
    left: Vec<T>,
    current: T,
    right: Vec<T>,
}

impl<T: Default> FromIterator<T> for Zipper<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            left: vec![],
            current: T::default(),
            right: Vec::from_iter(iter),
        }
    }
}

impl<T> Zipper<T> {
    pub fn map<U>(self, f: impl Fn(T) -> U) -> Zipper<U> {
        let current = f(self.current);
        Zipper {
            left: self.left.into_iter().map(&f).collect(),
            current,
            right: self.right.into_iter().map(f).collect(),
        }
    }

    pub fn top(&self) -> (Option<&T>, &T, Option<&T>) {
        (self.left.last(), &self.current, self.right.last())
    }
}

impl<T: Clone> Zipper<T> {
    pub fn new(size: usize, init: T) -> Self {
        Self {
            left: vec![init.clone(); size],
            current: init.clone(),
            right: vec![init; size],
        }
    }

    pub fn left(&self) -> Option<Self> {
        let mut cloned = self.clone();
        let current = cloned.left.pop()?;
        let right = std::mem::replace(&mut cloned.current, current);
        cloned.right.push(right);
        Some(cloned)
    }

    pub fn right(&self) -> Option<Self> {
        let mut cloned = self.clone();
        let current = cloned.right.pop()?;
        let left = std::mem::replace(&mut cloned.current, current);
        cloned.left.push(left);
        Some(cloned)
    }

    pub fn extract(&self) -> T {
        self.current.clone()
    }

    pub fn duplicate(&self) -> Zipper<Self> {
        let left = std::iter::successors(Some(self.clone()), Zipper::left)
            .skip(1)
            .collect();
        let right = std::iter::successors(Some(self.clone()), Zipper::right)
            .skip(1)
            .collect();
        Zipper {
            left,
            current: self.clone(),
            right,
        }
    }

    pub fn extend<U>(&self, f: impl Fn(Self) -> U) -> Zipper<U> {
        self.duplicate().map(f)
    }
}
