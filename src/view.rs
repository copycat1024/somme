use crate::{Action, Unit};

pub struct GameView<'a> {
    units: &'a [Unit],
    log: &'a [Action],
}

impl<'a> GameView<'a> {
    pub fn new(units: &'a [Unit], log: &'a [Action]) -> Self {
        Self { units, log }
    }

    pub fn units(&self) -> &'a [Unit] {
        self.units
    }

    pub fn log(&self) -> &'a [Action] {
        self.log
    }
}
