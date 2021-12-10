mod zipper;
mod zipper2;

use zipper2::Zipper2;

#[derive(Debug, Default, Clone)]
pub struct LifeGame(Zipper2<bool>);

impl LifeGame {
    pub fn new() -> Self {
        Self(Zipper2::new(20, 20, false))
    }

    pub fn count_neighbors(z: &Zipper2<bool>) -> usize {
        let ((top_left, top, top_right), (left, _, right), (bottom_left, bottom, bottom_right)) =
            z.top();
        top_left
            .into_iter()
            .chain(top)
            .chain(top_right)
            .chain(left)
            .chain(right)
            .chain(bottom_left)
            .chain(bottom)
            .chain(bottom_right)
            .filter(|&&b: &&bool| b)
            .count()
    }

    pub fn has_life(z: Zipper2<bool>) -> bool {
        matches!(
            (z.extract(), Self::count_neighbors(&z)),
            (true, 2 | 3) | (false, 3)
        )
    }

    pub fn next_step(&self) -> Self {
        Self(self.0.extend(Self::has_life))
    }
}

fn main() {
    println!("Hello, world!");
}
