mod zipper;
mod zipper2;

use zipper2::Zipper2;

#[derive(Debug, Default, Clone)]
pub struct LifeGame(Zipper2<bool>);

impl std::fmt::Display for LifeGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0.iter() {
            for ch in row.iter() {
                write!(f, "{}", if *ch { '■' } else { ' ' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl LifeGame {
    pub fn new() -> Self {
        Self(Zipper2::new(20, 20, false))
    }

    pub fn with_pattern(pattern: Vec<Vec<bool>>) -> Self {
        Self(Zipper2::from_iter(pattern))
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
    let mut life_game = LifeGame::with_pattern(vec![
        vec![false, true, false, false, false, false, false],
        vec![false, false, true, false, false, false, false],
        vec![true, true, true, false, false, false, false],
        vec![false, false, false, false, false, false, false],
        vec![false, false, false, false, false, false, false],
        vec![false, false, false, false, false, false, false],
        vec![false, false, false, false, false, false, false],
    ]);
    for _ in 0..20 {
        println!("{}", life_game);
        life_game = life_game.next_step();
    }
}
