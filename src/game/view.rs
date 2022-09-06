use hecs::World;

pub struct GameView<'a> {
    ecs: &'a World,
}

impl<'a> GameView<'a> {
    pub fn new(ecs: &'a World) -> Self {
        Self { ecs }
    }

    pub fn ecs(&self) -> &'a World {
        self.ecs
    }
}
