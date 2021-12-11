#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Zipper<T> {
    left: Vec<T>,
    current: Option<T>,
    right: Vec<T>,
}

impl<T> FromIterator<T> for Zipper<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        Self {
            left: vec![],
            current: iter.next(),
            right: Vec::from_iter(iter),
        }
    }
}

impl<T> Zipper<T> {
    pub fn map<U>(self, f: impl Fn(T) -> U) -> Zipper<U> {
        Zipper {
            left: self.left.into_iter().map(&f).collect(),
            current: self.current.map(&f),
            right: self.right.into_iter().map(f).collect(),
        }
    }

    pub fn top(&self) -> (Option<&T>, Option<&T>, Option<&T>) {
        (self.left.last(), self.current.as_ref(), self.right.last())
    }

    pub fn iter(&self) -> ZipperIter<T> {
        ZipperIter {
            left: self.left.iter(),
            current: self.current.as_ref(),
            right: self.right.iter(),
        }
    }
}

impl<T: Clone> Zipper<T> {
    pub fn new(size: usize, init: T) -> Self {
        if size == 0 {
            Self {
                left: vec![],
                current: None,
                right: vec![],
            }
        } else {
            Self {
                left: vec![],
                current: Some(init.clone()),
                right: vec![init; size - 1],
            }
        }
    }

    pub fn left(&self) -> Option<Self> {
        let mut cloned = self.clone();
        let current = cloned.left.pop();
        let right = std::mem::replace(&mut cloned.current, current)?;
        cloned.right.push(right);
        Some(cloned)
    }

    pub fn right(&self) -> Option<Self> {
        let mut cloned = self.clone();
        let current = cloned.right.pop();
        let left = std::mem::replace(&mut cloned.current, current)?;
        cloned.left.push(left);
        Some(cloned)
    }

    pub fn extract(&self) -> T {
        self.current.as_ref().cloned().unwrap()
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
            current: Some(self.clone()),
            right,
        }
    }

    pub fn extend<U>(&self, f: impl Fn(Self) -> U) -> Zipper<U> {
        self.duplicate().map(f)
    }
}

#[derive(Debug)]
pub struct ZipperIter<'z, T> {
    left: std::slice::Iter<'z, T>,
    current: Option<&'z T>,
    right: std::slice::Iter<'z, T>,
}

impl<'z, T> Iterator for ZipperIter<'z, T> {
    type Item = &'z T;

    fn next(&mut self) -> Option<Self::Item> {
        self.left
            .next()
            .or_else(|| self.current.take())
            .or_else(|| self.right.next_back())
    }
}
