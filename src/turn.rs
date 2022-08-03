use crate::State;

#[derive(Default)]
pub struct Turn {
    player: usize,
}

impl Turn {
    pub fn player(&self) -> usize {
        self.player
    }
}

impl State for Turn {
    type Step = ();

    fn step(&mut self, _: &Self::Step) {
        self.player = (self.player + 1) % 2;
    }
}
