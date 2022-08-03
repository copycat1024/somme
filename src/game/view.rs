use crate::Unit;

pub struct GameView<'a> {
    units: &'a [Unit],
}

impl<'a> GameView<'a> {
    pub fn new(units: &'a [Unit]) -> Self {
        Self { units }
    }

    pub fn units(&self) -> &'a [Unit] {
        self.units
    }
}
